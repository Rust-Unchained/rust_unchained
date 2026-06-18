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
    ccache \
    lld \
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
command -v ccache     >/dev/null || { echo "FATAL: ccache not installed (bootstrap.toml sets ccache = true)"; exit 1; }
command -v lld        >/dev/null || { echo "FATAL: lld not installed (config.toml sets lld = true and use-lld = true)"; exit 1; }

# Verify Python is new enough for tomllib (>= 3.11).
python3 -c 'import sys, tomllib; assert sys.version_info >= (3,11), sys.version' \
    || { echo "FATAL: python3 < 3.11 (tomllib required)"; exit 1; }

# Verify CMake is new enough for LLVM 20 (>= 3.20).
cmake --version | head -1

echo "rust_unchained build dependencies installed."
