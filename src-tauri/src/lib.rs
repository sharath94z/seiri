use chrono::{SecondsFormat, Utc};
use fs2::FileExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

const M3_PDF_RULE_ID: &str = "move-pdfs";
const M3_PDF_SOURCE_FOLDER: &str = "~/Downloads";
const M3_PDF_DESTINATION_FOLDER: &str = "~/Documents/PDFs/";
const M3_UNDO_WINDOW_HOURS: i64 = 24;
const MAX_CONFLICT_RENAME_ATTEMPTS: usize = 1000;

#[derive(Debug, Clone)]
struct StorageFiles {
    base_dir: PathBuf,
    rules: PathBuf,
    settings: PathBuf,
    activity: PathBuf,
    retry_queue: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RuleCondition {
    #[serde(rename = "type")]
    kind: String,
    value: String,
    negate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RuleConditionGroup {
    #[serde(rename = "conditionLogic")]
    condition_logic: String,
    conditions: Vec<RuleCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum RuleConditionNode {
    Condition(RuleCondition),
    Group(RuleConditionGroup),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RuleActionDefinition {
    #[serde(rename = "type")]
    kind: String,
    value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RuleDefinition {
    id: String,
    name: String,
    enabled: bool,
    priority: i64,
    #[serde(rename = "conditionLogic")]
    condition_logic: String,
    conditions: Vec<RuleConditionNode>,
    actions: Vec<RuleActionDefinition>,
    #[serde(rename = "isPrebuilt")]
    is_prebuilt: bool,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct LicenseState {
    tier: String,
    key: Option<String>,
    #[serde(rename = "validatedAt")]
    validated_at: Option<String>,
    #[serde(rename = "expiresAt")]
    expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct SettingsState {
    #[serde(rename = "onboardingCompleted")]
    onboarding_completed: bool,
    #[serde(rename = "launchAtLogin")]
    launch_at_login: bool,
    #[serde(rename = "dailySummaryEnabled")]
    daily_summary_enabled: bool,
    #[serde(rename = "dailySummaryStartedAt")]
    daily_summary_started_at: Option<String>,
    #[serde(rename = "dailySummaryLastSentAt")]
    daily_summary_last_sent_at: Option<String>,
    #[serde(rename = "watchedFolders")]
    watched_folders: Vec<String>,
    #[serde(rename = "fileStabilityWaitSeconds")]
    file_stability_wait_seconds: u64,
    #[serde(rename = "conflictBehavior")]
    conflict_behavior: String,
    #[serde(rename = "activityRetentionDays")]
    activity_retention_days: u64,
    license: LicenseState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ActivityEntry {
    id: String,
    timestamp: String,
    status: String,
    filename: String,
    #[serde(rename = "sourcePath")]
    source_path: String,
    #[serde(rename = "destinationPath")]
    destination_path: Option<String>,
    #[serde(rename = "finalPath")]
    final_path: Option<String>,
    #[serde(rename = "ruleId")]
    rule_id: Option<String>,
    #[serde(rename = "ruleName")]
    rule_name: Option<String>,
    #[serde(rename = "actionType")]
    action_type: Option<String>,
    message: Option<String>,
    #[serde(rename = "undoEligibleUntil")]
    undo_eligible_until: Option<String>,
    #[serde(rename = "undoneAt")]
    undone_at: Option<String>,
    #[serde(rename = "undoSourceEntryId")]
    undo_source_entry_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct RetryQueueEntry {
    id: String,
    #[serde(rename = "sourcePath")]
    source_path: String,
    filename: String,
    #[serde(rename = "firstQueuedAt")]
    first_queued_at: String,
    #[serde(rename = "lastCheckedAt")]
    last_checked_at: String,
    #[serde(rename = "lastFailureReason")]
    last_failure_reason: String,
    #[serde(rename = "attemptCount")]
    attempt_count: u64,
}

#[derive(Debug, Clone, Serialize)]
struct M3RunResult {
    status: String,
    message: String,
    #[serde(rename = "activityEntry")]
    activity_entry: Option<ActivityEntry>,
    entries: Vec<ActivityEntry>,
}

#[derive(Debug, Clone, Serialize)]
struct UndoActivityResult {
    status: String,
    message: String,
    #[serde(rename = "activityEntry")]
    activity_entry: Option<ActivityEntry>,
    entries: Vec<ActivityEntry>,
}

#[derive(Debug, Clone, Serialize)]
struct StorageStatus {
    #[serde(rename = "rulesExists")]
    rules_exists: bool,
    #[serde(rename = "settingsExists")]
    settings_exists: bool,
    #[serde(rename = "activityExists")]
    activity_exists: bool,
    #[serde(rename = "retryQueueExists")]
    retry_queue_exists: bool,
}

#[tauri::command]
fn storage_status(app: AppHandle) -> Result<StorageStatus, String> {
    let files = app_storage_files(&app)?;
    ensure_storage_dir(&files.base_dir)?;

    Ok(StorageStatus {
        rules_exists: files.rules.exists(),
        settings_exists: files.settings.exists(),
        activity_exists: files.activity.exists(),
        retry_queue_exists: files.retry_queue.exists(),
    })
}

#[tauri::command]
fn load_rules(app: AppHandle) -> Result<Vec<RuleDefinition>, String> {
    let files = app_storage_files(&app)?;
    let rules = load_persisted_or_initialize(&files.rules, default_rules())?;
    validate_rules(&rules)?;
    Ok(rules)
}

#[tauri::command]
fn save_rules(app: AppHandle, rules: Vec<RuleDefinition>) -> Result<(), String> {
    validate_rules(&rules)?;
    let files = app_storage_files(&app)?;
    write_json_atomic(&files.rules, &rules)
}

#[tauri::command]
fn load_settings(app: AppHandle) -> Result<SettingsState, String> {
    let files = app_storage_files(&app)?;
    let settings = load_persisted_or_initialize(&files.settings, default_settings())?;
    validate_settings(&settings)?;
    Ok(settings)
}

#[tauri::command]
fn save_settings(app: AppHandle, settings: SettingsState) -> Result<(), String> {
    validate_settings(&settings)?;
    let files = app_storage_files(&app)?;
    write_json_atomic(&files.settings, &settings)
}

#[tauri::command]
fn load_activity(app: AppHandle) -> Result<Vec<ActivityEntry>, String> {
    let files = app_storage_files(&app)?;
    let entries = load_persisted_or_initialize(&files.activity, default_activity())?;
    validate_activity(&entries)?;
    Ok(entries)
}

#[tauri::command]
fn save_activity(app: AppHandle, entries: Vec<ActivityEntry>) -> Result<(), String> {
    validate_activity(&entries)?;
    let files = app_storage_files(&app)?;
    write_json_atomic(&files.activity, &entries)
}

#[tauri::command]
fn load_retry_queue(app: AppHandle) -> Result<Vec<RetryQueueEntry>, String> {
    let files = app_storage_files(&app)?;
    let entries = load_persisted_or_initialize(&files.retry_queue, default_retry_queue())?;
    validate_retry_queue(&entries)?;
    Ok(entries)
}

#[tauri::command]
fn save_retry_queue(app: AppHandle, entries: Vec<RetryQueueEntry>) -> Result<(), String> {
    validate_retry_queue(&entries)?;
    let files = app_storage_files(&app)?;
    write_json_atomic(&files.retry_queue, &entries)
}

#[tauri::command]
fn run_m3_pdf_slice(app: AppHandle) -> Result<M3RunResult, String> {
    let files = app_storage_files(&app)?;
    let home_dir = user_home_dir()?;
    run_m3_pdf_slice_with_context(&files, &home_dir)
}

#[tauri::command]
fn undo_activity_entry(app: AppHandle, activity_entry_id: String) -> Result<UndoActivityResult, String> {
    let files = app_storage_files(&app)?;
    let home_dir = user_home_dir()?;
    undo_activity_entry_with_context(&files, &home_dir, &activity_entry_id)
}

fn run_m3_pdf_slice_with_context(files: &StorageFiles, home_dir: &Path) -> Result<M3RunResult, String> {
    let rules = load_persisted_or_initialize(&files.rules, default_rules())?;
    validate_rules(&rules)?;

    let settings = load_persisted_or_initialize(&files.settings, default_settings())?;
    validate_settings(&settings)?;

    let mut entries = load_persisted_or_initialize(&files.activity, default_activity())?;
    validate_activity(&entries)?;

    let rule = rules
        .iter()
        .find(|rule| rule.id == M3_PDF_RULE_ID)
        .ok_or_else(|| format!("required M3 rule {M3_PDF_RULE_ID} is missing"))?;

    let pdf_candidate = find_first_eligible_pdf(&expand_user_path(M3_PDF_SOURCE_FOLDER, home_dir))?;
    let Some(source_path) = pdf_candidate else {
        return Ok(M3RunResult {
            status: "no_match".into(),
            message: "No eligible PDF was found in ~/Downloads".into(),
            activity_entry: None,
            entries,
        });
    };

    let source_display = display_user_path(&source_path, home_dir);
    let source_file_name = source_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("file name is invalid UTF-8: {}", source_path.display()))?
        .to_string();

    let action = rule
        .actions
        .first()
        .ok_or_else(|| format!("rule {} has no actions", rule.id))?;
    if action.kind != "move" {
        return Err(format!(
            "rule {} must use a move action for the M3 PDF slice",
            rule.id
        ));
    }

    let destination_dir = expand_user_path(
        action
            .value
            .as_deref()
            .unwrap_or(M3_PDF_DESTINATION_FOLDER),
        home_dir,
    );
    if let Err(error) = ensure_storage_dir(&destination_dir) {
        let destination_error = format!("Destination folder could not be prepared: {error}");
        let entry = build_m3_activity_entry(
            "failed",
            &source_file_name,
            &source_display,
            Some(display_user_path(&destination_dir, home_dir)),
            Some(source_display.clone()),
            Some(rule.id.clone()),
            Some(rule.name.clone()),
            Some("move"),
            Some(destination_error.as_str()),
            None,
            None,
            None,
        );
        entries = prepend_activity_entry(entries, entry.clone());
        validate_activity(&entries)?;
        write_json_atomic(&files.activity, &entries)?;
        return Ok(M3RunResult {
            status: "failed".into(),
            message: destination_error,
            activity_entry: Some(entry),
            entries,
        });
    }

    let safety_window = Duration::from_secs(settings.file_stability_wait_seconds);
    if !file_size_is_stable(&source_path, safety_window)? {
        let entry = build_m3_activity_entry(
            "skipped",
            &source_file_name,
            &source_display,
            Some(display_user_path(&destination_dir, home_dir)),
            Some(source_display.clone()),
            Some(rule.id.clone()),
            Some(rule.name.clone()),
            Some("move"),
            Some("Skipped - file was not stable long enough to process"),
            None,
            None,
            None,
        );
        entries = prepend_activity_entry(entries, entry.clone());
        validate_activity(&entries)?;
        write_json_atomic(&files.activity, &entries)?;
        return Ok(M3RunResult {
            status: "skipped".into(),
            message: "Skipped - file was not stable long enough to process".into(),
            activity_entry: Some(entry),
            entries,
        });
    }

    if !file_is_unlocked(&source_path)? {
        let entry = build_m3_activity_entry(
            "skipped",
            &source_file_name,
            &source_display,
            Some(display_user_path(&destination_dir, home_dir)),
            Some(source_display.clone()),
            Some(rule.id.clone()),
            Some(rule.name.clone()),
            Some("move"),
            Some("Skipped - file is locked by another process"),
            None,
            None,
            None,
        );
        entries = prepend_activity_entry(entries, entry.clone());
        validate_activity(&entries)?;
        write_json_atomic(&files.activity, &entries)?;
        return Ok(M3RunResult {
            status: "skipped".into(),
            message: "Skipped - file is locked by another process".into(),
            activity_entry: Some(entry),
            entries,
        });
    }

    if !source_path.exists() {
        let entry = build_m3_activity_entry(
            "failed",
            &source_file_name,
            &source_display,
            Some(display_user_path(&destination_dir, home_dir)),
            Some(source_display.clone()),
            Some(rule.id.clone()),
            Some(rule.name.clone()),
            Some("move"),
            Some("Failed - file no longer exists at source"),
            None,
            None,
            None,
        );
        entries = prepend_activity_entry(entries, entry.clone());
        validate_activity(&entries)?;
        write_json_atomic(&files.activity, &entries)?;
        return Ok(M3RunResult {
            status: "failed".into(),
            message: "Failed - file no longer exists at source".into(),
            activity_entry: Some(entry),
            entries,
        });
    }

    let source_exists_at_destination = destination_dir.join(&source_file_name);
    let destination_exists = source_exists_at_destination.exists();
    let conflict_behavior = settings.conflict_behavior.as_str();

    if destination_exists && conflict_behavior == "skip" {
        let entry = build_m3_activity_entry(
            "skipped",
            &source_file_name,
            &source_display,
            Some(display_user_path(&destination_dir, home_dir)),
            Some(source_display.clone()),
            Some(rule.id.clone()),
            Some(rule.name.clone()),
            Some("move"),
            Some("Skipped - file already exists at destination"),
            None,
            None,
            None,
        );
        entries = prepend_activity_entry(entries, entry.clone());
        validate_activity(&entries)?;
        write_json_atomic(&files.activity, &entries)?;
        return Ok(M3RunResult {
            status: "skipped".into(),
            message: "Skipped - file already exists at destination".into(),
            activity_entry: Some(entry),
            entries,
        });
    }

    let final_path = if destination_exists {
        conflict_renamed_path(&destination_dir, &source_file_name)?
    } else {
        source_exists_at_destination
    };

    move_file_with_cross_device_fallback(&source_path, &final_path)?;

    let final_display = display_user_path(&final_path, home_dir);
    let destination_display = display_user_path(&destination_dir, home_dir);
    let undo_eligible_until = iso_timestamp_after_hours(M3_UNDO_WINDOW_HOURS);
    let entry = build_m3_activity_entry(
        "success",
        &source_file_name,
        &source_display,
        Some(destination_display),
        Some(final_display.clone()),
        Some(rule.id.clone()),
        Some(rule.name.clone()),
        Some("move"),
        Some("Moved successfully"),
        Some(undo_eligible_until),
        None,
        None,
    );

    entries = prepend_activity_entry(entries, entry.clone());
    validate_activity(&entries)?;
    write_json_atomic(&files.activity, &entries)?;

    Ok(M3RunResult {
        status: "moved".into(),
        message: "Moved successfully".into(),
        activity_entry: Some(entry),
        entries,
    })
}

fn undo_activity_entry_with_context(
    files: &StorageFiles,
    home_dir: &Path,
    activity_entry_id: &str,
) -> Result<UndoActivityResult, String> {
    let mut entries = load_persisted_or_initialize(&files.activity, default_activity())?;
    validate_activity(&entries)?;

    let target_index = entries
        .iter()
        .position(|entry| entry.id == activity_entry_id)
        .ok_or_else(|| format!("activity entry {activity_entry_id} was not found"))?;
    let target = entries[target_index].clone();

    if target.status != "success" {
        return Err(format!(
            "activity entry {} is not eligible for undo",
            target.id
        ));
    }

    if target.undone_at.is_some() {
        return Err(format!("activity entry {} has already been undone", target.id));
    }

    if target.action_type.as_deref() != Some("move") {
        return Err(format!(
            "activity entry {} is not a reversible move",
            target.id
        ));
    }

    let undo_eligible_until = target
        .undo_eligible_until
        .as_deref()
        .ok_or_else(|| format!("activity entry {} is missing undo eligibility", target.id))?;
    let undo_deadline = chrono::DateTime::parse_from_rfc3339(undo_eligible_until)
        .map_err(|error| format!("activity entry {} has an invalid undo deadline: {error}", target.id))?
        .with_timezone(&Utc);
    if Utc::now() > undo_deadline {
        return Err("Undo window expired".into());
    }

    let final_path = target
        .final_path
        .as_deref()
        .ok_or_else(|| format!("activity entry {} is missing its finalPath", target.id))?;
    let source_path = expand_user_path(&target.source_path, home_dir);
    let final_path = expand_user_path(final_path, home_dir);

    if !final_path.exists() {
        return Err("Cannot undo - the file no longer exists at its organised location.".into());
    }

    if let Some(parent) = source_path.parent() {
        if !parent.exists() {
            return Err("Cannot undo - the original folder no longer exists.".into());
        }
    }

    if source_path.exists() {
        return Err("Cannot undo - the original path is already occupied.".into());
    }

    fs::rename(&final_path, &source_path).map_err(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            return "Cannot undo - the original path is already occupied.".into();
        }

        format!(
            "failed to move {} back to {}: {error}",
            final_path.display(),
            source_path.display()
        )
    })?;

    let now = iso_timestamp_now();
    entries[target_index].undone_at = Some(now.clone());
    let undo_entry = ActivityEntry {
        id: generate_entry_id("undo"),
        timestamp: now,
        status: "undone".into(),
        filename: target.filename.clone(),
        source_path: target
            .final_path
            .clone()
            .unwrap_or_else(|| target.source_path.clone()),
        destination_path: Some(target.source_path.clone()),
        final_path: Some(target.source_path.clone()),
        rule_id: target.rule_id.clone(),
        rule_name: target.rule_name.clone(),
        action_type: target.action_type.clone(),
        message: Some("Undid move".into()),
        undo_eligible_until: None,
        undone_at: None,
        undo_source_entry_id: Some(target.id.clone()),
    };

    entries = prepend_activity_entry(entries, undo_entry.clone());
    validate_activity(&entries)?;
    write_json_atomic(&files.activity, &entries)?;

    Ok(UndoActivityResult {
        status: "undone".into(),
        message: "Restored file to its original location".into(),
        activity_entry: Some(undo_entry),
        entries,
    })
}

fn find_first_eligible_pdf(downloads_dir: &Path) -> Result<Option<PathBuf>, String> {
    if !downloads_dir.exists() {
        return Ok(None);
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(downloads_dir).map_err(|error| {
        format!(
            "failed to read watched folder {}: {error}",
            downloads_dir.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!(
                "failed to inspect watched folder {}: {error}",
                downloads_dir.display()
            )
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "failed to inspect file type for {}: {error}",
                path.display()
            )
        })?;

        if file_type.is_file() {
            entries.push(path);
        }
    }

    entries.sort_by(|left, right| {
        left.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .cmp(right.file_name().and_then(|name| name.to_str()).unwrap_or_default())
    });

    Ok(entries.into_iter().find(|path| is_pdf_file(path)))
}

fn is_pdf_file(path: &Path) -> bool {
    if is_hidden_path(path) {
        return false;
    }

    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

fn is_hidden_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn file_size_is_stable(path: &Path, wait: Duration) -> Result<bool, String> {
    let first = fs::metadata(path)
        .map_err(|error| format!("failed to inspect file size for {}: {error}", path.display()))?
        .len();
    thread::sleep(wait);
    let second = fs::metadata(path)
        .map_err(|error| format!("failed to re-check file size for {}: {error}", path.display()))?
        .len();

    Ok(first == second)
}

fn file_is_unlocked(path: &Path) -> Result<bool, String> {
    let file = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|error| format!("failed to open {} for lock check: {error}", path.display()))?;

    match file.try_lock_exclusive() {
        Ok(()) => {
            let _ = file.unlock();
            Ok(true)
        }
        Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(false),
        Err(error) => Err(format!("failed to check lock state for {}: {error}", path.display())),
    }
}

fn conflict_renamed_path(destination_dir: &Path, file_name: &str) -> Result<PathBuf, String> {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let source = Path::new(file_name);
    let stem = source
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(file_name);
    let extension = source.extension().and_then(|value| value.to_str());

    let file_name = match extension {
        Some(extension) if !extension.is_empty() => format!("{stem}_{timestamp}.{extension}"),
        _ => format!("{stem}_{timestamp}"),
    };
    let candidate = destination_dir.join(&file_name);
    if !candidate.exists() {
        return Ok(candidate);
    }

    for _ in 0..MAX_CONFLICT_RENAME_ATTEMPTS {
        let unique_file_name = match extension {
            Some(extension) if !extension.is_empty() => {
                format!("{stem}_{timestamp}_{}.{extension}", unique_suffix())
            }
            _ => format!("{stem}_{timestamp}_{}", unique_suffix()),
        };
        let unique_candidate = destination_dir.join(unique_file_name);
        if !unique_candidate.exists() {
            return Ok(unique_candidate);
        }
    }

    Err(format!(
        "failed to generate unique filename under {} for {} after {} attempts",
        destination_dir.display(),
        file_name,
        MAX_CONFLICT_RENAME_ATTEMPTS
    ))
}

fn build_m3_activity_entry(
    status: &str,
    filename: &str,
    source_path: &str,
    destination_path: Option<String>,
    final_path: Option<String>,
    rule_id: Option<String>,
    rule_name: Option<String>,
    action_type: Option<&str>,
    message: Option<&str>,
    undo_eligible_until: Option<String>,
    undone_at: Option<String>,
    undo_source_entry_id: Option<String>,
) -> ActivityEntry {
    ActivityEntry {
        id: generate_entry_id("activity"),
        timestamp: iso_timestamp_now(),
        status: status.into(),
        filename: filename.into(),
        source_path: source_path.into(),
        destination_path,
        final_path,
        rule_id,
        rule_name,
        action_type: action_type.map(|value| value.into()),
        message: message.map(|value| value.into()),
        undo_eligible_until,
        undone_at,
        undo_source_entry_id,
    }
}

fn prepend_activity_entry(mut entries: Vec<ActivityEntry>, entry: ActivityEntry) -> Vec<ActivityEntry> {
    entries.insert(0, entry);
    entries.truncate(1000);
    entries
}

fn move_file_with_cross_device_fallback(source_path: &Path, final_path: &Path) -> Result<(), String> {
    match fs::rename(source_path, final_path) {
        Ok(()) => Ok(()),
        Err(error)
            if error.kind() == ErrorKind::CrossesDevices
                || error.raw_os_error() == Some(18)
                || error.raw_os_error() == Some(17) =>
        {
            fs::copy(source_path, final_path).map_err(|copy_error| {
                format!(
                    "failed to copy {} to {} after cross-device move error {error}: {copy_error}",
                    source_path.display(),
                    final_path.display()
                )
            })?;

            if let Err(remove_error) = fs::remove_file(source_path) {
                let _ = fs::remove_file(final_path);
                return Err(format!(
                    "copied {} to {} but failed to remove source after cross-device move error {error}: {remove_error}",
                    source_path.display(),
                    final_path.display()
                ));
            }

            Ok(())
        }
        Err(error) => Err(format!(
            "failed to move {} to {}: {error}",
            source_path.display(),
            final_path.display()
        )),
    }
}

fn iso_timestamp_now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn iso_timestamp_after_hours(hours: i64) -> String {
    (Utc::now() + chrono::Duration::hours(hours)).to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn generate_entry_id(prefix: &str) -> String {
    format!("{prefix}-{}", unique_suffix())
}

fn expand_user_path(value: &str, home_dir: &Path) -> PathBuf {
    if value == "~" {
        return home_dir.to_path_buf();
    }

    if let Some(rest) = value.strip_prefix("~/") {
        return home_dir.join(rest);
    }

    PathBuf::from(value)
}

fn display_user_path(path: &Path, home_dir: &Path) -> String {
    match path.strip_prefix(home_dir) {
        Ok(relative) if relative.as_os_str().is_empty() => "~".into(),
        Ok(relative) => format!("~/{}", relative.to_string_lossy()),
        Err(_) => path.display().to_string(),
    }
}

fn user_home_dir() -> Result<PathBuf, String> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| "failed to resolve the current user's home directory".into())
}

fn app_storage_files(app: &AppHandle) -> Result<StorageFiles, String> {
    let base_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("failed to resolve app data directory: {error}"))?;

    Ok(storage_files_for_base_dir(base_dir))
}

fn storage_files_for_base_dir(base_dir: PathBuf) -> StorageFiles {
    StorageFiles {
        rules: base_dir.join("rules.json"),
        settings: base_dir.join("settings.json"),
        activity: base_dir.join("activity.json"),
        retry_queue: base_dir.join("retry-queue.json"),
        base_dir,
    }
}

fn ensure_storage_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|error| {
        format!(
            "failed to create storage directory at {}: {error}",
            path.display()
        )
    })
}

#[cfg(test)]
fn read_json_or_default<T>(path: &Path, default: T) -> Result<T, String>
where
    T: Clone + DeserializeOwned,
{
    if let Some(parent) = path.parent() {
        ensure_storage_dir(parent)?;
    }

    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(default),
        Err(error) => {
            return Err(format!("failed to read {}: {error}", path.display()));
        }
    };

    match serde_json::from_slice::<T>(&bytes) {
        Ok(value) => Ok(value),
        Err(_) => Ok(default),
    }
}

fn load_persisted_or_initialize<T>(path: &Path, default: T) -> Result<T, String>
where
    T: Clone + DeserializeOwned + Serialize,
{
    if let Some(parent) = path.parent() {
        ensure_storage_dir(parent)?;
    }

    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            write_json_atomic(path, &default)?;
            return Ok(default);
        }
        Err(error) => {
            return Err(format!("failed to read {}: {error}", path.display()));
        }
    };

    match serde_json::from_slice::<T>(&bytes) {
        Ok(value) => Ok(value),
        Err(error) => {
            let backup_path = corrupt_backup_path(path);
            fs::write(&backup_path, &bytes).map_err(|write_error| {
                format!(
                    "failed to back up corrupt JSON from {} to {} after parse error {error}: {write_error}",
                    path.display(),
                    backup_path.display()
                )
            })?;
            eprintln!(
                "warning: failed to parse {}: {error}. Backed up corrupt bytes to {}",
                path.display(),
                backup_path.display()
            );
            write_json_atomic(path, &default)?;
            Ok(default)
        }
    }
}

