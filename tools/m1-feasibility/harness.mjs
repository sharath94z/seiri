import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const here = path.dirname(fileURLToPath(import.meta.url));
const fixturePath = path.join(here, "fixtures", "m1-cases.json");

const capabilitySupport = {
  extension: {
    verdict: "native",
    reason: "Organize supports standard file filters such as extension matching.",
  },
  orderedEvaluation: {
    verdict: "wrapper",
    reason:
      "Organize evaluates rules top-to-bottom, but Seiri still needs wrapper-side first-match stopping semantics.",
  },
  nestedBoolean: {
    verdict: "wrapper",
    reason:
      "Organize exposes flat `all` / `any` / `none` grouping, not Seiri's nested boolean tree shape.",
  },
  negation: {
    verdict: "wrapper",
    reason:
      "Negated filters exist, but Seiri's exclusion model is expressed through nested condition groups.",
  },
  simulate: {
    verdict: "native",
    reason: "Organize has a built-in simulate / dry-run command.",
  },
  move: {
    verdict: "native",
    reason: "Move is a first-class organize action.",
  },
  rename: {
    verdict: "native",
    reason: "Rename is a first-class organize action.",
  },
  dateFolder: {
    verdict: "wrapper",
    reason:
      "Date-folder routing is possible through destination templating, but Seiri must translate its own rule syntax.",
  },
  trash: {
    verdict: "native",
    reason: "Trash is a first-class organize action.",
  },
  sourceUrlProvenance: {
    verdict: "wrapper",
    reason:
      "Organize does not document source URL provenance as a native filter, so Seiri must prefilter it.",
  },
};

const verdictRank = {
  native: 0,
  wrapper: 1,
  missing: 2,
};

function assertScenarioShape(entry, index) {
  if (typeof entry !== "object" || entry === null || Array.isArray(entry)) {
    throw new Error(`Case ${index + 1} is not an object.`);
  }

  for (const key of ["id", "name", "goal", "status", "expectedCompatibility", "requiredCapabilities"]) {
    if (key === "requiredCapabilities") {
      if (!Array.isArray(entry[key]) || entry[key].length === 0) {
        throw new Error(`Case ${index + 1} is missing a valid "${key}" array.`);
      }
      continue;
    }

    if (typeof entry[key] !== "string" || entry[key].trim() === "") {
      throw new Error(`Case ${index + 1} is missing a valid "${key}" string.`);
    }
  }

  if (entry.sampleFile && typeof entry.sampleFile !== "object") {
    throw new Error(`Case ${index + 1} has an invalid "sampleFile" value.`);
  }

  if (entry.ruleSet && !Array.isArray(entry.ruleSet)) {
    throw new Error(`Case ${index + 1} has an invalid "ruleSet" value.`);
  }
}

function evaluateCapability(capability) {
  const support = capabilitySupport[capability];
  if (!support) {
    return {
      verdict: "missing",
      reason: `No organize support mapping exists for capability "${capability}".`,
    };
  }

  return support;
}

function evaluateScenario(entry) {
  const capabilityResults = entry.requiredCapabilities.map(evaluateCapability);
  const verdict = capabilityResults.reduce((current, next) =>
    verdictRank[next.verdict] > verdictRank[current] ? next.verdict : current,
  "native");

  return {
    verdict,
    capabilityResults,
  };
}

function formatCapabilities(capabilityResults) {
  return capabilityResults
    .map((result) => `${result.verdict}:${result.reason}`)
    .join(" | ");
}

async function main() {
  const raw = await readFile(fixturePath, "utf8");
  const cases = JSON.parse(raw);

  if (!Array.isArray(cases) || cases.length === 0) {
    throw new Error("M1 feasibility fixture must contain at least one case.");
  }

  cases.forEach(assertScenarioShape);

  const results = cases.map((entry) => {
    const evaluation = evaluateScenario(entry);
    return {
      ...entry,
      actualCompatibility: evaluation.verdict,
      capabilityResults: evaluation.capabilityResults,
      matchesExpectation: evaluation.verdict === entry.expectedCompatibility,
    };
  });

  const summary = results.reduce(
    (acc, entry) => {
      acc[entry.actualCompatibility] += 1;
      if (!entry.matchesExpectation) {
        acc.mismatches += 1;
      }
      return acc;
    },
    { native: 0, wrapper: 0, missing: 0, mismatches: 0 },
  );

  console.log("Seiri M1 feasibility harness");
  console.log(`Loaded ${results.length} scenario definitions from ${path.relative(process.cwd(), fixturePath)}`);
  console.log(
    `Summary: native=${summary.native}, wrapper=${summary.wrapper}, missing=${summary.missing}, mismatches=${summary.mismatches}`,
  );
  console.log("");

  for (const entry of results) {
    const verdictMark = entry.matchesExpectation ? "PASS" : "FAIL";
    console.log(`[${verdictMark}] ${entry.id}: ${entry.name}`);
    console.log(`  goal: ${entry.goal}`);
    console.log(`  status: ${entry.status}`);
    console.log(`  expected: ${entry.expectedCompatibility}`);
    console.log(`  actual:   ${entry.actualCompatibility}`);
    if (entry.sampleFile?.name) {
      console.log(`  sample:   ${entry.sampleFile.name}`);
    }
    if (entry.expectedOutcome) {
      console.log(`  outcome:  ${entry.expectedOutcome}`);
    }
    console.log(`  evidence: ${entry.evidence}`);
    console.log(`  support:  ${formatCapabilities(entry.capabilityResults)}`);
    console.log("");
  }

  if (summary.mismatches > 0) {
    throw new Error("M1 harness found compatibility mismatches. Update the fixture or support map.");
  }

  console.log("Harness state: verified feasibility matrix");
  console.log("Prepared behavior: scenario loading, shape validation, compatibility classification, and mismatch detection.");
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
});
