# Fork changes inventory — orphan rules + rustfmt

**Repo:** `/home/houtamelo/Documents/rust_unchained`
**HEAD:** `23499da8d16f` (Rust 1.88 base)
**Commit author / fork:** Houtamelo `<antoniopedrogf@hotmail.com>`

## Executive summary

Every fork-specific code change in this repository is contained in **one single commit**:

| SHA | Date | Message |
| --- | --- | --- |
| `7ecc0046695057c0cf707444e97dcd258e0a2fa6` | 2025-02-20 | Disabling another subcase of orphan rules. |

That commit does two unrelated things bundled together:

1. **Disables** the `E0117` "local-before-uncovered" subcase of the orphan rule for downstream impls of foreign traits over foreign types. Implemented as a 6-line `match` in `compiler/rustc_hir_analysis/src/coherence/orphan.rs:40-45`. Hardcoded, no flag gates it.
2. **Adds a new rustfmt config option** `let_else_style` (`ElseOnSameLine` / `ElseOnNewLine`) so the user can force the `else` keyword of `let-else` blocks onto a new line. Default is `ElseOnSameLine` (matches upstream), so the change is a no-op for users who don't opt in.

All other Houtamelo commits in the history are meta (`README.md`, `version`, `config.toml` install paths, version bumps to 1.88, merge-of-upstream, and the unrelated `AsyncDrop` revert at HEAD).

The fork's `.stderr` files for `coherence-fundamental-trait-objects`, `coherence-impl-trait-for-marker-trait-{positive,negative}`, and `type-alias-impl-trait/coherence` have been updated to expect **build-pass** instead of an `E0117` error.

---

## 1. Orphan rule disabling

### Commit list

Only one commit affects this area: **`7ecc00466950`** (2025-02-20, Houtamelo). Verified by `git log --author="Houtamelo"` against the four candidate files; no other commits touch them.

### Per-commit deep dive

**Commit:** `7ecc0046695057c0cf707444e97dcd258e0a2fa6` — "Disabling another subcase of orphan rules."

#### File: `compiler/rustc_hir_analysis/src/coherence/orphan.rs`

- **Lines changed:** 40–45 (6 added, 1 removed).
- **What the diff does:**

```rust
// BEFORE
Err(err) => return Err(emit_orphan_check_error(tcx, trait_ref, impl_def_id, err)),

// AFTER
Err(err) => {
    match err {
        OrphanCheckErr::NonLocalInputType(_) => {}
        OrphanCheckErr::UncoveredTyParams(_) => return Err(emit_orphan_check_error(tcx, trait_ref, impl_def_id, err)),
    }
},
```

- **Plain-English intent:** When the orphan checker hits a `Compat`-mode failure, only emit an error if it's an `UncoveredTyParams` violation (the "E0210 / uncovered type parameter" case). If it's a `NonLocalInputType` (the "E0117 / no local type before any uncovered type parameter" case) **silently accept the impl**. This is the disabled subcase.
- **What the user can now compile that they couldn't before:**
  - `impl ForeignTrait for ForeignType<T> where T: SomeLocalTrait` style impls when the local type is only present as an uncovered type-parameter bound.
  - Specifically the test cases the fork moves from `compile-fail` to `build-pass`:
    - `impl Misc for dyn Fundamental<Local> {}` (`tests/ui/coherence/coherence-fundamental-trait-objects.rs:13`)
    - `impl foreign_crate::ForeignTrait for AliasOfForeignType<()> {}` (`tests/ui/type-alias-impl-trait/coherence.rs:18`)
    - `impl Send for dyn Marker2 {}` / `impl !Send for dyn Marker2 {}` in the marker-trait tests, where the local type appears only in `dyn LocalTrait` position.
- **No env-var / `-Z` flag / Cargo feature** gates the disabled behaviour. It's hardcoded; the only switch is "use this fork's rustc".

#### Files: `compiler/rustc_error_codes/src/error_codes/E011{6,7}.md`, `E0210.md`, `E0390.md`

