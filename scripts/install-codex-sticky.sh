#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd -- "$script_dir/.." && pwd)"

cd "$repo_root/codex-rs"
cargo build --release --bin codex

install -d "$HOME/.local/libexec" "$HOME/.local/bin"
install -m 0755 "$repo_root/codex-rs/target/release/codex" \
  "$HOME/.local/libexec/codex-sticky-bin"

cat >"$HOME/.local/bin/codex-sticky" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

exec "$HOME/.local/libexec/codex-sticky-bin" \
  -c 'tui.sticky_transcript=true' \
  "$@"
EOF
chmod 0755 "$HOME/.local/bin/codex-sticky"

echo "codex:"
which codex || true
echo
echo "codex-sticky:"
which codex-sticky || true
echo
echo "Confirm that codex and codex-sticky resolve to different entries before relying on both."
