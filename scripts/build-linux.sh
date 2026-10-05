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
  exec sudo --preserve-env=BASH "$0" "$@"
fi

command -v lb >/dev/null || { echo "live-build is required" >&2; exit 1; }
command -v xorriso >/dev/null || { echo "xorriso is required" >&2; exit 1; }

mkdir -p "$DIST_DIR" "$CACHE_DIR"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

SOURCE_PACKAGE_LIST="$PROFILE_DIR/package-lists/nexos.list.chroot"
if [[ "$NO_DESKTOP" -eq 0 && ! -f "$SOURCE_PACKAGE_LIST" ]]; then
  echo "ERROR: Desktop package list missing: $SOURCE_PACKAGE_LIST" >&2
  exit 1
fi

# live-build expects its profile directories under config/.
# Copy the repository profile into a clean live-build working tree.
cp -a "$PROFILE_DIR/." "$BUILD_DIR/config/"

PACKAGE_LIST="$BUILD_DIR/config/package-lists/nexos.list.chroot"
if [[ "$NO_DESKTOP" -eq 0 && ! -f "$PACKAGE_LIST" ]]; then
  echo "ERROR: Desktop package list was not copied to $PACKAGE_LIST" >&2
  find "$BUILD_DIR" -maxdepth 4 -type f | sort >&2 || true
  exit 1
fi

cd "$BUILD_DIR"

if [[ "$NO_DESKTOP" -eq 1 ]]; then
  sed -i '/^task-xfce-desktop[[:space:]]*$/d;/^task-laptop[[:space:]]*$/d;/^task-printing[[:space:]]*$/d;/^lightdm[[:space:]]*$/d;/^lightdm-gtk-greeter[[:space:]]*$/d;/^xfce4[[:space:]]*$/d;/^xfce4-session[[:space:]]*$/d;/^xfce4-panel[[:space:]]*$/d;/^xfdesktop4[[:space:]]*$/d;/^xfce4-terminal[[:space:]]*$/d;/^thunar[[:space:]]*$/d;/^firefox-esr[[:space:]]*$/d' "$PACKAGE_LIST"
fi

if [[ "$NO_DESKTOP" -eq 0 ]]; then
  grep -Eq '^task-xfce-desktop[[:space:]]*$' "$PACKAGE_LIST" || {
    echo "ERROR: Desktop package profile missing" >&2
    exit 1
  }
fi

# Do not manufacture /root/isolinux links. live-build owns bootloader staging.
lb config   --mode debian   --distribution trixie   --architectures amd64   --binary-images iso-hybrid   --bootloader grub   --archive-areas "main contrib non-free non-free-firmware"   --mirror-bootstrap "http://deb.debian.org/debian/"   --mirror-binary "http://deb.debian.org/debian/"   --mirror-chroot "http://deb.debian.org/debian/"   --debian-installer false   --initsystem systemd   --memtest none   --apt-indices false   --apt-source-archives false   --apt-recommends true \
  --security false   --linux-packages "none"   --bootappend-live "boot=live components username=nexos hostname=nexos"

lb build 2>&1 | tee "$ROOT_DIR/build/nexos-live-build.log"

ISO="$(find "$BUILD_DIR" -maxdepth 1 -type f -name '*.iso' -print -quit)"
if [[ -z "$ISO" ]]; then
  echo "ERROR: live-build did not produce an ISO" >&2
  exit 1
fi

cp "$ISO" "$DIST_DIR/NexOS-Linux-x64.iso"
sha256sum "$DIST_DIR/NexOS-Linux-x64.iso" > "$DIST_DIR/NexOS-Linux-x64.iso.sha256"
sha512sum "$DIST_DIR/NexOS-Linux-x64.iso" > "$DIST_DIR/NexOS-Linux-x64.iso.sha512"

SIZE="$(stat -c '%s' "$DIST_DIR/NexOS-Linux-x64.iso")"
echo "Built: $DIST_DIR/NexOS-Linux-x64.iso ($SIZE bytes)"
