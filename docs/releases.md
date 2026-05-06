# Release Channels

ferryx supports three channels:

- stable
- beta
- nightly

## Compatibility Matrix

- IR schema compatibility enforced by `IR_VERSION`.
- rewrite pipeline compatibility validated through semantic snapshots.
- emitter compatibility validated in cross-target verification.

## Channel Rules

- stable: semver-governed, migration notes required for breaks.
- beta: preview of next stable line with compatibility warnings.
- nightly: rolling build with date-tagged pre-release metadata.

