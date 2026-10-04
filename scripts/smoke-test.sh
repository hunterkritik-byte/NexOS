#!/usr/bin/env bash
set -euo pipefail

ISO="${1:-dist/NexOS-Linux-amd64.iso}"
[[ -f "$ISO" ]] || { echo "ISO not found: $ISO" >&2; exit 1; }

command -v qemu-system-x86_64 >/dev/null || { echo "qemu-system-x86 is required" >&2; exit 1; }
command -v xorriso >/dev/null || { echo "xorriso is required" >&2; exit 1; }

echo "[1/2] Checking ISO structure..."
xorriso -indev "$ISO" -report_el_torito as_mkisofs >/dev/null

echo "[2/2] Starting QEMU for 30 seconds..."
set +e
timeout 30s qemu-system-x86_64   -enable-kvm   -m 2048   -smp 2   -cdrom "$ISO"   -boot d   -display none   -serial stdio   -no-reboot
rc=$?
set -e

if [[ "$rc" -eq 124 ]]; then
  echo "QEMU remained alive for the full smoke-test window."
  exit 0
fi

if [[ "$rc" -ne 0 ]]; then
  echo "QEMU exited with status $rc" >&2
  exit "$rc"
fi
