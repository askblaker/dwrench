#!/bin/bash
# Exit on error
set -e

echo 'Formatting...'
cargo fmt

echo 'Building...'
cargo build

echo 'Testing...'
cargo test

echo 'Generating readme...'
./generate_readme.sh

echo 'Checking for changes...'
git diff HEAD --exit-code

echo 'Done, all good'