fn corrupt_backup_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| format!("{name}.corrupt"))
        .unwrap_or_else(|| "storage.corrupt".to_string());

    path.with_file_name(file_name)
}

fn write_json_atomic<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        ensure_storage_dir(parent)?;
    }

    let temp_path = path.with_extension(format!("{}.tmp", unique_suffix()));
    let payload = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("failed to serialize {}: {error}", path.display()))?;

    fs::write(&temp_path, payload)
        .map_err(|error| format!("failed to write temp file {}: {error}", temp_path.display()))?;

    fs::rename(&temp_path, path).map_err(|error| {
        format!(
            "failed to move temp file {} into place at {}: {error}",
            temp_path.display(),
            path.display()
        )
    })
}

fn unique_suffix() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

fn validate_rules(rules: &[RuleDefinition]) -> Result<(), String> {
    let mut rule_ids = HashSet::new();

    for rule in rules {
        if rule.id.trim().is_empty() {
            return Err("rule id cannot be empty".into());
        }
        if rule.name.trim().is_empty() {
            return Err("rule name cannot be empty".into());
        }
        if rule.priority < 1 {
            return Err(format!("rule {} must have priority >= 1", rule.id));
        }
        if !matches!(rule.condition_logic.as_str(), "all" | "any") {
            return Err(format!("rule {} has invalid conditionLogic", rule.id));
        }
        if !rule_ids.insert(rule.id.clone()) {
            return Err(format!("rule {} appears more than once", rule.id));
        }
        if rule.conditions.is_empty() {
            return Err(format!("rule {} must contain at least one condition", rule.id));
        }
        if rule.actions.is_empty() {
            return Err(format!("rule {} must contain at least one action", rule.id));
        }

        for node in &rule.conditions {
            match node {
                RuleConditionNode::Condition(condition) => validate_condition(condition, &rule.id)?,
                RuleConditionNode::Group(group) => {
                    if group.condition_logic != "any" {
                        return Err(format!(
                            "rule {} contains a nested condition group that is not 'any'",
                            rule.id
                        ));
                    }
                    if group.conditions.is_empty() {
                        return Err(format!(
                            "rule {} contains an empty nested condition group",
                            rule.id
                        ));
                    }
                    for condition in &group.conditions {
                        validate_condition(condition, &rule.id)?;
                    }
                }
            }
        }

        for action in &rule.actions {
            validate_action(action, &rule.id)?;
        }
    }

    Ok(())
}

