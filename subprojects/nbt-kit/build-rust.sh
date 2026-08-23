#!/usr/bin/env bash
set -e

export RUSTC="$(rustup which --toolchain stable rustc)"

if [ "$CMAKE_BUILD_TYPE" = "Release" ]; then
    rustup run nightly cargo -Z unstable-options build -F objc --release --artifact-dir "$1"
else
    rustup run nightly cargo -Z unstable-options build -F objc --artifact-dir "$1"
fi
