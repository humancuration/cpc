# Docs Consistency Checks

Purpose
These lightweight checks prevent regressions where critical docs (true entry points and canonical indexes) become undiscoverable. They are intentionally minimal and substring-based. They are not style linting; they only guard basic discoverability paths.

What is a “needle”?
A needle is a required substring that must appear in a target file. Example:
- Target file must contain substring: docs/adr/0009-bootstrap-stub-toggles.md
If the substring is missing, CI fails with a clear hint to either add the reference or update the rule.

Configuration format
- File path: tools/ci/needles.txt
- Each non-empty, non-comment line is one rule in the form:
  "<target_file>|<required_substring>"
- Split is on the first '|' and both sides are trimmed.
- Lines starting with '#' are comments.
- Empty lines are ignored.

Example
The initial rules shipped with the repo are:
```
apps/api_server/README.md|docs/api_server/volunteer_reputation_stub.md
docs/README.md|docs/adr/0009-bootstrap-stub-toggles.md
```

How to run locally
Use the same command as CI:
```
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency
```

Where to update rules
- Do not edit tools/ci/src/main.rs to add needles.
- Edit tools/ci/needles.txt instead; it is the source of truth for the checks.

Failure modes and guidance
- Missing config:
  "Missing tools/ci/needles.txt. Please create it with lines like: <file>|<required_substring>. See docs/dev/docs-consistency-checks.md."
- File not found:
  "File not found: <target_file>"
- Missing substring:
  The tool reports a discoverability failure showing the file and the missing substring, with guidance. Fix by either:
  a) Adding a reference (link or text containing the required substring) to the target file, or
  b) If the rule is obsolete or the phrasing/path changed intentionally, update tools/ci/needles.txt accordingly.

How CI runs the checker
- Command: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency
- Location: The checker lives in tools/ci.
- Workflow wiring: It runs in .github/workflows/ci.yml before tests. The workflow and command are unchanged.

Intent and philosophy
- Minimal, substring-based net to keep crucial docs discoverable.
- Not about wording or style. It’s a friendly, low-friction safeguard that helps “future us.”

References
- Overview: tools/ci/README.md
- Config: tools/ci/needles.txt
- Runner: .github/workflows/ci.yml (step “Check docs consistency”)
- Command: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency
## Troubleshooting

If the docs-consistency check fails, it usually means a rule in tools/ci/needles.txt no longer matches the docs. Use these quick fixes to resolve without digging into source.

Common cases and fixes
- Missing tools/ci/needles.txt
  - Create the file and add one rule per line: &lt;target_file&gt;|&lt;required_substring&gt;
  - Example starter:
    docs/README.md|What’s new
    docs/dev/CONTRIBUTING.md|docs-consistency
- File not found: &lt;target_file&gt;
  - The target file was renamed/moved or never committed.
  - Fix: update the left side of the rule to the correct path (forward slashes), or restore/create the file.
- Missing substring in target file
  - Decide if the docs should contain that link/text:
    - If yes: add the minimal missing text/link to the target file (prefer short, discoverable anchors).
    - If no (rule is outdated): update or remove the rule in tools/ci/needles.txt.
- Renamed or moved docs
  - Update the left side (target_file) to the new path, and confirm the right side (required_substring) still exists in the new file.
- New index/entry docs added
  - Add a needle when a new doc needs discoverability (e.g., add an index link in README or a section TOC).
  - Rule format: path/to/new_doc.md|Anchor text or link that should appear elsewhere.
- Local run vs CI run differences
  - Reproduce locally:
    cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency
  - Expect exit code 0 on success. If local passes but CI fails, ensure all changed files are committed and paths use forward slashes.
- Windows path gotchas
  - Always use forward slashes in target_file (e.g., docs/dev/file.md).
  - required_substring is case-sensitive; match exact casing from the doc.

Quick fix checklist
- Confirm tools/ci/needles.txt exists and uses one rule per line.
- Verify target_file exists at the path with forward slashes.
- Open target_file and check the exact, case-sensitive required_substring.
- If the rule is stale, update the path and/or substring (or remove it).
- Rerun locally and confirm exit code 0.

Run command
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency

See also: schema guardrails as developer-run consistency checks (docs/dev/schema-checks.md, docs/dev/schema-guardrails-architecture.md).