- All four have a new first line `**This error is disabled in Unchained**`.
- The `compile_fail,E0xxx` annotation on the example code blocks was removed (or replaced with `ignore` for E0210, `compile_fail` for E0117) so rustdoc tests don't try to compile a snippet that now compiles.
- **Note:** E0116 (inherent impl on foreign type) and E0390 (inherent impl on primitive / reference) are also genuinely disabled, but the disablement is in a different code path: `compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs:64-99`. The fork's `check_def_id` and `check_primitive_impl` add non-local and primitive impls to the `incoherent_impls` map and return `Ok(())` instead of erroring. Empirically verified: `impl Vec<u8> { fn bar() {} }` and `impl *mut Foo {}` both compile cleanly under the fork's rustc (exit 0, no diagnostic). So the docs are accurate, not aspirational.

#### Test impact (orphan rule)

| Test file | Change |
| --- | --- |
| `tests/ui/coherence/coherence-fundamental-trait-objects.rs` | Added `//@ build-pass`; removed `//~^ ERROR E0117` annotation |
| `tests/ui/coherence/coherence-fundamental-trait-objects.stderr` | **Deleted** |
| `tests/ui/coherence/coherence-impl-trait-for-marker-trait-negative.rs` | `//~ ERROR E0117` → `//~ ERROR E0321` on `impl !Send for dyn Marker2 {}` |
| `tests/ui/coherence/coherence-impl-trait-for-marker-trait-negative.stderr` | `E0117` block replaced with `E0321` (cross-crate auto-trait on non-struct/enum) |
| `tests/ui/coherence/coherence-impl-trait-for-marker-trait-positive.rs` | `//~ ERROR E0117` → `//~ ERROR E0321` on `unsafe impl Send for dyn Marker2 {}` |
| `tests/ui/coherence/coherence-impl-trait-for-marker-trait-positive.stderr` | `E0117` block replaced with `E0321` |
| `tests/ui/type-alias-impl-trait/coherence.rs` | Added `//@ build-pass`; removed `//~^ ERROR only traits defined…` |
| `tests/ui/type-alias-impl-trait/coherence.classic.stderr` | **Deleted** |
| `tests/ui/type-alias-impl-trait/coherence.next.stderr` | **Deleted** |

After the change, those five sites still produce one error each, but it is now `E0321` (the secondary "cross-crate auto trait on non-struct/enum" check) instead of `E0117`.

### How to identify the disabled rule from a compiler error

If the user is hitting a forbidden impl that this fork was designed to allow, they will see **no error at all** in this fork's rustc. On upstream Rust ≥ 1.84, the same code produces:

```
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src/lib.rs:LL:CC
   |
LL | impl <ForeignTrait> for <ForeignType<T>> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                `<ForeignType<T>>` is not defined in the current crate
   |
   = note: impl doesn't have any local type before any uncovered type parameters
   = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
   = note: define and implement a trait or new type instead
```

If a user reports "E0117 with note 'impl doesn't have any local type before any uncovered type parameters'" on a `dyn LocalTrait`, type-alias-impl-trait, or marker-trait impl — that is exactly the subcase the fork silences.

---

## 2. rustfmt edits

### Commit list

Only one commit: **`7ecc00466950`** (2025-02-20, Houtamelo). No other fork commits modify anything under `src/tools/rustfmt/`.

### Per-commit deep dive

**Commit:** `7ecc0046695057c0cf707444e97dcd258e0a2fa6` — "Disabling another subcase of orphan rules." (rustfmt changes bundled in.)

The rustfmt addition is a new boolean-equivalent config option `let_else_style`. With the default `ElseOnSameLine`, behaviour is identical to upstream. Setting `let_else_style = "ElseOnNewLine"` (TOML / CLI / `rustfmt.toml`) puts the `else` keyword on its own line in `let-else` blocks and lets the divergent block live on one line even when it contains two statements (instead of one).

#### File: `src/tools/rustfmt/src/config/options.rs`

