#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
export RUST_BACKTRACE=1
export RUSTFLAGS=-Awarnings

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

set -x
cargo run -v --bin dkvs_server -- $*
