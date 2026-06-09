#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd -- "$script_dir/.." && pwd)"
sticky_version="${CODEX_STICKY_VERSION:-0.138.0-sticky.1}"

cd "$repo_root/codex-rs"
cargo build --release --bin codex

install -d "$HOME/.local/libexec" "$HOME/.local/bin"
install -m 0755 "$repo_root/codex-rs/target/release/codex" \
  "$HOME/.local/libexec/codex-sticky-bin"

cat >"$HOME/.local/bin/codex-sticky" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ ${1:-} == "--version" || ${1:-} == "-V" ]] && [[ $# -eq 1 ]]; then
  echo "codex-sticky __CODEX_STICKY_VERSION__"
  exit 0
fi

exec -a codex-sticky "$HOME/.local/libexec/codex-sticky-bin" "$@"
EOF
sed -i "s/__CODEX_STICKY_VERSION__/$sticky_version/g" "$HOME/.local/bin/codex-sticky"
chmod 0755 "$HOME/.local/bin/codex-sticky"

echo "codex:"
which codex || true
echo
echo "codex-sticky:"
which codex-sticky || true
echo
echo "Confirm that codex and codex-sticky resolve to different entries before relying on both."
