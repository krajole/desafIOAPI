#!/bin/bash

set -e

TAG=$(grep '^version =' Cargo.toml | head -n 1 | sed -E 's/version = "([^"]+)"/v\1/g')

read -p "Creating new release for $TAG. Do you want to continue? [Y/n] " prompt

if [[ $prompt == "y" || $prompt == "Y" || $prompt == "yes" || $prompt == "Yes" ]]; then
    TAG=$TAG python scripts/prepare_changelog.py
 