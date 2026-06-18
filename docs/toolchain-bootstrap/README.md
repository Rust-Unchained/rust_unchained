# Toolchain Bootstrap

This directory tracks the work to make the rust_unchained sandbox capable of
building a working Rust 1.88 toolchain (matching the user's host build), with
the fork's `compiler/` and `src/tools/rustfmt/` changes preserved.

## Status: ✅ Complete

- **Build:** `./x.py build --stage 2 --target x86_64-unknown-linux-gnu` succeeds in 20:03
- **Toolchain:** `rustc 1.88.5-nightly + LLVM 20.1.2` (matches previous host build)
- **Tests:** 5241+ pass, 0 FAILED, 2 sandbox-only POSIX hangs (`test_process_group_*`)
- **Unchained commits:** all 9 preserved in `compiler/` and `src/tools/rustfmt/`

## Context

- **Project:** rust_unchained (Rust compiler fork)
- **Current HEAD:** `23499da8d16f` (fork master, 1 commit AHEAD of upstream official/master)
- **Upstream target:** `427288b3ce2d` (official/master, Rust 1.96.0-era)
- **Build goal:** `./x.py check` → `./x.py build stage2` → `./x.py test`
- **Verification criteria:** all tests pass + unchained commits preserved + toolchain works

## Doc map

| File | Owner | Status | Purpose |
| ---- | ----- | ------ | ------- |
| `01-repo-state-audit.md`     | subagent: repo-state       | ✅ done | Submodule drift, index vs .gitmodules vs .git/config, fix plan (302 lines) |
| `02-build-dependencies.md`   | subagent: build-deps       | ✅ done | x.py system requirements, 16 apt packages, LLVM strategy, disk/time estimates (401 lines) |
| `03-fork-changes-inventory.md` | subagent: fork-changes   | ✅ done | All unchained commits, orphan rule intent, rustfmt edits, merge-to-1.96 risks (247 lines) |
| `04-submodule-recovery.md`   | orchestrator               | ✅ done | Recovery execution log: 6 submodules reset, .gitmodules restored, decisions made |
| `05-build-verification.md`   | orchestrator               | ✅ done | Test results, all fixes applied, sandbox limitations, verification criteria status |
| `99-session-summary.md`      | orchestrator               | ✅ done | Final handoff: working-tree state, next steps, time spent |

## Quick start (reproduce the build)

```bash
cd /home/houtamelo/Documents/rust_unchained

# 1. Apply recovery (working-tree; restore .gitmodules + reset submodules)
git show 56b4b6063539:.gitmodules > .gitmodules
git -C library/stdarch        checkout 1245618ccf5b2df7ab1ebb0279b9f3f726670161
git -C src/doc/edition-guide  checkout 467f45637b73ec6aa70fb36bc3054bb50b8967ea
git -C src/doc/nomicon        checkout c76a20f0d987145dcedf05c5c073ce8d91f2e82a
git -C src/doc/reference      checkout 3bf3402aea982b876eb56c87da17b0685c6461d5
git -C src/gcc                checkout 0ea98a1365b81f7488073512c850e8ee951a4afd
git -C src/tools/cargo        checkout 7918c7eb59614c39f1c4e27e99d557720976bdd7

# 2. Build (build deps are installed via .claude-sandbox.deps.sh on every container start)
./x.py build --stage 2 --target x86_64-unknown-linux-gnu

# 3. Test
./x.py test --target x86_64-unknown-linux-gnu
```

**Important:** Always pass `--target x86_64-unknown-linux-gnu` to `x.py` on this sandbox. The default includes cross-compile to Windows MSVC which fails without the MSVC toolchain.

## Sandbox-specific infrastructure (persists across container resets)

- `.claude-sandbox.deps.sh` — installs build deps on every container creation.
- `~/.engram/` is bind-mounted (engram memory survives).
- The repo is bind-mounted (so all build artifacts persist).

## Next steps for the user

1. **Review the working-tree changes** with `git status` and `git diff`. Commit what you want to keep.
2. **Optionally revert** the bootstrap chmod patch (`src/bootstrap/src/lib.rs:2000-2012`) — it's a sandbox workaround, not a fork fix. Without it, only the compiletest sub-suites will run; non-compiletest tests are cut off by the bootstrap's install-step panic.
3. **When ready to advance to upstream 1.96.0**, the orphan-rule short-circuit at `compiler/rustc_hir_analysis/src/coherence/orphan.rs:40-45` will need translation into the new solver's `rustc_next_trait_solver/src/coherence.rs` semantics. See `03-fork-changes-inventory.md` §3.1.
4. **Rustfmt `let_else_style`** may conflict with an upstream option by the same name in 1.96. See `03-fork-changes-inventory.md` §3.2.
