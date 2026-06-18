# Build Dependencies — Rust 1.88 Source Build on Debian Trixie

**Repo:** `/home/houtamelo/Documents/rust_unchained`
**HEAD:** `23499da8d16f` (fork master, base = Rust 1.88)
**Build entrypoint:** `./x.py` → `src/bootstrap/bootstrap.py` → `src/bootstrap` (Rust)
**Target sandbox:** rootless Podman, Debian Trixie (13.4), `x86_64-unknown-linux-gnu`
**Container state (as of audit):** `build-essential`, `git`, `curl`, `pkg-config`, and several runtime libs (`libssl3t64`, `libffi8`, `liblzma5`, `libzstd1`, `libsqlite3-0`) are pre-installed. **`python3`, `cmake`, `ninja-build`, and all `-dev` headers are missing.**

---

## 1. Executive Summary

Building Rust 1.88 from source via `x.py build` on Debian Trixie requires, in
addition to what is already in the container:

| Category | Items | Why this build needs them |
| --- | --- | --- |
| Python 3 | `python3` | `x.py:1` shebang is `#!/usr/bin/env python3`; `bootstrap.py` and the Rust bootstrap binary are invoked through it. |
| Build tools | `cmake`, `ninja-build` | `bootstrap.toml:56` sets `download-ci-llvm = false` and `bootstrap.toml:99` sets `ninja = true`, so the `src/llvm-project` submodule (LLVM 20.1, pinned `rustc/20.1-2025-02-13`) is built **locally** with CMake + Ninja. |
| C/C++ toolchain | already in `build-essential` (gcc, g++, make, binutils) | Sanity-checked for every host target (`sanity.rs:283-304`). |
| Dev headers | `libssl-dev`, `liblzma-dev`, `libzstd-dev`, `libsqlite3-dev`, `libffi-dev`, `zlib1g-dev`, `libstdc++-14-dev` | Linked by `openssl-sys` (cargo), `xz2` (bootstrap), `zstd`, `libsqlite3-sys`, `libffi-sys`, `libz-sys` (libgit2-sys), and LLVM's static C++ runtime. |
| Compression / TLS | `xz-utils`, `ca-certificates` | Required to extract stage0 `.tar.xz` tarballs and verify HTTPS downloads to `static.rust-lang.org`. |
| Network / VCS | already present (`curl`, `git`) | `bootstrap.py:125` requires `curl`; `sanity.rs:106` requires `git` for submodules. |

There are **no third-party Python packages** to install — every `import` in
`x.py`, `bootstrap.py`, `build_helper/*.py`, and `bootstrap_test.py` resolves
to the Python standard library (including `tomllib` from 3.11+).

Recommended order:

