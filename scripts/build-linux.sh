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
cp -a "$PROFILE_DIR/." "$BUILD_DIR/"
cd "$BUILD_DIR"

rm -rf config/chroot_local-includes config/includes.binary
mkdir -p config/chroot_local-includes

if [[ "$NO_DESKTOP" -eq 1 ]]; then
  sed -i '/^task-xfce-desktop$/d;/^lightdm$/d;/^lightdm-gtk-greeter$/d;/^xfce4-terminal$/d;/^thunar$/d;/^firefox-esr$/d' config/package-lists/nexos.list.chroot
fi

lb config \
  --mode debian \
  --distribution trixie \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --archive-areas "main contrib non-free non-free-firmware" \
  --mirror-bootstrap "http://deb.debian.org/debian/" \
  --mirror-binary "http://deb.debian.org/debian/" \
  --mirror-chroot "http://deb.debian.org/debian/" \
  --debian-installer live \
  --debian-installer-gui true \
  --memtest none \
  --apt-recommends true \
  --linux-packages "linux-image linux-headers" \
  --bootappend-live "boot=live components username=nexos hostname=nexos console=ttyS0,115200"

# live-build 3.x on Ubuntu does not accept the newer security-mirror flags.
# Normalize its generated Trixie security suite before the build starts.
# live-build can materialize the security source in generated config files
# after "lb config", so normalize every generated text file before bootstrap.
find "$BUILD_DIR/config" -type f -print0 | while IFS= read -r -d "" file; do
  sed -i \
    -e "s#security.debian.org/debian-security#deb.debian.org/debian-security#g" \
    -e "s#security.debian.org#deb.debian.org/debian-security#g" \
    -e "s#trixie/updates#trixie-security#g" \
    "$file"
done

# Never reuse a bootstrap cache containing an obsolete Trixie security suite.
rm -rf "$BUILD_DIR/cache" "$BUILD_DIR/.build" "$BUILD_DIR/.stage"

lb build 2>&1 | tee "$ROOT_DIR/build/nexos-live-build.log"

ISO="$(find "$BUILD_DIR" -maxdepth 1 -type f -name '*.iso' -print -quit)"
if [[ -z "$ISO" ]]; then
  echo "ERROR: live-build did not produce an ISO" >&2
  exit 1
fi

cp "$ISO" "$DIST_DIR/NexOS-Linux-x64.iso"
sha256sum "$DIST_DIR/NexOS-Linux-x64.iso" > "$DIST_DIR/NexOS-Linux-x64.iso.sha256"

echo "Built: $DIST_DIR/NexOS-Linux-x64.iso"
