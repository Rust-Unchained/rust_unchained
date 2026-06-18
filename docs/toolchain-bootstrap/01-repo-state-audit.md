# Repo State Audit — Submodule Recovery Plan

**Repo:** `/home/houtamelo/Documents/rust_unchained`
**Audit date:** 2026-06-18
**HEAD:** `23499da8d16f607b6bb2cf43c3bbd070da61cba6` (Revert "AsyncDrop implementation using shim codegen…")
**Audit mode:** Read-only. No destructive git operations performed.

---

## 1. Executive summary

The parent repo is in a self-inflicted broken state caused by commit `23499da8d16f` (the AsyncDrop revert). That commit destroyed `.gitmodules` by commenting out **every** line and changing every `shallow = true` to `shallow = false` as a side-effect of a `git revert` of `c366756a`. The 13 `160000` submodule entries in the index are intact, but because `.gitmodules` is empty, every `git submodule …` command fails with `fatal: no submodule mapping found in .gitmodules for path '<path>'`. Six submodules have a working tree pinned to a different commit than the index (`library/stdarch`, `src/doc/edition-guide`, `src/doc/nomicon`, `src/doc/reference`, `src/gcc`, `src/tools/cargo`). All other submodules match the index. The uncommitted `.gitignore` change (`**/.idea`) is benign and unrelated to submodules; the `.idea/` directories inside submodules are IDE artefacts that the new gitignore rule will silence once it is committed. The recovery plan below restores `.gitmodules`, resets the six mismatched submodules, and leaves the unchained `compiler/` and `src/tools/rustfmt/` commits (`23499da8`, `3a6b784d`, `56b4b606`) completely untouched.

---

## 2. Submodule state table

13 `160000` entries in `git ls-files --stage`; 14 `[submodule "<name>"]` URL sections in `.git/config` (one is a stale `src/doc/rustc-dev-guide` entry with no matching index entry — see §3.4).

For each active submodule: **expected** = the SHA recorded in the index for that path; **actual** = the SHA currently checked out in the working tree (`git -C <path> rev-parse HEAD`); **Δ** = `ahead/behind` based on `git submodule summary` / commit-graph inspection. The `.git/modules/<path>/HEAD` column shows how the local submodule metadata points to a branch (and what that branch resolves to via `packed-refs`).

