#!/bin/bash

set -e

TAG=$(grep '^version =' Cargo.toml | head -n 1 | sed -E 's/version = "([^"]+)"/v\1/g')

read -p "Creating new re