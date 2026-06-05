#!/usr/bin/env bash
set -euo pipefail

REPO="${CODEX_STICKY_REPO:-Jurio0304/codex-sticky}"
VERSION="${CODEX_STICKY_VERSION:-}"
TARGET_TRIPLE="x86_64-unknown-linux-gnu"
VERSION_RE='^[0-9]+\.[0-9]+\.[0-9]+-sticky\.[1-9][0-9]*$'
INSTALL_TMPDIR=""

usage() {
    cat <<'USAGE'
Install codex-sticky for Linux x86_64.

Usage:
  scripts/install.sh

Environment:
  CODEX_STICKY_VERSION  Install an explicit version, e.g. 0.137.0-sticky.1.
  CODEX_STICKY_REPO     GitHub repo to read releases from; defaults to Jurio0304/codex-sticky.

The installer downloads codex-sticky-<version>-x86_64-unknown-linux-gnu.tar.gz
and SHA256SUMS from GitHub Releases, verifies the checksum, and installs only:
  ~/.local/bin/codex-sticky

It does not install or overwrite a binary named "codex".
USAGE
}

die() {
    echo "error: $*" >&2
    exit 1
}

require_command() {
    command -v "$1" >/dev/null 2>&1 || die "missing required command: $1"
}

cleanup() {
    if [[ -n "$INSTALL_TMPDIR" ]]; then
        rm -rf "$INSTALL_TMPDIR"
    fi
}

validate_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    if [[ "$os" != "Linux" || "$arch" != "x86_64" ]]; then
        die "codex-sticky installer only supports Linux x86_64; detected ${os}/${arch}"
    fi
}

validate_version() {
    local version="$1"
    if [[ ! "$version" =~ $VERSION_RE ]]; then
        die "invalid codex-sticky version '$version'; expected format like 0.137.0-sticky.1 with no v prefix"
    fi
}

curl_download() {
    local url="$1"
    local output="$2"

    curl -fsSL --retry 3 --retry-delay 2 --connect-timeout 20 \
        "$url" \
        -o "$output"
}

github_api_download() {
    local url="$1"
    local output="$2"
    local headers=(-H "Accept: application/vnd.github+json")

    if [[ -n "${GITHUB_TOKEN:-}" ]]; then
        headers+=(-H "Authorization: Bearer ${GITHUB_TOKEN}")
    fi

    curl -fsSL --retry 3 --retry-delay 2 --connect-timeout 20 \
        "${headers[@]}" \
        "$url" \
        -o "$output"
}

parse_latest_version_with_python() {
    local releases_json="$1"
    python3 - "$releases_json" <<'PY'
import json
import re
import sys

pattern = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+-sticky\.[1-9][0-9]*$")
with open(sys.argv[1], "r", encoding="utf-8") as handle:
    releases = json.load(handle)

for release in releases:
    tag = release.get("tag_name", "")
    if not release.get("draft") and not release.get("prerelease") and pattern.fullmatch(tag):
        print(tag)
        sys.exit(0)

sys.exit(1)
PY
}

parse_latest_version_with_awk() {
    local releases_json="$1"
    awk '
        function string_value(line, value) {
            value = line
            sub(/^.*"tag_name"[[:space:]]*:[[:space:]]*"/, "", value)
            sub(/".*$/, "", value)
            return value
        }
        function bool_value(line, value) {
            value = line
            sub(/^.*:[[:space:]]*/, "", value)
            sub(/[,[:space:]].*$/, "", value)
            return value
        }
        /"tag_name"[[:space:]]*:/ {
            tag = string_value($0)
            draft = ""
            prerelease = ""
        }
        /"draft"[[:space:]]*:/ {
            draft = bool_value($0)
        }
        /"prerelease"[[:space:]]*:/ {
            prerelease = bool_value($0)
            if (tag ~ /^[0-9]+\.[0-9]+\.[0-9]+-sticky\.[1-9][0-9]*$/ && draft == "false" && prerelease == "false") {
                print tag
                found = 1
                exit 0
            }
        }
        END { exit found ? 0 : 1 }
    ' "$releases_json"
}

latest_version_from_github() {
    local releases_json="$1"
    local latest=""

    github_api_download "https://api.github.com/repos/${REPO}/releases?per_page=100" "$releases_json"

    if command -v python3 >/dev/null 2>&1; then
        if latest="$(parse_latest_version_with_python "$releases_json")"; then
            echo "$latest"
            return 0
        fi
    fi

    if latest="$(parse_latest_version_with_awk "$releases_json")"; then
        echo "$latest"
        return 0
    fi

    return 1
}

install_codex_sticky() {
    : "${HOME:?HOME is not set}"

    local tmpdir archive asset checksums selected_checksums extract_dir binary install_dir target
    INSTALL_TMPDIR="$(mktemp -d)"
    tmpdir="$INSTALL_TMPDIR"
    trap cleanup EXIT

    if [[ -z "$VERSION" ]]; then
        if ! VERSION="$(latest_version_from_github "$tmpdir/releases.json")"; then
            die "could not find a non-draft, non-prerelease codex-sticky release for ${REPO}; set CODEX_STICKY_VERSION explicitly"
        fi
    fi
    validate_version "$VERSION"

    asset="codex-sticky-${VERSION}-${TARGET_TRIPLE}.tar.gz"
    archive="$tmpdir/$asset"
    checksums="$tmpdir/SHA256SUMS"
    selected_checksums="$tmpdir/SHA256SUMS.selected"
    extract_dir="$tmpdir/extract"
    install_dir="$HOME/.local/bin"
    target="$install_dir/codex-sticky"

    echo "Installing codex-sticky ${VERSION} from ${REPO}"
    curl_download "https://github.com/${REPO}/releases/download/${VERSION}/${asset}" "$archive"
    curl_download "https://github.com/${REPO}/releases/download/${VERSION}/SHA256SUMS" "$checksums"

    awk -v file="$asset" '
        $2 == file || $2 == "./" file { print; found = 1 }
        END { exit found ? 0 : 1 }
    ' "$checksums" > "$selected_checksums" || die "SHA256SUMS does not contain an entry for ${asset}"

    (cd "$tmpdir" && sha256sum -c "$selected_checksums")

    mkdir -p "$extract_dir"
    tar -xzf "$archive" -C "$extract_dir"

    binary="$extract_dir/codex-sticky"
    [[ -f "$binary" ]] || die "release archive did not contain a top-level codex-sticky binary"

    install -d -m 0755 "$install_dir"
    install -m 0755 "$binary" "$target"

    echo "Installed: $target"
    echo "The official 'codex' binary was not modified."

    if [[ ":${PATH}:" != *":${install_dir}:"* ]]; then
        echo "Add this to your shell profile if needed:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi

    echo "Uninstall with:"
    echo "  rm -f \"$target\""
}

case "${1:-}" in
    -h|--help)
        usage
        exit 0
        ;;
    "")
        ;;
    *)
        usage >&2
        die "unexpected argument: $1"
        ;;
esac

validate_platform
require_command curl
require_command tar
require_command sha256sum
require_command install

install_codex_sticky