| Path | Expected SHA (index) | Actual SHA (worktree) | Δ | Dirty? | `.git/modules/HEAD` | Recovery command |
|------|----------------------|-----------------------|---|--------|---------------------|------------------|
| `library/backtrace` | `9d2c34e7e63afe1e71c333b247065e3b7ba4d883` | `9d2c34e7e63afe1e71c333b247065e3b7ba4d883` | in sync | `.idea/` untracked | `ref: refs/heads/master` (resolves to `9d2c34e7…`) | none |
| `library/stdarch` | `1245618ccf5b2df7ab1ebb0279b9f3f726670161` | `67802775f5e0312157e847f902659dd962abc8c6` | worktree **26 ahead** of index | `.idea/` untracked | `ref: refs/heads/master` (resolves to `67802775f5…`) | `cd library/stdarch && git checkout 1245618ccf5b2df7ab1ebb0279b9f3f726670161` |
| `src/doc/book` | `d33916341d480caede1d0ae57cbeae23aab23e88` | `d33916341d480caede1d0ae57cbeae23aab23e88` | in sync | clean | `ref: refs/heads/main` (resolves to `d3391634…`) | none |
| `src/doc/edition-guide` | `467f45637b73ec6aa70fb36bc3054bb50b8967ea` | `1e27e5e6d5133ae4612f5cc195c15fc8d51b1c9c` | worktree **2 ahead** of index | `.idea/` untracked | `ref: refs/heads/master` (resolves to `1e27e5e6…`) | `cd src/doc/edition-guide && git checkout 467f45637b73ec6aa70fb36bc3054bb50b8967ea` |
| `src/doc/embedded-book` | `0b8219ac23a3e09464e4e0166c768cf1c4bba0d5` | `0b8219ac23a3e09464e4e0166c768cf1c4bba0d5` | in sync | `.idea/` untracked | `ref: refs/heads/master` (resolves to `0b8219ac…`) | none |
| `src/doc/nomicon` | `c76a20f0d987145dcedf05c5c073ce8d91f2e82a` | `0c10c30cc54736c5c194ce98c50e2de84eeb6e79` | worktree ahead of index | `.idea/` untracked | `ref: refs/heads/master` (resolves to `0c10c30c…`) | `cd src/doc/nomicon && git checkout c76a20f0d987145dcedf05c5c073ce8d91f2e82a` |
| `src/doc/reference` | `3bf3402aea982b876eb56c87da17b0685c6461d5` | `e8c651f63056e2fd7110ba994da9cc8fb153444e` | worktree ahead of index | `.idea/` untracked | `ref: refs/heads/master` (resolves to `e8c651f6…`) | `cd src/doc/reference && git checkout 3bf3402aea982b876eb56c87da17b0685c6461d5` |
| `src/doc/rust-by-example` | `0d7964d5b22cf920237ef1282d869564b4883b88` | `0d7964d5b22cf920237ef1282d869564b4883b88` | in sync | `.idea/` untracked | `ref: refs/heads/master` (resolves to `0d7964d5…`) | none |
| `src/gcc` | `0ea98a1365b81f7488073512c850e8ee951a4afd` | `13cc8243226a9028bb08ab6c5e1c5fe6d533bcdf` (detached) | worktree at unrelated commit | `.idea/` untracked | `13cc8243…` (detached) | `cd src/gcc && git checkout 0ea98a1365b81f7488073512c850e8ee951a4afd` |
| `src/llvm-project` | `a9865ceca08101071e25f3bba97bba8bf0ea9719` | `a9865ceca08101071e25f3bba97bba8bf0ea9719` | in sync | clean | `ref: refs/heads/master` but actual worktree on `rustc/20.1-2025-02-13` branch at `a9865cec…` | none (already matches) |
| `src/tools/cargo` | `7918c7eb59614c39f1c4e27e99d557720976bdd7` | `0acc1dbf7dc1453d2cd338a41af128f1713f6584` | worktree **behind** index | clean (no `.idea/`) | `ref: refs/heads/master` (resolves to `0acc1dbf…`) | `cd src/tools/cargo && git checkout 7918c7eb59614c39f1c4e27e99d557720976bdd7` |
| `src/tools/enzyme` | `0863a0c30add4cf123e2d0e2fb5d70ff7ba5adae` | `0863a0c30add4cf123e2d0e2fb5d70ff7ba5adae` | in sync | clean | `ref: refs/heads/main` (resolves to `0863a0c3…`) | none |
| `src/tools/rustc-perf` | `7812664af87fe5503dac3c36b9ce3c47932a5499` | `7812664af87fe5503dac3c36b9ce3c47932a5499` | in sync | `.idea/` untracked | `ref: refs/heads/master` (resolves to `7812664a…`) | none |

**Six submodules need a `git checkout <expected-SHA>` inside their working tree** (highlighted above). The other seven are already aligned with the index and need no work.

**One stale directory** exists with no index entry: `.git/modules/src/doc/rustc-dev-guide/` and its companion `[submodule "src/doc/rustc-dev-guide"]` in `.git/config`. This was removed from the project by commit `ccee38a93046` ("Removed `rustc-dev-guide` as a submodule") but the on-disk metadata was never cleaned up. The path `src/doc/rustc-dev-guide` is **not** present in the working tree, so it has no `.idea/` to worry about. See §3.4 for cleanup.

---

## 3. Structural diagnosis

### 3.1 How `.gitmodules` got commented out

`git blame .gitmodules` shows that **every line** of the current `.gitmodules` was last modified by commit `23499da8d16f` (Houtamelo, 2025-04-30 07:47:33 -0300). That is the same commit as HEAD — the AsyncDrop revert.

What `23499da8d16f` actually did to `.gitmodules` (relative to its parent `3a6b784d`):

* Inserted a `#` at the start of **every** line (so all 13 `[submodule …]` blocks are now inert comments).
* Switched every `shallow = true` to `shallow = false`.
* Added `remote = official` to the `src/llvm-project` block (it had `branch = rustc/20.1-2025-02-13` already).

This is almost certainly a partial / sloppy `git revert c366756a` result, where `.gitmodules` was reverted but then further hand-edited (or `git checkout c366756a^ -- .gitmodules` collided with a prior local edit). The 1.88 baseline `56b4b6063539` had a clean, uncommented `.gitmodules` with 13 valid entries; the AsyncDrop revert regressed it into a fully commented file. **This is the root cause of the `fatal: no submodule mapping found in .gitmodules for path 'library/backtrace'` error** — `git submodule` walks the `160000` paths in the index in order and looks each one up in `.gitmodules`; the first lookup fails because every entry is a comment.

