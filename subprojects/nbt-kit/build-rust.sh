#!/usr/bin/env bash
set -e
RUSTC="$(rustup which --toolchain stable rustc)" \
  rustup run nightly cargo -Z unstable-options build --artifact-dir "$1"
