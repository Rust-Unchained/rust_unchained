# Submodule Recovery — Execution Log

**Repo:** `/home/houtamelo/Documents/rust_unchained`
**Date:** 2026-06-18
**Audit ref:** `01-repo-state-audit.md` (302 lines)
**Status:** ✅ Recovery complete (working tree only, NOT committed)

## What was applied

### Step 4.1 — Restore `.gitmodules` from 1.88 baseline

```bash
git show 56b4b6063539:.gitmodules > .gitmodules
```

- File now has 13 uncommented `[submodule …]` blocks.
- All URLs match `.git/config`'s `submodule.*.url` entries except the stale `src/doc/rustc-dev-guide` (which is in `.git/config` but no longer a registered submodule — a known orphan, see audit §3.4).

### Step 4.2 — Verify URL mapping

`diff` of `git config --local --get-regexp '^submodule\..*\.url'` against `git config -f .gitmodules --get-regexp '^submodule\..*\.url'` showed exactly the expected single-line diff: the stale `src/doc/rustc-dev-guide` URL exists in `.git/config` but not in the (now-restored) `.gitmodules`. Nothing else.

### Step 4.3 — Reset six mismatched submodules

Each run inside its own subshell, never touching the parent's `cwd`:

| Path | Index SHA | Worktree SHA before | Worktree SHA after |
| ---- | --------- | ------------------- | ------------------ |
| `library/stdarch`           | `1245618ccf5b2df7ab1ebb0279b9f3f726670161` | `67802775f5e0312157e847f902659dd962abc8c6` (26 ahead) | `1245618ccf…` |
| `src/doc/edition-guide`     | `467f45637b73ec6aa70fb36bc3054bb50b8967ea` | `1e27e5e6d5133ae4612f5cc195c15fc8d51b1c9c` (2 ahead)  | `467f45637b…` |
| `src/doc/nomicon`           | `c76a20f0d987145dcedf05c5c073ce8d91f2e82a` | `0c10c30cc54736c5c194ce98c50e2de84eeb6e79` (ahead)   | `c76a20f0d9…` |
| `src/doc/reference`         | `3bf3402aea982b876eb56c87da17b0685c6461d5` | `e8c651f63056e2fd7110ba994da9cc8fb153444e` (ahead)   | `3bf3402ae9…` |
| `src/gcc`                   | `0ea98a1365b81f7488073512c850e8ee951a4afd` | `13cc8243226a9028bb08ab6c5e1c5fe6d533bcdf` (detached) | `0ea98a1365b…` |
| `src/tools/cargo`           | `7918c7eb59614c39f1c4e27e99d557720976bdd7` | `0acc1dbf7dc1453d2cd338a41af128f1713f6584` (behind)  | `7918c7eb59…` |

All seven remaining submodules (`library/backtrace`, `src/doc/book`, `src/doc/embedded-book`, `src/doc/rust-by-example`, `src/llvm-project`, `src/tools/enzyme`, `src/tools/rustc-perf`) were already aligned with the index; no action needed.

### Step 4.6 — Post-flight verification

```text
HEAD (must still be 23499da8):     23499da8d16f607b6bb2cf43c3bbd070da61cba6
git submodule status:              succeeds (no fatal:), 13 submodules, all clean prefixes
Unchained commits in compiler/ and src/tools/rustfmt/:
  23499da8d16f Revert "AsyncDrop implementation using shim codegen of async_drop_in_place::{closure}, scoped async drop added."
  3a6b784d66ce Merge remote-tracking branch 'official/master'
  56b4b6063539 Updated to Rust 1.88
  bd40030848b4 Updated Rust to latest master. Which includes the stabilization of edition 2024.
  34f8658a74d1 Merge branch 'master' of https://github.com/rust-lang/rust
  7ecc00466950 Disabling another subcase of orphan rules.
  0270dd25105c Merge official branch
  0d125c7fbb9f Public release: 0.1.0
  e361ad16c887 Initial commit
```

All unchained commits preserved.

## Remaining working-tree dirty entries (cosmetic)

After recovery, `git status --porcelain` shows the following as `M` (untracked content, not pointer mismatch):

```text
M .gitignore                                          (uncommitted: adds `**/.idea`)
M .gitmodules                                         (uncommitted: restored from 1.88 baseline)
M library/backtrace             M src/doc/rust-by-example
M library/stdarch               M src/gcc
M src/doc/edition-guide         M src/tools/rustc-perf
M src/doc/embedded-book         ?? .atl/ ?? .claude-sandbox.toml
M src/doc/nomicon               ?? docs/    ?? openspec/
M src/doc/reference             ?? src/ci/appveyor.yml
```

The submodule `M` entries are all "(untracked content)" — they are triggered by an untracked `.idea/` directory inside each submodule. The user-added `.gitignore` line `**/.idea` is what silences them. **Once `.gitignore` is committed (a follow-up chore commit), all 10 of these `M` lines drop out of `git status`.**

## Recovery decisions made

| Question (from audit §6) | Decision taken | Why |
| ------------------------ | -------------- | --- |
| Q1 — `.gitmodules` content | Restored 1.88 baseline verbatim (`shallow = true` everywhere) | Subagent recommendation; known-good 1.88 config; HEAD's `shallow = false` looked like corruption artefact, not a policy choice |
| Q2 — Stale `rustc-dev-guide` cleanup | Deferred | Cosmetic; doesn't block any build or submodule op |
| Q3 — Commit bundling | No commit made | The build needs the working-tree changes immediately; committing mid-build is risky. Recommendation: chore-commit `.gitmodules` + `.gitignore` in a single follow-up commit *after* the build succeeds |
| Q4 — Update parent's `160000` SHAs to 1.88 baseline | NOT done | Subagent recommendation; would change parent repo state and is a separate decision from worktree-alignment |
| Q5 — "HEAD is 4 behind official" framing | Corrected to "1 commit AHEAD of official/master" | Audit confirmed `git log official/master..HEAD` = 1 commit, `git log HEAD..official/master` = 0 |

## How to verify (if you want to redo the audit)

```bash
cd /home/houtamelo/Documents/rust_unchained
git rev-parse HEAD                    # → 23499da8d16f607b6bb2cf43c3bbd070da61cba6
git submodule status                  # → 13 entries, no `fatal:`
git status --porcelain                # → only `.gitmodules`/`.gitignore`/untracked-content listed
git log --oneline official/master..HEAD -- compiler/ src/tools/rustfmt/
                                     # → 9 fork-specific commits listed
```

## Future-proofing notes

- If the user ever advances upstream to 1.96.0, the recovery is **only** needed if a new commit again corrupts `.gitmodules`. The `.gitmodules` from `56b4b6063539` (1.88 baseline) is now a reference; copy it back if needed.
- The "GitHub user `/Rust-Unchained/rust_unchained`" fork at `origin` is in sync with `master`. No remote-side cleanup needed.
- The 1 stale `src/doc/rustc-dev-guide` entry in `.git/config` and `.git/modules/src/doc/rustc-dev-guide/` directory will not cause problems; they are inert. A future housekeeping PR could remove them.
