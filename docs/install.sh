#!/bin/sh
set -eu

FEMACS_VERSION="${FEMACS_VERSION:-0.0.1}"
FEMACS_INSTALL_DIR="${FEMACS_INSTALL_DIR:-$HOME/.local/bin}"
FEMACS_DOWNLOAD_BASE_URL="${FEMACS_DOWNLOAD_BASE_URL:-https://github.com/liuchong/femacs/releases/download/v${FEMACS_VERSION}}"

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "femacs install: missing required command: $1" >&2
    exit 1
  fi
}

need_cmd uname
need_cmd tar
need_cmd mktemp
need_cmd awk

if command -v curl >/dev/null 2>&1; then
  fetch_to() {
    curl -fsSL "$1" -o "$2"
  }
elif command -v wget >/dev/null 2>&1; then
  fetch_to() {
    wget -qO "$2" "$1"
  }
else
  echo "femacs install: requires curl or wget" >&2
  exit 1
fi

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$1" | awk '{print $NF}'
  else
    echo "femacs install: requires sha256sum, shasum, or openssl for SHA-256 verification" >&2
    exit 1
  fi
}

os_name=$(uname -s)
arch_name=$(uname -m)

case "$os_name:$arch_name" in
  Darwin:arm64|Darwin:aarch64)
    target="aarch64-apple-darwin"
    ;;
  Darwin:x86_64)
    target="x86_64-apple-darwin"
    ;;
  Linux:aarch64|Linux:arm64)
    target="aarch64-unknown-linux-gnu"
    ;;
  Linux:x86_64|Linux:amd64)
    target="x86_64-unknown-linux-gnu"
    ;;
  *)
    echo "femacs install: no prebuilt archive for ${os_name}/${arch_name}." >&2
    echo "Install from source with Cargo: cargo install elisp femacs" >&2
    exit 1
    ;;
esac

archive="femacs-v${FEMACS_VERSION}-${target}.tar.gz"
archive_url="${FEMACS_DOWNLOAD_BASE_URL}/${archive}"
sums_url="${FEMACS_DOWNLOAD_BASE_URL}/SHA256SUMS.txt"

tmpdir=$(mktemp -d "${TMPDIR:-/tmp}/femacs-install.XXXXXX")
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT INT TERM

echo "Downloading ${archive_url}..."
fetch_to "$sums_url" "$tmpdir/SHA256SUMS.txt"
fetch_to "$archive_url" "$tmpdir/$archive"

expected_sha=$(awk -v f="$archive" '$2 == f { print $1; exit }' "$tmpdir/SHA256SUMS.txt")
if [ -z "$expected_sha" ]; then
  echo "femacs install: checksum entry missing for ${archive} in SHA256SUMS.txt" >&2
  exit 1
fi

actual_sha=$(sha256_of "$tmpdir/$archive")
if [ "$actual_sha" != "$expected_sha" ]; then
  echo "femacs install: SHA-256 mismatch for ${archive}" >&2
  echo "  expected: $expected_sha" >&2
  echo "  actual:   $actual_sha" >&2
  exit 1
fi

mkdir -p "$tmpdir/unpack" "$FEMACS_INSTALL_DIR"
tar -xzf "$tmpdir/$archive" -C "$tmpdir/unpack"

for bin_name in elisp femacs fe; do
  if [ ! -f "$tmpdir/unpack/$bin_name" ]; then
    echo "femacs install: archive ${archive} missing binary ${bin_name}" >&2
    exit 1
  fi
  stage_bin="$FEMACS_INSTALL_DIR/.${bin_name}.tmp.$$"
  cp "$tmpdir/unpack/$bin_name" "$stage_bin"
  chmod 0755 "$stage_bin"
  mv -f "$stage_bin" "$FEMACS_INSTALL_DIR/$bin_name"
done

echo "Installed elisp, femacs, fe (v${FEMACS_VERSION}, ${target}) into ${FEMACS_INSTALL_DIR}"
case ":${PATH}:" in
  *":${FEMACS_INSTALL_DIR}:"*) ;;
  *)
    echo "Add ${FEMACS_INSTALL_DIR} to your PATH:"
    echo "  export PATH=\"${FEMACS_INSTALL_DIR}:\$PATH\""
    ;;
esac