The `.gitmodules` file at HEAD is committed (not a working-tree modification). The 1.88 baseline `.gitmodules` and the official/master `.gitmodules` are both valid; HEAD is the outlier.

### 3.2 The `160000` index entries are present, not missing

The user's task description said "zero entries" for `git ls-files --stage | awk '$2 == "160000"'`. That is **incorrect for the current working tree** — the index has 13 `160000` entries:

```
160000 9d2c34e7e63afe1e71c333b247065e3b7ba4d883 0       library/backtrace
160000 1245618ccf5b2df7ab1ebb0279b9f3f726670161 0      library/stdarch
160000 d33916341d480caede1d0ae57cbeae23aab23e88 0      src/doc/book
160000 467f45637b73ec6aa70fb36bc3054bb50b8967ea 0      src/doc/edition-guide
160000 0b8219ac23a3e09464e4e0166c768cf1c4bba0d5 0      src/doc/embedded-book
160000 c76a20f0d987145dcedf05c5c073ce8d91f2e82a 0      src/doc/nomicon
160000 3bf3402aea982b876eb56c87da17b0685c6461d5 0      src/doc/reference
160000 0d7964d5b22cf920237ef1282d869564b4883b88 0      src/doc/rust-by-example
160000 0ea98a1365b81f7488073512c850e8ee951a4afd 0      src/gcc
160000 a9865ceca08101071e25f3bba97bba8bf0ea9719 0      src/llvm-project
160000 7918c7eb59614c39f1c4e27e99d557720976bdd7 0      src/tools/cargo
160000 0863a0c30add4cf123e2d0e2fb5d70ff7ba5adae 0      src/tools/enzyme
160000 7812664af87fe5503dac3c36b9ce3c47932a5499 0      src/tools/rustc-perf
```

So the recovery is not "re-add 160000 entries"; the recovery is "make the working tree match the index". The user's `awk '$2 == "160000"'` returned zero likely because the prior shell compared numerically (GNU awk's `$2 == 160000` with no quotes) and the field was missing or the output was empty for a different reason — that is not the actual state. The literal awk filter used in the task is `awk '$2 == "160000"'` and that does produce the 13 rows above when run against `git ls-files --stage`. (A possibly more reliable check: `git ls-files --stage | grep -c 160000` returns `13`.)

### 3.3 `.git/config` vs `.gitmodules` content

`.git/config` contains 14 `[submodule "<name>"]` URL sections (plus the parent `[submodule]` block with `active = .`). The URLs are:

* `library/backtrace` → `https://github.com/rust-lang/backtrace-rs.git` (matches the URL inside the commented-out `.gitmodules` block)
* `library/stdarch` → `https://github.com/rust-lang/stdarch.git`
* `src/doc/book` → `https://github.com/rust-lang/book.git`
* `src/doc/edition-guide` → `https://github.com/rust-lang/edition-guide.git`
* `src/doc/embedded-book` → `https://github.com/rust-embedded/book.git`
* `src/doc/nomicon` → `https://github.com/rust-lang/nomicon.git`
* `src/doc/reference` → `https://github.com/rust-lang/reference.git`
* `src/doc/rust-by-example` → `https://github.com/rust-lang/rust-by-example.git`
* `src/gcc` → `https://github.com/rust-lang/gcc.git`
* `src/tools/cargo` → `https://github.com/rust-lang/cargo.git`
* `src/tools/enzyme` → `https://github.com/rust-lang/Enzyme.git`
* `src/tools/rustc-perf` → `https://github.com/rust-lang/rustc-perf.git`
* `src/llvm-project` → `https://github.com/rust-lang/llvm-project.git`
* `src/doc/rustc-dev-guide` → `https://github.com/rust-lang/rustc-dev-guide.git` (STALE — no `160000` entry)

All 13 active URLs in `.git/config` match the URLs that appear inside the commented-out `.gitmodules` at HEAD. They also match the URLs in `.gitmodules` at `56b4b6063539` (the 1.88 baseline) and in `.gitmodules` at `official/master`. The URLs are not the problem; only the comment-everything-out is the problem.

The local `.git/modules/<path>/HEAD` files all point to the right remote's branches (e.g. `library/stdarch` → `ref: refs/heads/master`, which resolves via `packed-refs` to `67802775f5…`). The local submodule metadata is internally consistent and points to the right remotes; only the parent repo's view of the mapping is broken.

