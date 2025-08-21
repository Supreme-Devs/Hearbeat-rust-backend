#!/bin/bash
set -e

# Run common setup
./setup.sh

# Run Rust migrations
./setup_rs.sh