- **Lines changed:** 223–244 (new enum + `StyleEditionDefault` impl), 677 (config macro line).
- **What the diff does:**
  - Defines a new `#[config_type] pub enum LetElseStyle { ElseOnSameLine, ElseOnNewLine }`.
  - Implements `StyleEditionDefault` returning `ElseOnSameLine` for every style edition (so the option doesn't change across editions).
  - Registers `LetElseStyleConfig, LetElseStyle, _ => LetElseStyle::ElseOnSameLine` in the `config_option_with_style_edition_default!` macro table.
- **Plain-English intent:** Add a stable config knob that downstream users can put in `rustfmt.toml` to opt into a more conservative `let-else` layout.

#### File: `src/tools/rustfmt/src/config/mod.rs`

- **Lines changed:** 159–161 (3 added, 1 removed — also drops one blank line at 61).
- **What the diff does:** Declares the user-facing option:
  ```rust
  let_else_style: LetElseStyle, false, "In `let-else` statements, whether to put the `else`\
      keyword on the same line or the next one.";
  ```
  And the import is added via the same `crate::config::options` re-export path used for every other option.
- **Plain-English intent:** Same as above; this is the option-macro registration that wires `let_else_style` into `Config` and `--config=…` CLI parsing.

#### File: `src/tools/rustfmt/src/expr.rs`

- **Lines changed:** 17 (import), 1076 (new parameter), 1088–1109 (match arm logic).
- **What the diff does:**
  - Adds `use crate::config::LetElseStyle`.
  - Adds a third parameter `is_let_else: bool` to `pub(crate) fn rewrite_else_kw_with_comments`.
  - Hoists `control_brace_style()` into a local `brace_style` so it is reused.
  - Adds a new match arm in `before_sep`:
    ```rust
    ControlBraceStyle::AlwaysSameLine if is_let_else => {
        match context.config.let_else_style() {
            LetElseStyle::ElseOnSameLine => " ",
            LetElseStyle::ElseOnNewLine => newline_sep.as_ref(),
        }
    },
    ```
  - The one non-let-else caller (`rewrite_result` for chained `if/else if/else`) is updated to pass `false` for the new param at line 1192.
- **Plain-English intent:** When formatting a `let-else`, the choice between a single space and a newline before `else {` now also depends on the new option, not just `control_brace_style`.

#### File: `src/tools/rustfmt/src/items.rs`

- **Lines changed:** 18 (import), 174–194 (match arm logic + `is_let_else: true` arg), 261–281 (rewritten `allow_single_line_let_else_block`).
- **What the diff does:**
  - Adds `use crate::config::LetElseStyle`.
  - Passes `is_let_else = true` into the `rewrite_else_kw_with_comments` call inside `rewrite_result` for the `let-else` code path (around line 160 in the patched file).
  - Replaces the previous `style_edition` branch on `assign_str_with_else_kw` with:
    ```rust
    let assign_str_with_else_kw = match context.config.let_else_style() {
        LetElseStyle::ElseOnSameLine => {
            if context.config.style_edition() >= StyleEdition::Edition2024 {
                &result[let_kw_offset..]
            } else {
                result.as_str()
            }
        }
        LetElseStyle::ElseOnNewLine => else_kw.as_str(),
    };
    ```
    In `ElseOnNewLine` mode, `available_space` is computed against just the rewritten `else` keyword (since the divergent block sits on its own line), allowing single-line divergent blocks that are wider than they would be in `ElseOnSameLine` mode.
  - Rewrites `allow_single_line_let_else_block` to take `&RewriteContext` and pick the threshold by `let_else_style`:
    ```rust
    match context.config.let_else_style() {
        LetElseStyle::ElseOnSameLine => { /* unchanged: <= 1 stmt */ }
        LetElseStyle::ElseOnNewLine  => { /* <= 2 stmts */ }
    }
    ```
- **Plain-English intent:** When the user picks `ElseOnNewLine`, more `let-else` blocks fit on one line and slightly more divergent blocks (up to 2 statements) are kept on one line.

#### File: `src/tools/rustfmt/Cargo.lock`

- **Lines changed:** 1 added — `serde` added to the dependency list for one of the proc-macro crates (`unicode-width 0.3.0` → no, actually `proc-macro2 / quote / serde / syn`). Reading the diff literally: `serde` is added to one entry's `dependencies = [...]` array. This is a Cargo.lock regeneration side-effect of the `serde::{Deserialize, Serialize, Serializer}` imports that were already used in `options.rs` line 12; the import was already there pre-patch, so this is a stale Cargo.lock sync the patch happened to clean up. Not a behavioural change.

### Test impact (rustfmt)

The commit adds **zero new tests** under `src/tools/rustfmt/tests/`. No `let-else` source/target fixtures were added, and no `Config::let_else_style` unit test was added. This is a known gap:

- The option exists in code but is exercised only when a downstream user writes `let_else_style = "ElseOnNewLine"` in their `rustfmt.toml`.
- After the upstream 1.84 → 1.88 → 1.96 merges, the test suite has no regression coverage for either branch of the new option.

When porting forward, the preservation checklist below requires either adding two new fixture pairs under `tests/source/configs/let_else_style/else_on_new_line/` (and `…/else_on_same_line/`) or accepting that this option is shipped untested.

---

## 3. Preservation checklist (must-not-change when merging upstream 1.96.0)

These are the items that, if lost, would defeat the purpose of the fork:

- [ ] **`compiler/rustc_hir_analysis/src/coherence/orphan.rs` lines 40–45** — the `match` that silently accepts `OrphanCheckErr::NonLocalInputType`. The match-arm style and the parameterless `Ok(())` outer branches upstream may change, so re-implement the same semantic suppression rather than copy-pasting the exact 6 lines.
- [ ] **Error-code docs:** the first line `**This error is disabled in Unchained**` on `E0116.md`, `E0117.md`, `E0210.md`, `E0390.md`. (Decide: keep the E0116 / E0390 banners, or remove them since the compiler-side disabling isn't done; see risks below.)
- [ ] **Test files marked build-pass:**
  - `tests/ui/coherence/coherence-fundamental-trait-objects.rs` (`//@ build-pass`)
  - `tests/ui/type-alias-impl-trait/coherence.rs` (`//@ build-pass`)
  - The `E0117 → E0321` re-pointing in `tests/ui/coherence/coherence-impl-trait-for-marker-trait-{positive,negative}.rs` and their `.stderr`.
  - The deleted `.stderr` files for the three sites above must stay deleted (regenerate with `bless` if any upstream test infra requires them).
- [ ] **rustfmt option `let_else_style`** in:
  - `src/tools/rustfmt/src/config/options.rs:223–244` and `:677`
  - `src/tools/rustfmt/src/config/mod.rs:159–161`
  - `src/tools/rustfmt/src/expr.rs:17, 1076, 1088–1109, 1192`
  - `src/tools/rustfmt/src/items.rs:18, 160–163, 174–194, 261–281`
- [ ] **`else_kw.as_str()` width calculation** in `items.rs` for `ElseOnNewLine` — the available-width semantics, not just the literal code.
- [ ] **`allow_single_line_let_else_block` `<= 2 stmts`** rule for `ElseOnNewLine` — the magic-number is part of the feature's intent.

## 4. Merge-to-1.96 risks (2–3 bullets)

1. **`orphan.rs` API churn in the new solver.** The expression `OrphanCheckErr::NonLocalInputType(_)` and the `orphan_check_impl` two-stage `Proper` → `Compat` flow was the shape in 1.84–1.88. The 1.96 `-Znext-solver=coherence` rewrite (already partially in 1.88) refactors `OrphanCheckErr` and may rename / split the variants. The short-circuit may need to move into a new `NextSolver`-only branch or the new `coherence::orphan_check_next` function; preserving intent requires re-reading `rustc_next_trait_solver/src/coherence.rs` on every merge and translating the same behaviour into the new module, not blindly re-applying the 6-line patch.
2. **`control_brace_style` enum may absorb `let_else_style`.** Upstream may have already (or may in 1.96) added its own let-else style flag with different naming (`brace_style` / `ControlBraceStyle` variant, etc.) — there is non-zero chance the rustfmt addition here conflicts with an upstream feature by 1.96. The preservation strategy is: keep the option but allow it to delegate to the upstream mechanism if the semantics match, or rename `LetElseStyle` to a fork-prefixed name (e.g. `unchained_let_else_style`) to avoid silent override.
3. **The E0116 / E0390 doc banners are accurate** — those errors ARE disabled in the fork, just via a different code path than E0117 (see note above about `inherent_impls.rs`). The 1.96 merge must preserve the `inherent_impls.rs` change in addition to the `orphan.rs` change.

---

## Appendix: how this was derived

```bash
# Author discovery
git log --all --oneline --author="Houtamelo"
# → exactly one commit (7ecc00466950) touches compiler/rustc_hir_analysis or src/tools/rustfmt/src/

# Per-file blame for line ranges
git blame -L 38,55 compiler/rustc_hir_analysis/src/coherence/orphan.rs
git blame -L 218,250 src/tools/rustfmt/src/config/options.rs
git blame -L 670,685 src/tools/rustfmt/src/config/options.rs

# Diff context
git show 7ecc00466950 -- <path>

# Upstream sanity check (proves LetElseStyle is NOT in upstream)
git show official/master:src/tools/rustfmt/src/expr.rs | grep -c let_else_style
# → 0
```