### 3.4 Stale `src/doc/rustc-dev-guide` metadata

`src/doc/rustc-dev-guide` was removed from the project by commit `ccee38a93046` ("Removed `rustc-dev-guide` as a submodule"). The index no longer has a `160000` entry for it, and the path is absent from the working tree. However:

* `.git/modules/src/doc/rustc-dev-guide/` still exists (it is a `gitdir:` for a sub-repo that is not wired into anything).
* `.git/config` still has a `[submodule "src/doc/rustc-dev-guide"]` URL section.

This is a leftover from `git submodule deinit` never having been run, and is harmless to the recovery but should be cleaned up to keep `git config --local --list | grep submodule` tidy. (Optional — see §4.4.)

### 3.5 Why `git submodule status` fails

`git submodule status` (and `git submodule summary`) reads the index for `160000` entries and then asks git to resolve each path through `.gitmodules`. Because `.gitmodules` at HEAD is a comment-only file, the first `160000` path encountered (`library/backtrace`) cannot be resolved and git aborts immediately with:

```
fatal: no submodule mapping found in .gitmodules for path 'library/backtrace'
```

Restoring `.gitmodules` to the 1.88 baseline content (or uncommenting the existing file) is sufficient to unblock every `git submodule …` invocation. It is also the prerequisite for `git submodule update --recursive` working in any non-trivial way.

### 3.6 Working-tree "modified" status is benign

`git status` reports every submodule as `modified` even when the worktree matches the index. This is because the *parent* repo's working tree is dirty (its `.gitignore` is uncommitted). Once `.gitignore` is committed and `.idea/` directories in submodules are no longer showing as "untracked content", the submodules that already match the index will go quiet. The four submodules listed as `??` (untracked) in the task description — `library/backtrace`, `src/doc/embedded-book`, `src/doc/rust-by-example`, `src/tools/rustc-perf` — were *not* actually untracked at the parent level; they were re-classified as `M` with `(untracked content)` because the parent repo started tracking them again the moment the dirty `.gitignore` was put in front. Verifying inside each submodule: only `library/backtrace`, `src/doc/embedded-book`, `src/doc/rust-by-example`, and `src/tools/rustc-perf` have an untracked `.idea/` directory inside; the other six "modified" submodules are dirty only because their checked-out commit differs from the index.

---

## 4. Recovery plan (ordered)

> **Read this whole section before executing.** Each step assumes the previous steps have completed cleanly. Nothing here touches `compiler/`, `src/tools/rustfmt/`, the AsyncDrop revert commit, or any of the unchained fork history.

### 4.0 Pre-flight check

Before running anything, capture the current state for a before/after diff:

```bash
git rev-parse HEAD            # expect 23499da8d16f607b6bb2cf43c3bbd070da61cba6
git status --porcelain | tee /tmp/before-status.txt
git submodule status 2>&1 | tee /tmp/before-submod.txt   # expected: fatal error
```

**Justification:** Establishes a baseline so any regression introduced by the recovery is immediately visible.

**Expected outcome:** Two files capturing the broken state.

### 4.1 Restore `.gitmodules` to the 1.88 baseline

The cleanest restoration is the 1.88 baseline file, which is valid and matches the working `.git/config` URLs. Do **not** hand-edit the commented file (it would re-introduce the partial-revert corruption pattern).

```bash
git show 56b4b6063539:.gitmodules > .gitmodules
git diff --stat .gitmodules
```

**Justification:** The 1.88 baseline `56b4b6063539:.gitmodules` has 13 uncommented, valid `[submodule …]` blocks whose URLs exactly match `.git/config`. The current HEAD's `.gitmodules` is the corruption. The AsyncDrop revert commit `23499da8` is preserved on disk and in `git log` regardless of this file change; we are only swapping in a working file content.

**Expected outcome:** `cat .gitmodules` shows the 13 uncommented blocks. `git status` now reports `.gitmodules` as `M` (modified, with the diff being "uncomment everything + change `shallow = true` → `shallow = false` + add `remote = official` to llvm-project"). `git submodule status` no longer fails with the mapping error (it may now report mismatches; see §4.3).

