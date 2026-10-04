#!/usr/bin/env bash
set -euo pipefail

ISO="${1:-dist/NexOS-Linux-amd64.iso}"
TIMEOUT="${NEXOS_QEMU_TIMEOUT:-90}"

[[ -f "$ISO" ]] || { echo "ISO not found: $ISO" >&2; exit 1; }
command -v qemu-system-x86_64 >/dev/null || { echo "qemu-system-x86 is required" >&2; exit 1; }
command -v xorriso >/dev/null || { echo "xorriso is required" >&2; exit 1; }

echo "[1/3] Validating ISO..."
xorriso -indev "$ISO" -report_el_torito as_mkisofs >/dev/null
size="$(stat -c %s "$ISO")"
(( size > 50000000 )) || { echo "ISO is unexpectedly small: $size bytes" >&2; exit 1; }

echo "[2/3] Booting NexOS in QEMU for ${TIMEOUT}s..."
set +e
timeout --signal=TERM --kill-after=10s "${TIMEOUT}s" \
  qemu-system-x86_64 \
    -m 2048 \
    -smp 2 \
    -cdrom "$ISO" \
    -boot d \
    -display none \
    -serial stdio \
    -no-reboot \
    -no-shutdown \
    2>&1 | tee build/qemu-smoke.log
pipe_status=${PIPESTATUS[0]}
set -e

if [[ "$pipe_status" -ne 124 ]]; then
  echo "QEMU exited before the smoke-test timeout (status $pipe_status)." >&2
  exit "$pipe_status"
fi

echo "[3/3] Checking boot log..."
if grep -Eiq 'NexOS Linux|Welcome to NexOS Linux|systemd\[[0-9]+\].*Reached target|live-config' build/qemu-smoke.log; then
  echo "NexOS ISO boot smoke test passed."
else
  echo "QEMU stayed alive, but no NexOS/live boot marker was observed." >&2
  tail -n 100 build/qemu-smoke.log >&2 || true
  exit 1
fi
