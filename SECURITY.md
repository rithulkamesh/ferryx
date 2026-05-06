# Security Policy

ferryx sits on language boundaries (Rust/Python/ABI). Security, safety, and correctness are inseparable.

## Reporting a Vulnerability

Email `security@ferryx.dev` with:

- Affected version(s).
- Reproduction steps or PoC.
- Impact assessment.
- Suggested mitigation (if known).

Do not open public issues for unpatched vulnerabilities.

## Supported Versions

Security fixes are backported to:

- Latest stable release.
- Previous minor release line (when feasible).
- Critical issues may receive targeted backports further.

## Security Review Surface

Priority areas:

- FFI boundary (`ferryx-ffi`): pointer lifetimes, nullability, ABI layout.
- Macro expansion (`ferryx-macros`): malicious/malformed input handling.
- Runtime registry (`ferryx-runtime`): metadata trust and deserialization boundaries.
- Build orchestration (`ferryx-build`): command invocation and artifact integrity.

## Unsafe Rust and FFI

Mandatory for any unsafe/ABI code:

- `#[repr(C)]` or explicit layout intent.
- Documented ownership transfer and lifetime model.
- Fuzz or property tests for boundary parsing where applicable.
- No unchecked transmute without dedicated design review.

## ABI Compatibility and Evolution

- ABI-affecting changes require RFC.
- Additive ABI changes preferred; breaking ABI requires major release and migration path.
- `ferryx_ffi::ABI_VERSION` must change when ABI contracts change.

## Dependency and Supply Chain

- CI runs `cargo audit`.
- Lockfile updates should be explicit in release notes.
- New dependencies need security rationale in PR.

## Disclosure Process

1. Acknowledge report within 72 hours.
2. Confirm severity and scope.
3. Prepare patch and coordinated release timeline.
4. Publish security advisory with remediation steps.