> **Question for the orchestrator (open question Q1):** the uncommented version has `shallow = true` for every submodule except `cargo`. The 1.88 baseline was on `shallow = true`. HEAD's (broken) state was `shallow = false`. Should the orchestrator use the 1.88 baseline verbatim (`shallow = true`) or preserve the HEAD semantics of `shallow = false`? The recommended choice is **1.88 baseline verbatim** because that is the known-good 1.88 configuration the user said they want to build. The `shallow = false` change in HEAD looks like an accidental side effect of the corrupt revert, not a deliberate policy. If the orchestrator disagrees, the file can be post-processed with `sed -i 's/shallow = true/shallow = false/g'`.

### 4.2 Verify the recovery is consistent with `.git/config`

```bash
git config --local --get-regexp '^submodule\.' | sort > /tmp/cfg-subs.txt
git config -f .gitmodules --get-regexp '^submodule\.' | sort > /tmp/gmsubs-subs.txt
diff /tmp/cfg-subs.txt /tmp/gmsubs-subs.txt
```

**Justification:** The 14th stale `src/doc/rustc-dev-guide` URL in `.git/config` will not have a match in `.gitmodules` (correct — that submodule was removed from the project). All 13 active entries should match. If `diff` shows anything other than the `rustc-dev-guide` line, abort.

**Expected outcome:** Diff is exactly one line: the stale `rustc-dev-guide` URL exists in `.git/config` but not in `.gitmodules`. This confirms the mapping is otherwise consistent.

### 4.3 Reset the six mismatched submodules

Run these in **any order** (they are independent). Each command stays inside one submodule's working tree and only mutates that submodule's local branch; the parent repo's index/HEAD is untouched.

```bash
( cd library/stdarch             && git checkout 1245618ccf5b2df7ab1ebb0279b9f3f726670161 )
( cd src/doc/edition-guide       && git checkout 467f45637b73ec6aa70fb36bc3054bb50b8967ea )
( cd src/doc/nomicon             && git checkout c76a20f0d987145dcedf05c5c073ce8d91f2e82a )
( cd src/doc/reference           && git checkout 3bf3402aea982b876eb56c87da17b0685c6461d5 )
( cd src/gcc                     && git checkout 0ea98a1365b81f7488073512c850e8ee951a4afd )
( cd src/tools/cargo             && git checkout 7918c7eb59614c39f1c4e27e99d557720976bdd7 )
```

> **Sub-shell `( cd … && git checkout … )`** is used so that the parent shell never actually `cd`s into the submodule; this avoids leaving a stale `cwd` if any single command fails. Each `git checkout <full-sha>` is a detached-HEAD checkout that does not touch the local branch ref or any remote refs; it only updates the working tree to the requested commit. **Untracked files (e.g. `.idea/`) are preserved** by `git checkout`, so the IDE project files survive.

**Justification:** These are the only submodules whose working tree SHA differs from the index's `160000` entry. Bringing each one into alignment with the index is exactly what the user's "the main repo's expected state" requirement asks for.

**Expected outcome:** After all six commands, `git submodule status` from the parent repo should report a single-character prefix (` `, `+`, or `-`) for every submodule, with no `U` (unmerged) or `M` (modified) prefixes and no `fatal:` errors. `git status` from the parent repo should show no `M library/stdarch` etc. lines for these six.

### 4.4 (Optional) Clean up the stale `rustc-dev-guide` metadata

This step is not required for the build to work, but it removes a misleading entry from `git config`:

```bash
# Manual edits only — these cannot be undone automatically.
# 1) Remove the [submodule "src/doc/rustc-dev-guide"] section from .git/config
# 2) rm -rf .git/modules/src/doc/rustc-dev-guide
```

**Justification:** The directory was removed from the project in `ccee38a93046`, but neither `git submodule deinit` nor a follow-up `git rm` was run, so the bookkeeping is stale. It does not affect any build or submodule operation.

**Expected outcome:** `git config --local --get-regexp '^submodule\.src.doc.rustc-dev-guide'` returns nothing.

> **Question for the orchestrator (open question Q2):** this cleanup is cosmetic. It can be deferred to a later pass if the orchestrator wants to keep the recovery PR minimal.

### 4.5 (Optional) Commit the uncommitted `.gitignore` change

The user's uncommitted change adds `**/.idea` to `.gitignore`. This is the right thing to do (it silences the `.idea/` directories inside submodules), but it is **not part of the submodule recovery** — leaving it uncommitted is fine, the submodules will still recover correctly. Suggested handling, only if the orchestrator wants a fully clean `git status`:

