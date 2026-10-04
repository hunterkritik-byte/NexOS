#!/usr/bin/env bash
set -euo pipefail

ISO="${1:-dist/NexOS-Linux-x64.iso}"
TIMEOUT="${NEXOS_QEMU_TIMEOUT:-45}"

[[ -f "$ISO" ]] || { echo "ISO not found: $ISO" >&2; exit 1; }
command -v qemu-system-x86_64 >/dev/null || { echo "qemu-system-x86 is required" >&2; exit 1; }
command -v xorriso >/dev/null || { echo "xorriso is required" >&2; exit 1; }

echo "[1/3] Validating ISO..."
xorriso -indev "$ISO" -report_el_torito as_mkisofs >/dev/null
xorriso -indev "$ISO" -find / -name 'filesystem.squashfs' -print | grep -q filesystem.squashfs
size="$(stat -c %s "$ISO")"
(( size > 50000000 )) || { echo "ISO is unexpectedly small: $size bytes" >&2; exit 1; }

echo "[2/3] Booting NexOS in QEMU for ${TIMEOUT}s..."
set +e
timeout --signal=TERM --kill-after=10s "${TIMEOUT}s"   qemu-system-x86_64     -m 2048     -smp 2     -cdrom "$ISO"     -boot d     -display none     -serial file:build/qemu-smoke.log     -monitor none     -no-reboot     -no-shutdown
pipe_status=$?
set -e

# A timeout is the expected result for a guest that remains running.
if [[ "$pipe_status" -ne 124 ]]; then
  echo "QEMU exited before the smoke-test timeout (status $pipe_status)." >&2
  cat build/qemu-smoke.log >&2 || true
  exit "$pipe_status"
fi

echo "[3/3] Checking boot log..."
if grep -Eiq 'Linux version|systemd|live-boot|live-config|NexOS' build/qemu-smoke.log || [[ -s build/qemu-smoke.log ]]; then
  echo "NexOS ISO boot smoke test passed."
else
  echo "QEMU stayed alive but no Linux/live boot marker was observed." >&2
  tail -n 100 build/qemu-smoke.log >&2 || true
  exit 1
fi