fn validate_condition(condition: &RuleCondition, rule_id: &str) -> Result<(), String> {
    let valid_type = matches!(
        condition.kind.as_str(),
        "extension"
            | "filename_contains"
            | "filename_startswith"
            | "filename_endswith"
            | "mime_type"
            | "file_size_gt"
            | "file_size_lt"
            | "date_added_older_than"
            | "source_url_contains"
            | "filename_regex"
    );

    if !valid_type {
        return Err(format!(
            "rule {rule_id} has unsupported condition type {}",
            condition.kind
        ));
    }

    if condition.value.trim().is_empty() {
        return Err(format!("rule {rule_id} contains a condition with an empty value"));
    }

    Ok(())
}

fn validate_action(action: &RuleActionDefinition, rule_id: &str) -> Result<(), String> {
    match action.kind.as_str() {
        "move" | "move_with_date" | "rename" => {
            if action
                .value
                .as_ref()
                .map(|value| value.trim().is_empty())
                .unwrap_or(true)
            {
                return Err(format!(
                    "rule {rule_id} action {} requires a non-empty value",
                    action.kind
                ));
            }
        }
        "trash" | "notify_only" => {}
        _ => {
            return Err(format!(
                "rule {rule_id} has unsupported action type {}",
                action.kind
            ));
        }
    }

    Ok(())
}

