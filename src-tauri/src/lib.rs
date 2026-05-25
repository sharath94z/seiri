use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

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
    for rule in rules {
        if rule.name.trim().is_empty() {
            return Err("rule name cannot be empty".into());
        }
        if rule.priority < 1 {
            return Err(format!("rule {} must have priority >= 1", rule.id));
        }
        if !matches!(rule.condition_logic.as_str(), "all" | "any") {
            return Err(format!("rule {} has invalid conditionLogic", rule.id));
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
        if entry.filename.trim().is_empty() {
            return Err("activity entry filename cannot be empty".into());
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
            save_retry_queue
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
}
