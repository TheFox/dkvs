#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
export RUST_BACKTRACE=full

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."
source ./.env

set -x
cargo test --workspace $* -- --nocapture