fn validate_settings(settings: &SettingsState) -> Result<(), String> {
    if settings.watched_folders.len() > 10 {
        return Err("settings cannot contain more than 10 watched folders".into());
    }

    if !matches!(settings.conflict_behavior.as_str(), "rename" | "skip") {
        return Err("settings conflictBehavior must be 'rename' or 'skip'".into());
    }

    if !(7..=90).contains(&settings.activity_retention_days) {
        return Err("settings activityRetentionDays must be between 7 and 90".into());
    }

    if settings.file_stability_wait_seconds == 0 {
        return Err("settings fileStabilityWaitSeconds must be greater than 0".into());
    }

    if !matches!(settings.license.tier.as_str(), "free" | "monthly" | "lifetime") {
        return Err("settings license tier is invalid".into());
    }

    Ok(())
}

fn validate_activity(entries: &[ActivityEntry]) -> Result<(), String> {
    for entry in entries {
        if entry.id.trim().is_empty() {
            return Err("activity entry id cannot be empty".into());
        }
        if entry.timestamp.trim().is_empty() {
            return Err(format!("activity entry {} timestamp cannot be empty", entry.id));
        }
        if entry.filename.trim().is_empty() {
            return Err("activity entry filename cannot be empty".into());
        }
        if entry.source_path.trim().is_empty() {
            return Err(format!("activity entry {} sourcePath cannot be empty", entry.id));
        }
        if !matches!(entry.status.as_str(), "success" | "skipped" | "failed" | "undone") {
            return Err(format!("activity entry {} has an invalid status", entry.id));
        }
        if let Some(action_type) = &entry.action_type {
            if !matches!(
                action_type.as_str(),
                "move" | "move_with_date" | "rename" | "trash" | "notify_only"
            ) {
                return Err(format!(
                    "activity entry {} has an invalid actionType",
                    entry.id
                ));
            }
        }
    }

    Ok(())
}

