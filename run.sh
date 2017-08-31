#!/bin/sh

set -eu

cargo build

./target/debug/gc --location ~/repos2.csv $@
