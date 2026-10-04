#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$ROOT_DIR/build/live"
DIST_DIR="$ROOT_DIR/dist"
PROFILE_DIR="$ROOT_DIR/distro/config"
NO_DESKTOP=0

for arg in "$@"; do
  case "$arg" in
    --no-desktop) NO_DESKTOP=1 ;;
    -h|--help)
      echo "Usage: sudo $0 [--no-desktop]"
      exit 0
      ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

if [[ "$EUID" -ne 0 ]]; then
  echo "Run as root: sudo $0" >&2
  exit 1
fi

command -v lb >/dev/null || { echo "live-build is required"; exit 1; }
command -v xorriso >/dev/null || { echo "xorriso is required"; exit 1; }

mkdir -p "$DIST_DIR"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
cp -a "$PROFILE_DIR/." "$BUILD_DIR/"

cd "$BUILD_DIR"

if [[ "$NO_DESKTOP" -eq 1 ]]; then
  sed -i '/^task-xfce-desktop$/d;/^lightdm$/d;/^lightdm-gtk-greeter$/d;/^xfce4-terminal$/d;/^thunar$/d;/^firefox-esr$/d' config/package-lists/nexos.list.chroot
fi

lb config   --distribution trixie   --architectures amd64   --binary-images iso-hybrid   --archive-areas "main contrib non-free non-free-firmware"   --debian-installer live   --debian-installer-gui true   --memtest none   --apt-recommends true   --linux-packages "linux-image linux-headers"   --bootappend-live "boot=live components username=nexos hostname=nexos"

lb build 2>&1 | tee "$ROOT_DIR/build/nexos-live-build.log"

ISO="$(find "$BUILD_DIR" -maxdepth 1 -type f -name '*.iso' -print -quit)"
if [[ -z "$ISO" ]]; then
  echo "ERROR: live-build did not produce an ISO" >&2
  exit 1
fi

cp "$ISO" "$DIST_DIR/NexOS-Linux-amd64.iso"
sha256sum "$DIST_DIR/NexOS-Linux-amd64.iso" > "$DIST_DIR/NexOS-Linux-amd64.iso.sha256"

echo "Built: $DIST_DIR/NexOS-Linux-amd64.iso"