fn validate_retry_queue(entries: &[RetryQueueEntry]) -> Result<(), String> {
    if entries.len() > 500 {
        return Err("retry queue cannot contain more than 500 entries".into());
    }

    for entry in entries {
        if entry.id.trim().is_empty() {
            return Err("retry queue entry id cannot be empty".into());
        }
        if entry.source_path.trim().is_empty() {
            return Err(format!("retry queue entry {} sourcePath cannot be empty", entry.id));
        }
        if entry.filename.trim().is_empty() {
            return Err(format!("retry queue entry {} filename cannot be empty", entry.id));
        }
    }

    Ok(())
}

fn default_rules() -> Vec<RuleDefinition> {
    let seed_timestamp = "2026-05-17T23:21:42.000Z".to_string();

    vec![
        RuleDefinition {
            id: "move-disk-images".into(),
            name: "Move Disk Images".into(),
            enabled: false,
            priority: 1,
            condition_logic: "all".into(),
            conditions: vec![RuleConditionNode::Group(RuleConditionGroup {
                condition_logic: "any".into(),
                conditions: vec![
                    RuleCondition {
                        kind: "extension".into(),
                        value: "dmg".into(),
                        negate: false,
                    },
                    RuleCondition {
                        kind: "extension".into(),
                        value: "pkg".into(),
                        negate: false,
                    },
                ],
            })],
            actions: vec![RuleActionDefinition {
                kind: "move".into(),
                value: Some("~/Downloads/Installers/".into()),
            }],
            is_prebuilt: true,
            created_at: seed_timestamp.clone(),
            updated_at: seed_timestamp.clone(),
        },
        RuleDefinition {
            id: "clean-stale-partials".into(),
            name: "Clean Stale Partial Downloads".into(),
            enabled: false,
            priority: 2,
            condition_logic: "all".into(),
            conditions: vec![
                RuleConditionNode::Group(RuleConditionGroup {
                    condition_logic: "any".into(),
                    conditions: vec![
                        RuleCondition {
                            kind: "extension".into(),
                            value: "crdownload".into(),
                            negate: false,
                        },
                        RuleCondition {
                            kind: "extension".into(),
                            value: "part".into(),
                            negate: false,
                        },
                        RuleCondition {
                            kind: "extension".into(),
                            value: "download".into(),
                            negate: false,
                        },
                    ],
                }),
                RuleConditionNode::Condition(RuleCondition {
                    kind: "date_added_older_than".into(),
                    value: "1 day".into(),
                    negate: false,
                }),
            ],
            actions: vec![RuleActionDefinition {
                kind: "trash".into(),
                value: None,
            }],
            is_prebuilt: true,
            created_at: seed_timestamp.clone(),
            updated_at: seed_timestamp.clone(),
        },
        RuleDefinition {
            id: "move-pdfs".into(),
            name: "Move PDFs".into(),
            enabled: false,
            priority: 5,
            condition_logic: "all".into(),
            conditions: vec![RuleConditionNode::Condition(RuleCondition {
                kind: "extension".into(),
                value: "pdf".into(),
                negate: false,
            })],
            actions: vec![RuleActionDefinition {
                kind: "move".into(),
                value: Some("~/Documents/PDFs/".into()),
            }],
            is_prebuilt: true,
            created_at: seed_timestamp.clone(),
            updated_at: seed_timestamp,
        },
    ]
}