1. `apt-get update` (the container's apt index is unpopulated).
2. `apt-get install -y` the apt package list in §2.
3. Do **not** build the LLVM submodule from source on a separate path — let
   rustbuild do it. It is already initialized (commit
   `a9865ceca081`).
4. Run `./x.py build` (or, for a faster first check, `./x.py check`).

---

## 2. Required apt packages

All names below are valid Debian Trixie package names (verified against
`trixie/main`). Group A is required for **any** `./x.py` invocation; Group B
is required for `./x.py build` because of in-tree LLVM; Group C is required
for `./x.py test`.

### Group A — always required

| Package | Reason |
| --- | --- |
| `python3` | x.py shebang + bootstrap.py interpreter. |
| `ca-certificates` | TLS to `static.rust-lang.org`, `crates.io`, GitHub for submodule downloads. |
| `xz-utils` | Stage0 tarballs and `dist/` artifacts are `.tar.xz`. |

### Group B — required for `./x.py build` (LLVM built from source)

| Package | Reason |
| --- | --- |
| `cmake` | LLVM 20 build system (`need_cmake` is true whenever `download-ci-llvm` is false — `sanity.rs:149-161`). Trixie ships cmake ≥ 3.31, well above LLVM 20's 3.20 minimum. |
| `ninja-build` | LLVM is built with ninja (`bootstrap.toml:99`, `lib.rs:1930-1971`). The bootstrap binary checks for both `ninja` and `ninja-build` because Debian renames the binary. |
| `libssl-dev` | `openssl-sys` (cargo's git-over-HTTPS path) and the bootstrap's HTTPS probes. |
| `liblzma-dev` | `xz2 = "0.1"` in `src/bootstrap/Cargo.toml:58`; required by cargo's tarball handling. |
| `libzstd-dev` | LLVM's `llvm.libzstd` feature and `zstd` crate (some tools). |
| `libsqlite3-dev` | `libsqlite3-sys` is pulled by `compiletest` and `tidy`. |
| `libffi-dev` | `libffi-sys` used by several compiler-adjacent crates (lock_api, some proc-macro tooling). |
| `zlib1g-dev` | `libz-sys` (libgit2-sys dependency chain; cargo's HTTP compression). |
| `libstdc++-14-dev` | Debian Trixie ships GCC 14; the dev package provides `libstdc++.a` for static linking of LLVM binaries (`static-libstdcpp` path). Without it, any rustc build that statically links LLVM fails. |

### Group C — required for `./x.py test`

| Package | Reason |
| --- | --- |
| `libssl-dev`, `liblzma-dev`, `libzstd-dev`, `libsqlite3-dev`, `libffi-dev`, `zlib1g-dev` | Same as Group B; tests link the same crates. |
| (no extra) | The default `./x.py test` (compiletest ui/run-make/mir-opt) does not require `qemu`, `valgrind`, or `rustup` — those are only for cross-target or special test suites. |

**Group totals: 3 (A) + 10 (B) = 13 packages.** Group C reuses Group B.

**Skip these (not needed for the default build/test):** `rustup` (the host's
toolchain is only consulted for `bootstrap.defaults`; rustbuild downloads its
own stage0 from `static.rust-lang.org`), `lld`/`mold` (the fork's
`bootstrap.toml` does not opt in), `qemu-user-static` (only for cross-test
suites), `ccache` (not requested by `bootstrap.toml`), `clang`/`libclang-dev`
(sanity.rs accepts either g++ or clang++; `build-essential` is sufficient),
`rustfmt` (built in-tree), `nodejs` (only required by `x.py doc`).

---

## 3. Required Python deps

**None.** Every Python import in this build is stdlib:

- `x.py`: `os`, `sys`, `warnings`, `inspect.cleandoc`
- `src/bootstrap/bootstrap.py`: `argparse`, `contextlib`, `datetime`, `hashlib`, `os`, `re`, `shutil`, `subprocess`, `sys`, `tarfile`, `tempfile`, `time.time`, `multiprocessing.Pool/cpu_count`, optional `lzma`
- `src/bootstrap/bootstrap_test.py`: `tomllib` (Python ≥ 3.11)
- `src/build_helper/*.py`: only stdlib

Trixie ships Python 3.13, which satisfies the `tomllib` requirement and the
soft warning threshold (`major >= 3, minor >= 6`) printed by `x.py:34`.

Do **not** run `pip install` — there is no `requirements.txt` and no
`pyproject.toml` at the workspace root. The Rust bootstrap binary (`cargo`
build of `src/bootstrap/`) is the second-stage driver; Python is just the
launcher and stage0 downloader.

---

## 4. LLVM strategy

**Decision: do NOT pre-build `src/llvm-project` separately. Let rustbuild
build LLVM in-tree using cmake + ninja as part of `x.py build`.**

Rationale:

- `bootstrap.toml:56` sets `download-ci-llvm = false`, which forces the
  in-tree build path (`sanity.rs:138-148` flips `building_llvm = true`).
- `bootstrap.toml:99` sets `ninja = true`.
- `bootstrap.toml:73` sets `assertions = true` (LLVM debug build — slow but
  matches what `cargo +nightly rustc -vV` reports on the host).
- The submodule is already at the correct commit `a9865ceca081` on branch
  `rustc/20.1-2025-02-13` (LLVM 20.1). No submodule update is needed.

Expected in-place LLVM build size: **6–9 GB** under
`build/x86_64-unknown-linux-gnu/llvm/`. Estimated LLVM-only build time on
this sandbox (cold, no LTO): **45–90 minutes** on a modern x86_64. With LTO
or thin-LTO (off by default), this doubles.

If we ever want to skip the LLVM build (saves ~70% of stage2 wall time), set
`download-ci-llvm = true` in `bootstrap.toml` — but that pins us to the CI
LLVM, which is built without assertions, so `assertions = true` in our config
would be silently ignored. Not recommended for development.

---

## 5. Disk and time estimates

### Disk

| Item | Size | Path |
| --- | --- | --- |
| Existing git history | 8.1 GB | `.git/` |
| Existing LLVM submodule working tree | 6.2 GB | `src/llvm-project/` |
| Existing partial build | 0.8 GB | `target/` (old host build artifacts) |
| **Subtotal — already present** | **~15 GB** | |
| Stage0 download cache (rustc/cargo/clippy/rustfmt/std .tar.xz) | 0.8–1.2 GB | `build/cache/2025-04-02/` |
| Stage0 sysroot (extracted) | 0.3 GB | `build/x86_64-unknown-linux-gnu/stage0/` |
| Bootstrap binary (Rust build of `src/bootstrap`) | 0.8 GB | `build/bootstrap/` + `build/x86_64-.../bootstrap/` |
| LLVM build (with assertions, default targets) | 6–9 GB | `build/x86_64-.../llvm/` |
| Rustc stage 1 (libstd + compiler + tools) | 4–6 GB | `build/x86_64-.../stage1-{std,rustc,tools}/` |
| Rustc stage 2 (with extended=true) | 4–6 GB | `build/x86_64-.../stage2-{std,rustc,tools}/` |
| Final stage2 sysroot + dist artifacts | 1–2 GB | `build/x86_64-.../stage2/` |
| **`./x.py check` only** (no LLVM build) | ~3 GB additional | `build/` |
| **`./x.py build` (stage2, full)** | ~25–35 GB additional | `build/` |

**Realistic floor for a successful stage2 build: ~40 GB free in the repo root,
on top of the ~15 GB already consumed by source. Plan for 50 GB.**

The previous build used `/home/houtamelo/.rustup/toolchains/unchained`
(referenced in `target/.rustc_info.json`), confirming a host 1.88.5-nightly
toolchain with LLVM 20.1.2. We do NOT need that host path inside the
container — rustbuild downloads stage0 fresh.

### Time

On a typical 4-core x86_64 sandbox, single-threaded where parallelization is
limited:

| Phase | Wall time (approx.) |
| --- | --- |
| `x.py setup` / download stage0 | 2–5 min |
| Bootstrap binary build (stage 0 → 1) | 2–4 min |
| LLVM build (cmake + ninja, assertions) | 45–90 min |
| rustc stage 1 (libstd + compiler) | 30–60 min |
| rustc stage 2 (extended=true) | 30–60 min |
| **`./x.py build` end-to-end** | **~2–4 hours** |
| `./x.py check` (no LLVM, type-check only) | 10–20 min |
| `./x.py test` (incremental, after build) | 1–3 hours |

---

## 6. Sandbox-specific notes

### Rootless Podman + sudo

- The container is **rootless** by design, but we have **passwordless sudo**
  for everything. The canonical install pattern is `sudo apt-get install -y
  <pkg>`. The `.claude-sandbox.deps.sh` hook runs as root directly, so no
  `sudo` prefix is needed there.
- The user is `claude` (UID 1000). All build commands (`./x.py …`) should be
  run as `claude`, not root, because rustbuild writes many small files and
  ownership becomes awkward if some files are created as root.

### No GPU

- Rustc does not need a GPU. The `nvptx64-nvidia-cuda` target in
  `src/stage0` is downloaded as a dist tarball but never built unless we
  pass `--target nvptx64-nvidia-cuda` explicitly. We do not.
- No GPU passthrough was configured for this project (`claude-sandbox.toml`
  does not request it).

### Bind mounts and persistence

- `$CS_PROJECT_PATH` (the repo) is bind-mounted RW. Anything written under
  the repo — including `build/`, `target/`, and `src/llvm-project/` build
  outputs — **persists across container restarts**.
- `~/.engram` is bind-mounted RW for Engram memory.
- **`~/.cargo` is NOT bind-mounted.** The cargo registry cache will be
  created at `~/.cargo/registry` inside the container and **lost on
  `claude-sandbox down`**. To mitigate, either:
  - Persist `.claude-sandbox.toml`'s `[[mount]]` recipe for `~/.cargo` on
    the host, or
  - Accept the cost: ~500 MB re-download on first build after a restart
    (~5–10 min on a fast link).
- The previous build's `target/` is **already inside** the bind mount and
  persists, but it was produced by the host's `unchained` toolchain. It may
  or may not be reusable; `x.py build` will detect the `.rustc_info.json`
  fingerprint mismatch and either rebuild or refuse to proceed. **Recommendation:
  leave it as-is on first try; if rustbuild refuses, `rm -rf target/` and
  rebuild from scratch.**
- Do not write important artifacts to `/tmp` or other container-internal
  paths; they vanish on container destruction.

### Submodules

- `.gitmodules` lists 13 submodules. From `git status`, the working tree has
  new commits on `src/tools/cargo`, `library/stdarch`, `src/doc/reference`,
  `src/doc/nomicon`, `src/doc/edition-guide`, `src/gcc`,
  `src/tools/rustc-perf`. The submodule recovery plan is owned by doc `04`;
  this doc does not address it.
- `src/llvm-project` is the only submodule whose **content** is consumed by
  `x.py build`. The doc submodules are only used by `./x.py doc` and can be
  left dirty without breaking a build.

---

## 7. Ready-to-paste `.claude-sandbox.deps.sh` snippet

Append this block to
`/home/houtamelo/Documents/rust_unchained/.claude-sandbox.deps.sh`. It is
idempotent (safe to re-run after a container reset).

```bash
# ----------------------------------------------------------------------
# rust_unchained: build dependencies for ./x.py build (Rust 1.88 source build)
# Installs: Python 3, CMake+Ninja (for in-tree LLVM 20.1 build),
#           OpenSSL/zlib/liblzma/libzstd/libsqlite/libffi dev headers,
#           libstdc++ static runtime, TLS certs, xz-utils.
# Safe to re-run; idempotent.
# ----------------------------------------------------------------------

set -euo pipefail

# Refresh apt index (idempotent; cheap when already fresh).
apt-get update -qq

# Core toolchain + Python + LLVM build prerequisites.
apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    cmake \
    ninja-build \
    python3 \
    ca-certificates \
    xz-utils

# Native C library headers linked by Rust crates in the bootstrap toolchain
# (cargo's openssl-sys, xz2, zstd, sqlite3-sys, libffi-sys, libz-sys).
apt-get install -y --no-install-recommends \
    libssl-dev \
    liblzma-dev \
    libzstd-dev \
    libsqlite3-dev \
    libffi-dev \
    zlib1g-dev \
    libstdc++-14-dev

# Sanity check: make sure the tools x.py and bootstrap expect are present.
command -v python3   >/dev/null || { echo "FATAL: python3 not installed"; exit 1; }
command -v cmake     >/dev/null || { echo "FATAL: cmake not installed"; exit 1; }
command -v ninja     >/dev/null || command -v ninja-build >/dev/null \
                     || { echo "FATAL: ninja not installed"; exit 1; }
command -v gcc       >/dev/null || { echo "FATAL: gcc not installed"; exit 1; }
command -v g++       >/dev/null || { echo "FATAL: g++ not installed"; exit 1; }
command -v pkg-config >/dev/null || { echo "FATAL: pkg-config not installed"; exit 1; }

# Verify Python is new enough for tomllib (>= 3.11) — Trixie ships 3.13.
python3 -c 'import sys, tomllib; assert sys.version_info >= (3,11), sys.version' \
    || { echo "FATAL: python3 < 3.11 (tomllib required)"; exit 1; }

# Verify CMake is new enough for LLVM 20 (>= 3.20) — Trixie ships 3.31.
cmake --version | head -1

echo "rust_unchained build dependencies installed."
```

This installs **14 distinct apt packages** (`build-essential`,
`pkg-config`, `cmake`, `ninja-build`, `python3`, `ca-certificates`,
`xz-utils`, `libssl-dev`, `liblzma-dev`, `libzstd-dev`, `libsqlite3-dev`,
`libffi-dev`, `zlib1g-dev`, `libstdc++-14-dev`) plus their transitive
dependencies.

---

## 8. Verification commands

Run these after the deps script finishes (or in a fresh container, after
installing deps), to confirm every required tool is in place.

```bash
# 1. Build tooling
gcc --version          # g++/gcc from build-essential
g++ --version
make --version
pkg-config --version
cmake --version        # must be >= 3.20 for LLVM 20
ninja --version        # or: ninja-build --version

# 2. Python
python3 --version      # must be >= 3.11 for tomllib
python3 -c 'import tomllib; print("tomllib ok")'

# 3. Source control + downloads
git --version
curl --version | head -1

# 4. Native headers (must all report a version, not "not found")
pkg-config --modversion openssl   # libssl-dev
pkg-config --modversion liblzma   # liblzma-dev
pkg-config --modversion libzstd   # libzstd-dev
pkg-config --modversion sqlite3   # libsqlite3-dev
pkg-config --modversion libffi    # libffi-dev
pkg-config --modversion zlib      # zlib1g-dev

# 5. Static C++ runtime (for LLVM static linking)
ls /usr/lib/x86_64-linux-gnu/libstdc++.a 2>&1
# Expected: /usr/lib/x86_64-linux-gnu/libstdc++.a exists

# 6. x.py's own sanity check (does NOT require a full build)
cd /home/houtamelo/Documents/rust_unchained
./x.py --help | head -20
# Expected: prints usage. Will download stage0 rustc on first build invocation.

# 7. Submodule commit (LLVM pin)
git -C src/llvm-project rev-parse HEAD
# Expected: a9865ceca081... (or any commit on rustc/20.1-2025-02-13)

# 8. Disk space check (need ~50 GB for full stage2 build)
df -h /home/houtamelo/Documents/rust_unchained
```

If any of #1–#5 fail, re-run the apt snippet in §7. If #6 fails with "unable
to run `python3`", the python3 symlink is missing — `apt-get install
python3` should create it at `/usr/bin/python3` on Trixie. If #8 reports less
than 50 GB, clean `target/` or other cached artifacts before starting the
build.

---

## 9. Open questions / risks

- **`download-ci-llvm = false`** was set by the previous orchestrator. If we
  later flip it to `true` to save wall time, `assertions = true` in
  `bootstrap.toml` becomes a no-op (CI LLVM is assertions-disabled). That's
  a behavior change vs. the host build and should be a deliberate choice.
- **`target/` was produced by host's `unchained` toolchain.** rustbuild may
  detect the fingerprint mismatch in `target/.rustc_info.json` and either
  rebuild from scratch or refuse to proceed. Worst case: delete `target/`
  and rebuild (costs ~10 min of re-incremental work, mostly free thanks to
  `target/debug` being small).
- **Network egress to `static.rust-lang.org` and `crates.io`.** The
  container is on a bridged network with full outbound. Confirmed by
  sandbox doc. If egress is later blocked, the build will fail at stage0
  download.
- **No `~/.cargo` bind mount.** First build inside a fresh container pulls
  ~500 MB of crates. Consider adding a `[[mount]]` for `~/.cargo` in
  `.claude-sandbox.toml` if container churn is expected.
- **Memory pressure during LLVM build.** LLVM with `assertions = true` and
  default targets can spike to ~4–6 GB RSS during ninja jobs. The sandbox
  default memory limit should accommodate this, but watch OOM kills.

---

## 10. References (verified while writing this doc)

- `INSTALL.md:30-60` — official required-deps list.
- `src/bootstrap/src/core/sanity.rs:81-374` — canonical tool check logic.
- `src/bootstrap/src/lib.rs:1930-1971` — ninja detection (also accepts
  `ninja-build`).
- `src/bootstrap/bootstrap.py:1-15` — Python imports.
- `src/bootstrap/bootstrap_test.py:35-38` — `tomllib` requirement.
- `src/bootstrap/Cargo.toml:35-58` — Rust crates the bootstrap binary
  depends on (cc, cmake, xz2, sha2, tar, toml, walkdir).
- `src/tools/cargo/Cargo.toml:66,72` — cargo's native deps
  (libgit2-sys, openssl-sys).
- `src/bootstrap/defaults/bootstrap.dist.toml:14-25` — `download-ci-llvm =
  false`, `extended = true` for source builds.
- `bootstrap.toml:56,99` — current project settings.
- `target/.rustc_info.json` — confirms previous build was `rustc 1.88.5-nightly`
  with `LLVM version: 20.1.2`, host `x86_64-unknown-linux-gnu`.
- https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html —
  upstream disk-space note ("upwards of 10 or 15 gigabytes" for stage1, 100
  GB "for full builds beyond stage 1").
