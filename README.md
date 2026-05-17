# Seiri

Seiri is a macOS file organization utility built with Tauri, React, TypeScript, and Rust.

## Source Of Truth

- [Product requirements](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-prd-v1.2.md)
- [Implementation plan](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-implementation-plan.md)
- [UI/UX spec](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-uiux-spec.md)
- [Hazel notes](/Users/sharatharadhyamath/Documents/github/seiri/docs/hazel-implementation-notes.md)
- [Build checklist](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-build-checklist.md)

## Current Focus

The app shell and foundation scaffold are in place. The next deliberate milestone is M1 engine feasibility.

## Running Locally

```bash
npm install
npm run dev
```

For the native app shell:

```bash
source "$HOME/.cargo/env"
cargo check --manifest-path src-tauri/Cargo.toml
```

## Project Notes

- M0 uses plain CSS for now.
- Keep changes intentional and milestone-driven.
- Update [docs/seiri-build-checklist.md](/Users/sharatharadhyamath/Documents/github/seiri/docs/seiri-build-checklist.md) as work lands.