fn default_settings() -> SettingsState {
    SettingsState {
        onboarding_completed: false,
        launch_at_login: true,
        daily_summary_enabled: true,
        daily_summary_started_at: None,
        daily_summary_last_sent_at: None,
        watched_folders: vec!["~/Downloads".into()],
        file_stability_wait_seconds: 5,
        conflict_behavior: "rename".into(),
        activity_retention_days: 30,
        license: LicenseState {
            tier: "free".into(),
            key: None,
            validated_at: None,
            expires_at: None,
        },
    }
}

fn default_activity() -> Vec<ActivityEntry> {
    vec![ActivityEntry {
        id: "activity-seed-1".into(),
        timestamp: "2026-05-17T15:30:00.000Z".into(),
        status: "success".into(),
        filename: "Invoice-April.pdf".into(),
        source_path: "~/Downloads/Invoice-April.pdf".into(),
        destination_path: Some("~/Documents/Finance".into()),
        final_path: Some("~/Documents/Finance/Invoice-April.pdf".into()),
        rule_id: Some("move-invoices-receipts".into()),
        rule_name: Some("Move Invoices and Receipts".into()),
        action_type: Some("move".into()),
        message: Some("Moved successfully".into()),
        undo_eligible_until: Some("2026-05-18T15:30:00.000Z".into()),
        undone_at: None,
        undo_source_entry_id: None,
    }]
}

