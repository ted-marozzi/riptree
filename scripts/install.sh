#!/bin/bash
set -euo pipefail

cargo build -r

sudo mv ./target/release/riptree /usr/local/bin
