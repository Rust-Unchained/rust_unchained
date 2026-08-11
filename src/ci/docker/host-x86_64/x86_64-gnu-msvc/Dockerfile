FROM ubuntu:26.04

ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
  g++ \
  make \
  ninja-build \
  file \
  curl \
  ca-certificates \
  python3 \
  git \
  cmake \
  sudo \
  gdb \
  libssl-dev \
  pkg-config \
  xz-utils \
  zlib1g-dev \
  libzstd-dev \
  clang \
  lld \
  llvm \
  && ln -s "$(command -v clang)" /usr/local/bin/clang-cl \
  && ln -s "$(command -v llvm-ar)" /usr/local/bin/llvm-lib \
  && ln -s "$(command -v lld-link)" /usr/local/bin/lld-link \
  && rm -rf /var/lib/apt/lists/*

COPY scripts/sccache.sh /scripts/
RUN sh /scripts/sccache.sh

# xwin is mounted at /xwin by run.sh when XWIN_ROOT is configured.
ENV CC_x86_64_pc_windows_msvc=clang-cl
ENV CXX_x86_64_pc_windows_msvc=clang-cl
ENV AR_x86_64_pc_windows_msvc=llvm-lib
ENV CFLAGS_x86_64_pc_windows_msvc="-Wno-unused-command-line-argument -fuse-ld=lld-link /vctoolsdir /xwin/crt /winsdkdir /xwin/sdk"
ENV CXXFLAGS_x86_64_pc_windows_msvc="${CFLAGS_x86_64_pc_windows_msvc}"
ENV RUST_BOOTSTRAP_CONFIG=/checkout/src/ci/docker/host-x86_64/x86_64-gnu-msvc/bootstrap.toml
ENV SCRIPT="python3 ../x.py build"
