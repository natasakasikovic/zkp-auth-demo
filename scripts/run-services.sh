#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

WAREHOUSE_URL="${WAREHOUSE_URL:-http://127.0.0.1:3001}"
PAYMENT_URL="${PAYMENT_URL:-http://127.0.0.1:3002}"

PIDS=()

cleanup() {
  echo
  echo "Stopping services..."
  for pid in "${PIDS[@]}"; do
    kill "$pid" 2>/dev/null || true
  done
}

trap cleanup EXIT INT TERM

echo "Starting warehouse-service on http://127.0.0.1:3001"
cargo run -p warehouse-service &
PIDS+=("$!")

echo "Starting payment-service on http://127.0.0.1:3002"
cargo run -p payment-service &
PIDS+=("$!")

echo "Starting order-service on http://127.0.0.1:3000"
WAREHOUSE_URL="$WAREHOUSE_URL" PAYMENT_URL="$PAYMENT_URL" cargo run -p order-service &
PIDS+=("$!")

echo
echo "All services are starting. Press Ctrl+C to stop them."

wait
