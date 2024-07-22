#!/usr/bin/env bash
set -uro pipefail
cd "$(dirname "$0")/.." || exit 1

sea-orm-cli generate entity --output-dir src/database/entities