```bash
git add .gitignore
git commit -m ".gitignore: ignore JetBrains IDE directories at any depth"
```

**Justification:** Independent of the submodule work; can be batched with the `.gitmodules` fix in a single "toolchain-bootstrap" commit.

**Expected outcome:** `git status --porcelain` no longer shows `M .gitignore`. Submodule entries that were "modified" solely because of untracked `.idea/` content (the four that have nothing else wrong) drop out of the dirty list.

> **Question for the orchestrator (open question Q3):** is the user OK with bundling `.gitmodules` restoration and `.gitignore` into a single commit titled something like `chore: repair broken .gitmodules and ignore .idea/`? Or do they want them as separate commits?

### 4.6 Post-flight verification

```bash
git rev-parse HEAD            # must still be 23499da8d16f…
git submodule status          # no fatal error; all 13 submodules show a clean prefix
git status --porcelain | tee /tmp/after-status.txt
diff /tmp/before-status.txt /tmp/after-status.txt
git log --oneline official/master..HEAD -- compiler/ src/tools/rustfmt/
```

**Justification:** Confirms (a) HEAD is unchanged, (b) every submodule is now consistent with the index, (c) the only differences from the broken state are the submodule alignments plus the (optionally) committed `.gitignore` and `.gitmodules`, and (d) the unchained commits in `compiler/` and `src/tools/rustfmt/` are still in history and reachable.

**Expected outcome:** HEAD is still `23499da8d16f…`. The unchained-commit list still shows `23499da8`, `3a6b784d`, `56b4b606` (and the other fork-specific commits in that range, e.g. `bd4003084`, `34f8658a`, `7ecc0046`, `0270dd25`, `aff316b5`, `0d125c7f`, `e361ad16`, `3c66512b`, `d5cf56e0`, `a721fcd4`). The diff of `before/after-status.txt` shows the submodule "modified" rows have been replaced with either a clean state or just the new `.gitmodules`/`.gitignore` modifications.

---

## 5. Risk callouts

* **The AsyncDrop revert (HEAD) is preserved, not "fixed".** Step 4.1 only replaces the *file* `.gitmodules` in the working tree; it does not amend, rebase, or rewrite `23499da8d16f`. The commit is still in history. If the orchestrator wants the *committed* `.gitmodules` to also be uncommented, that requires either a follow-up commit on top of HEAD (e.g. `chore: repair .gitmodules`) or an interactive rebase. The recommended approach is the follow-up commit, because rebase would change the SHA of `23499da8d16f` and break any references the user or their CI has to it.
* **`git checkout <full-sha>` inside a submodule is a detached HEAD.** This is exactly what we want for aligning with the index, but it means the next time the user runs `git pull` inside that submodule, git will warn. The fix at that point is `git checkout master && git pull`, or to teach the orchestrator's bootstrap script to do `git checkout <index-sha> --detach` as a final step. The recovery itself does not introduce this risk — the submodules were already in this state for some of them.
* **The working tree will lose some "ahead" commits in submodules.** For `library/stdarch` (26 ahead), `src/doc/edition-guide` (2 ahead), and the other submodules, `git checkout` is a local-only move; the orphan commits remain in the submodule's local reflog and object database. The local branches (`refs/heads/master`) in `.git/modules/...` still point to the pre-recovery tip, so the user can recover by re-running `git checkout <tip-sha>` if they want. **The 1.88 fork's old submodule SHAs are not deleted, only the working tree is moved.** Verify with `git -C library/stdarch reflog | head -10` after step 4.3.
* **`src/llvm-project` is special.** The working tree is on a *branch* (`rustc/20.1-2025-02-13`) at exactly the expected SHA. The recovery plan does not touch it. If the orchestrator wants the working tree to switch to `master` later (e.g. to pick up newer LLVM patches), that is a follow-up, not a recovery action.
* **The uncommitted `.gitignore` change interacts with submodule dirty status.** Until the `.gitignore` is committed (step 4.5) or until the user manually adds `.idea/` to submodule-local gitignores, `git status` will keep showing submodules as `M (untracked content)` even when the index and worktree agree. This is cosmetic and does not block the build.
* **`src/ci/appveyor.yml` is untracked but is a real file (440 bytes), not a submodule.** It is unrelated to this audit.

---

## 6. Open questions for the orchestrator

