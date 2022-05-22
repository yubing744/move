#!/bin/bash
# Copyright (c) The Move Contributors
# SPDX-License-Identifier: Apache-2.0
#
# A script to check whether a local commit is ready for a PR.
# This simulates CI checks locally

set -e

BUILD_FLAGS=

BASE=$(git rev-parse --show-toplevel)
echo "*************** [build-web] Assuming move root at $BASE"


echo "build move-cli"
cd $BASE/language/tools/move-cli && cargo +nightly build -Zbuild-std=std,panic_abort --target=wasm32-wasi --release