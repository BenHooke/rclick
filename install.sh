#!/bin/sh
set -e

REPO="BenHooke/rclick"
BINARY="rclick"
INSTALL_DIR="/usr/local/bin"

# ── Detect OS ─────────────────────────────────────────────────────────────────

OS=$(uname -s)
case "$OS" in
  Linux)  os="linux" ;;
  Darwin) os="macos" ;;
  *)
    echo "Unsupported OS: $OS"
    echo "Please build from source: https://github.com/$REPO"
    exit 1
    ;;
esac

# ── Detect architecture ───────────────────────────────────────────────────────

ARCH=$(uname -m)
case "$ARCH" in
  x86_64)          arch="x86_64" ;;
  aarch64 | arm64) arch="aarch64" ;;
  *)
    echo "Unsupported architecture: $ARCH"
    echo "Please build from source: https://github.com/$REPO"
    exit 1
    ;;
esac

# ── Fetch latest release version ─────────────────────────────────────────────

echo "Fetching latest release..."
VERSION=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"tag_name"' \
  | sed 's/.*"tag_name": *"\(.*\)".*/\1/')

if [ -z "$VERSION" ]; then
  echo "Could not determine latest version. Check https://github.com/$REPO/releases"
  exit 1
fi

echo "Latest version: $VERSION"

# ── Build download URL ────────────────────────────────────────────────────────

ASSET="${BINARY}-${VERSION}-${arch}-${os}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET"

# ── Download and install ──────────────────────────────────────────────────────

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading $ASSET..."
if ! curl -fsSL "$URL" -o "$TMP_DIR/$ASSET"; then
  echo "Download failed. No release found for ${arch}-${os}."
  echo "Please build from source: https://github.com/$REPO"
  exit 1
fi

tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"

# ── Write to install dir (request sudo if needed) ────────────────────────────

if [ -w "$INSTALL_DIR" ]; then
  mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
else
  echo "Installing to $INSTALL_DIR (sudo required)..."
  sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"
fi

chmod +x "$INSTALL_DIR/$BINARY"

echo ""
echo "✓ rclick $VERSION installed to $INSTALL_DIR/$BINARY"
echo ""
echo "To enable 'cd' support, add this to your .bashrc or .zshrc:"
echo ""
echo '  rclick() {'
echo '    local tmp'
echo '    tmp=$(mktemp)'
echo '    RCLICK_CD_FILE="$tmp" command rclick "$@"'
echo '    local dir'
echo '    dir=$(cat "$tmp" 2>/dev/null)'
echo '    rm -f "$tmp"'
echo '    if [[ -n "$dir" ]]; then'
echo '      cd "$dir"'
echo '    fi'
echo '  }'
echo ""
echo "Then run: source ~/.bashrc"