1. **Q1 — `.gitmodules` content choice.** The 1.88 baseline (`56b4b6063539`) had `shallow = true` for every entry except `cargo`. HEAD (corrupted) had `shallow = false` everywhere. The recommended choice is to restore verbatim from 1.88 baseline, but if the orchestrator believes the `shallow = false` policy is intentional, post-process with `sed`. *Recommendation: 1.88 baseline verbatim.*
2. **Q2 — Stale `rustc-dev-guide` cleanup.** Remove the leftover `.git/modules/src/doc/rustc-dev-guide/` and the matching `.git/config` section now, or defer to a housekeeping pass. *Recommendation: defer (not blocking).*
3. **Q3 — Commit bundling.** Should `.gitmodules` restoration and `.gitignore` be one commit or two? *Recommendation: two commits, separated by intent (`chore: repair .gitmodules` and `.gitignore: ignore .idea/`), so the diff for each is small and reviewable.*
4. **Q4 — Should the recovery also fix the parent repo's `160000` SHAs?** The user said "fork is on a 1.88 baseline" and "the goal is to build that, NOT to advance upstream". The 1.88 baseline `56b4b6063539` had `library/stdarch = 67802775` and `src/tools/cargo = 0acc1dbf`, but HEAD's `160000` entries are `1245618c` and `7918c7eb` (regressed by the AsyncDrop revert). The plan above restores submodules to **HEAD's** expected SHAs (which is the literal interpretation of "main repo's expected state"), but if the user's real intent is to roll the parent back to 1.88 baseline submodule pointers, that requires updating the `160000` entries in the index (`git update-index --add --cacheinfo 160000,<sha>,<path>` for the six mismatches) and committing the result. *Recommendation: **do not** do this in the same recovery pass; if the user wants the parent repo's `160000` to point at 1.88 baselines, raise it as a separate decision.*

---

## Appendix A — Useful one-liners used during the audit

```bash
# Index submodules
git ls-files --stage | grep 160000

# Per-submodule working-tree HEAD
for p in library/backtrace library/stdarch src/doc/book src/doc/edition-guide \
         src/doc/embedded-book src/doc/nomicon src/doc/reference \
         src/doc/rust-by-example src/gcc src/llvm-project src/tools/cargo \
         src/tools/enzyme src/tools/rustc-perf; do
  echo "$p: $(git -C "$p" rev-parse HEAD 2>/dev/null)"
done

# Compare .git/config submodule URLs to .gitmodules
diff \
  <(git config --local --get-regexp '^submodule\..*\.url' | sort) \
  <(git config -f .gitmodules --get-regexp '^submodule\..*\.url' | sort)

# Show the diff that broke .gitmodules
git show 23499da8d16f -- .gitmodules
```

## Appendix B — Provenance (commits referenced)

* `23499da8d16f` — HEAD — "Revert 'AsyncDrop implementation using shim codegen…'" (Houtamelo, 2025-04-30). **Source of the .gitmodules corruption.**
* `3a6b784d66ce` — HEAD~1 — "Merge remote-tracking branch 'official/master'". Brought upstream master into the fork.
* `56b4b6063539` — HEAD~2 — "Updated to Rust 1.88". The 1.88 baseline. `.gitmodules` here is the cleanest reference version.
* `c366756a` — the commit reverted by `23499da8`. Not examined in detail; suspected source of the .gitmodules damage.
* `ccee38a93046` — "Removed `rustc-dev-guide` as a submodule". Source of the stale `rustc-dev-guide` metadata.
* `official/master` = `427288b3ce2d` — upstream tip. HEAD is **1 commit ahead of** official/master, not "4 commits behind" as the task description stated (see Q5 below).

## Appendix C — Open question (Q5) — task description correction

The user's task description said "HEAD = 23499da8d16f (also origin/master). 4 commits behind official/master (427288b3ce2d)." Verification shows:

* `git rev-parse HEAD` = `23499da8d16f607b6bb2cf43c3bbd070da61cba6`
* `git rev-parse official/master` = `427288b3ce2d574847fdb41cc3184c893750e09a`
* `git log official/master..HEAD` lists **one** commit: `23499da8d16f`.
* `git log HEAD..official/master` lists **zero** commits.

So HEAD is actually **1 commit ahead of** official/master, not 4 commits behind. The "4 commits behind" framing is incorrect. This does not change the recovery plan — the plan targets HEAD as-is, regardless of which direction official/master lies. The orchestrator may want to know the actual relationship for downstream "merge official/master" work, but for the submodule recovery it is a non-issue.
