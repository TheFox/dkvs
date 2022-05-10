#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
export RUSTFLAGS=-Awarnings

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

cargo run --bin dkvs_server -- $*