fn default_retry_queue() -> Vec<RetryQueueEntry> {
    Vec::new()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            storage_status,
            load_rules,
            save_rules,
            load_settings,
            save_settings,
            load_activity,
            save_activity,
            load_retry_queue,
            save_retry_queue,
            run_m3_pdf_slice,
            undo_activity_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_storage_root(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("seiri-tests-{name}-{}", unique_suffix()));
        fs::create_dir_all(&dir).expect("temp dir should be creatable");
        dir
    }

    #[test]
    fn read_defaults_when_file_is_missing() {
        let base_dir = temp_storage_root("defaults");
        let files = storage_files_for_base_dir(base_dir);

        let settings = load_persisted_or_initialize(&files.settings, default_settings())
            .expect("missing file should return defaults");

        assert_eq!(settings, default_settings());
        assert!(files.settings.exists());
    }

    #[test]
    fn malformed_json_falls_back_to_default() {
        let base_dir = temp_storage_root("malformed");
        let files = storage_files_for_base_dir(base_dir);

        ensure_storage_dir(&files.base_dir).expect("storage dir should exist");
        fs::write(&files.settings, "{bad json").expect("fixture should be writable");

        let settings = load_persisted_or_initialize(&files.settings, default_settings())
            .expect("malformed file should not error");

        assert_eq!(settings, default_settings());
    }

    #[test]
    fn atomic_write_persists_payload() {
        let base_dir = temp_storage_root("atomic");
        let files = storage_files_for_base_dir(base_dir);
        let rules = default_rules();

        write_json_atomic(&files.rules, &rules).expect("write should succeed");

        let loaded_rules =
            read_json_or_default(&files.rules, Vec::<RuleDefinition>::new()).expect("read");

        assert_eq!(loaded_rules, rules);
    }

    #[test]
    fn ensure_storage_dir_creates_nested_path() {
        let base_dir = temp_storage_root("mkdir").join("nested").join("deeper");
        ensure_storage_dir(&base_dir).expect("directory creation should succeed");
        assert!(base_dir.exists());
    }

    #[test]
    fn validate_rules_rejects_duplicate_ids() {
        let mut rules = default_rules();
        let duplicate = rules[0].clone();
        rules.push(duplicate);

        let error = validate_rules(&rules).expect_err("duplicate ids should be rejected");

        assert!(error.contains("appears more than once"));
    }

    #[test]
    fn validate_activity_requires_timestamp_and_source_path() {
        let mut entries = default_activity();
        entries[0].timestamp = String::new();

        let timestamp_error =
            validate_activity(&entries).expect_err("empty timestamp should be rejected");
        assert!(timestamp_error.contains("timestamp cannot be empty"));

        let mut source_entries = default_activity();
        source_entries[0].source_path = String::new();

        let source_error =
            validate_activity(&source_entries).expect_err("empty sourcePath should be rejected");
        assert!(source_error.contains("sourcePath cannot be empty"));
    }

    #[test]
    fn m3_slice_moves_the_first_pdf_and_then_undoes_it() {
        let storage_root = temp_storage_root("m3-run");
        let home_root = temp_storage_root("m3-home");
        let downloads_dir = home_root.join("Downloads");
        let destination_dir = home_root.join("Documents").join("PDFs");
        ensure_storage_dir(&downloads_dir).expect("downloads dir should exist");

        fs::write(downloads_dir.join("zeta.pdf"), b"zeta").expect("fixture should be writable");
        fs::write(downloads_dir.join("alpha.pdf"), b"alpha").expect("fixture should be writable");
        fs::write(downloads_dir.join(".hidden.pdf"), b"hidden")
            .expect("fixture should be writable");

        let files = storage_files_for_base_dir(storage_root);
        let mut settings = default_settings();
        settings.file_stability_wait_seconds = 1;
        write_json_atomic(&files.settings, &settings).expect("settings should write");

        let run_result =
            run_m3_pdf_slice_with_context(&files, &home_root).expect("run should succeed");

        assert_eq!(run_result.status, "moved");
        let moved_entry = run_result
            .activity_entry
            .clone()
            .expect("run should create an activity entry");
        assert_eq!(moved_entry.filename, "alpha.pdf");
        assert_eq!(moved_entry.rule_id.as_deref(), Some("move-pdfs"));
        assert_eq!(moved_entry.action_type.as_deref(), Some("move"));
        assert!(destination_dir.join("alpha.pdf").exists());
        assert!(downloads_dir.join("zeta.pdf").exists());
        assert!(downloads_dir.join(".hidden.pdf").exists());

        let undo_result = undo_activity_entry_with_context(&files, &home_root, &moved_entry.id)
            .expect("undo should succeed");

        assert_eq!(undo_result.status, "undone");
        assert!(downloads_dir.join("alpha.pdf").exists());
        assert!(!destination_dir.join("alpha.pdf").exists());
        assert_eq!(
            undo_result
                .activity_entry
                .as_ref()
                .and_then(|entry| entry.undo_source_entry_id.as_deref()),
            Some(moved_entry.id.as_str())
        );
    }

    #[test]
    fn m3_slice_renames_on_conflict_when_destination_exists() {
        let storage_root = temp_storage_root("m3-conflict");
        let home_root = temp_storage_root("m3-conflict-home");
        let downloads_dir = home_root.join("Downloads");
        let destination_dir = home_root.join("Documents").join("PDFs");
        ensure_storage_dir(&downloads_dir).expect("downloads dir should exist");
        ensure_storage_dir(&destination_dir).expect("destination dir should exist");

        fs::write(downloads_dir.join("invoice.pdf"), b"source").expect("fixture should be writable");
        fs::write(destination_dir.join("invoice.pdf"), b"existing")
            .expect("fixture should be writable");

        let files = storage_files_for_base_dir(storage_root);
        let mut settings = default_settings();
        settings.file_stability_wait_seconds = 1;
        settings.conflict_behavior = "rename".into();
        write_json_atomic(&files.settings, &settings).expect("settings should write");

        let run_result =
            run_m3_pdf_slice_with_context(&files, &home_root).expect("run should succeed");

        assert_eq!(run_result.status, "moved");
        let moved_entry = run_result
            .activity_entry
            .as_ref()
            .expect("run should create an activity entry");
        let final_path = moved_entry
            .final_path
            .as_ref()
            .expect("final path should be recorded");

        assert!(final_path.contains("invoice_"));
        assert!(final_path.ends_with(".pdf"));
        assert!(destination_dir.join("invoice.pdf").exists());
        assert!(final_path != "~/Documents/PDFs/invoice.pdf");
    }

    #[test]
    fn m3_slice_skips_when_destination_conflict_is_skip() {
        let storage_root = temp_storage_root("m3-skip");
        let home_root = temp_storage_root("m3-skip-home");
        let downloads_dir = home_root.join("Downloads");
        let destination_dir = home_root.join("Documents").join("PDFs");
        ensure_storage_dir(&downloads_dir).expect("downloads dir should exist");
        ensure_storage_dir(&destination_dir).expect("destination dir should exist");

        fs::write(downloads_dir.join("report.pdf"), b"source").expect("fixture should be writable");
        fs::write(destination_dir.join("report.pdf"), b"existing")
            .expect("fixture should be writable");

        let files = storage_files_for_base_dir(storage_root);
        let mut settings = default_settings();
        settings.file_stability_wait_seconds = 1;
        settings.conflict_behavior = "skip".into();
        write_json_atomic(&files.settings, &settings).expect("settings should write");

        let run_result =
            run_m3_pdf_slice_with_context(&files, &home_root).expect("run should succeed");

        assert_eq!(run_result.status, "skipped");
        assert!(
            run_result
                .activity_entry
                .as_ref()
                .expect("skip should create an activity entry")
                .message
                .as_deref()
                .unwrap_or_default()
                .contains("already exists at destination")
        );
        assert!(downloads_dir.join("report.pdf").exists());
        assert!(destination_dir.join("report.pdf").exists());
    }

    #[test]
    fn conflict_renamed_path_avoids_existing_same_second_collision() {
        let destination_dir = temp_storage_root("m3-rename-helper");
        let first_candidate =
            conflict_renamed_path(&destination_dir, "invoice.pdf").expect("first path");

        fs::write(&first_candidate, b"existing").expect("fixture should be writable");

        let second_candidate =
            conflict_renamed_path(&destination_dir, "invoice.pdf").expect("second path");

        assert_ne!(first_candidate, second_candidate);
        assert!(!second_candidate.exists());
        assert!(second_candidate
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .starts_with("invoice_"));
    }
}
