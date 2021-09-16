#!/bin/bash

set -e

TAG=$(grep '^version =' Cargo.toml | head -n 1 | sed -E