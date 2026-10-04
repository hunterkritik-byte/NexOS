#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_DIR="$ROOT_DIR/build/live"
CACHE_DIR="$ROOT_DIR/build/cache"
DIST_DIR="$ROOT_DIR/dist"
PROFILE_DIR="$ROOT_DIR/distro/config"
NO_DESKTOP=0

for arg in "$@"; do
  case "$arg" in
    --no-desktop) NO_DESKTOP=1 ;;
    -h|--help) echo "Usage: $0 [--no-desktop]"; exit 0 ;;
    *) echo "Unknown argument: $arg" >&2; exit 2 ;;
  esac
done

if [[ "$EUID" -ne 0 ]]; then
  exec sudo --preserve-env=bash "$0" "$@"
fi

command -v lb >/dev/null || { echo "live-build is required"; exit 1; }
command -v xorriso >/dev/null || { echo "xorriso is required"; exit 1; }

mkdir -p "$DIST_DIR" "$CACHE_DIR"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
SOURCE_PACKAGE_LIST="$PROFILE_DIR/package-lists/nexos.list.chroot"
if [[ ! -f "$SOURCE_PACKAGE_LIST" ]]; then
  echo "ERROR: source desktop package list missing: $SOURCE_PACKAGE_LIST" >&2
  exit 1
fi
cp -a "$PROFILE_DIR/." "$BUILD_DIR/"
if [[ ! -f "$BUILD_DIR/package-lists/nexos.list.chroot" ]]; then
  echo "ERROR: live-build package list was not copied to $BUILD_DIR/package-lists/nexos.list.chroot" >&2
  find "$BUILD_DIR" -maxdepth 3 -type f | sort >&2 || true
  exit 1
fi
cd "$BUILD_DIR"

rm -rf config/chroot_local-includes config/includes.binary
mkdir -p config/chroot_local-includes
mkdir -p config/hooks/normal
cat > config/hooks/normal/9900-fix-isolinux-links.hook.chroot <<'HOOK'
#!/bin/sh
set -eu
mkdir -p /root/isolinux
rm -f /root/isolinux/isolinux.bin /root/isolinux/vesamenu.c32
ln -s /usr/lib/ISOLINUX/isolinux.bin /root/isolinux/isolinux.bin
ln -s /usr/lib/syslinux/modules/bios/vesamenu.c32 /root/isolinux/vesamenu.c32
HOOK
chmod +x config/hooks/normal/9900-fix-isolinux-links.hook.chroot


if [[ "$NO_DESKTOP" -eq 1 ]]; then
  sed -i '/^task-xfce-desktop$/d;/^task-laptop$/d;/^task-printing$/d;/^lightdm$/d;/^lightdm-gtk-greeter$/d;/^xfce4$/d;/^xfce4-session$/d;/^xfce4-panel$/d;/^xfdesktop4$/d;/^xfce4-terminal$/d;/^thunar$/d;/^firefox-esr$/d' "$BUILD_DIR/package-lists/nexos.list.chroot"
fi

if [[ "$NO_DESKTOP" -eq 0 ]]; then
  grep -Eq "^task-xfce-desktop[[:space:]]*$" "$BUILD_DIR/package-lists/nexos.list.chroot" || { echo "ERROR: Desktop package profile missing"; exit 1; }
fi

lb config \
  --mode debian \
  --distribution trixie \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --archive-areas "main contrib non-free non-free-firmware" \
  --mirror-bootstrap "http://deb.debian.org/debian/" \
  --mirror-binary "http://deb.debian.org/debian/" \
  --mirror-binary-security "http://deb.debian.org/debian-security/" \
  --mirror-chroot "http://deb.debian.org/debian/" \
  --mirror-chroot-security "http://deb.debian.org/debian-security/" \
  --debian-installer false \
  --initsystem systemd \
  --memtest none \
  --security false \
  --apt-indices false \
  --apt-source-archives false \
  --apt-recommends true \
  --linux-packages "none" \
  --bootappend-live "boot=live components username=nexos hostname=nexos console=ttyS0,115200"

lb build 2>&1 | tee "$ROOT_DIR/build/nexos-live-build.log"

ISO="$(find "$BUILD_DIR" -maxdepth 1 -type f -name '*.iso' -print -quit)"
if [[ -z "$ISO" ]]; then
  echo "ERROR: live-build did not produce an ISO" >&2
  exit 1
fi

cp "$ISO" "$DIST_DIR/NexOS-Linux-x64.iso"
sha256sum "$DIST_DIR/NexOS-Linux-x64.iso" > "$DIST_DIR/NexOS-Linux-x64.iso.sha256"

echo "Built: $DIST_DIR/NexOS-Linux-x64.iso"
