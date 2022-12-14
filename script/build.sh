#!/bin/bash
set -euo pipefail

cross build --release --target x86_64-pc-windows-gnu
mv "$GITHUB_WORKSPACE/target/x86_64-pc-windows-gnu/release/gh-github-status.exe" "$GITHUB_WORKSPACE/dist/gh-github-status-windows-amd64.exe"

cross build --release --target x86_64-unknown-linux-gnu
mv "$GITHUB_WORKSPACE/target/x86_64-unknown-linux-gnu/release/gh-github-status" "$GITHUB_WORKSPACE/dist/gh-github-status-linux-amd64"