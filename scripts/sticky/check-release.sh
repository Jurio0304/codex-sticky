#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/sticky/check-release.sh [sticky-release-tag]

Run local preflight checks for a Sticky release tag.
This script never pushes, creates tags, creates releases, uploads files, or builds.

Tag format:
  v<upstream-version>-sticky.<revision>

Example:
  v0.138.0-sticky.1
USAGE
}

fail() {
  echo "error: $*" >&2
  exit 1
}

warn() {
  echo "warning: $*" >&2
}

info() {
  echo "==> $*"
}

require_git_repo() {
  git rev-parse --git-dir >/dev/null 2>&1 || fail "must be run inside a Git repository"
}

require_clean_worktree() {
  if [[ -n "$(git status --porcelain)" ]]; then
    fail "working tree is dirty; release checks require a clean tree"
  fi
}

require_attached_head() {
  local branch
  branch="$(git branch --show-current)"
  if [[ -z "$branch" ]]; then
    fail "detached HEAD is not allowed for release checks"
  fi
}

current_exact_tag() {
  git describe --exact-match --tags HEAD 2>/dev/null || true
}

resolve_tag() {
  if [[ "$#" -eq 1 ]]; then
    echo "$1"
    return 0
  fi

  if [[ "$#" -ne 0 ]]; then
    usage >&2
    exit 2
  fi

  local tag
  tag="$(current_exact_tag)"
  [[ -n "$tag" ]] || fail "HEAD is not exactly on a tag; pass a sticky release tag explicitly"
  echo "$tag"
}

reject_prerelease_version() {
  local tag="$1"
  local lower="${tag,,}"

  if [[ "$lower" =~ (^|[^[:alnum:]])(alpha|beta|rc|pre|preview|canary|nightly|dev)([^[:alnum:]]|[0-9]|$) ]]; then
    fail "sticky release must be based on a stable upstream version: $tag"
  fi

  if [[ "$lower" =~ ^v[0-9]+([.][0-9]+)*(a|b|alpha|beta|rc|pre|preview)[0-9]+-sticky[.] ]]; then
    fail "sticky release must not use compact prerelease notation: $tag"
  fi
}

require_sticky_release_tag() {
  local tag="$1"

  [[ -n "$tag" ]] || fail "tag must not be empty"
  [[ "$tag" != -* ]] || fail "tag must not start with '-'"
  git check-ref-format "refs/tags/$tag" >/dev/null 2>&1 || fail "invalid tag name: $tag"

  if [[ ! "$tag" =~ ^v[0-9]+([.][0-9]+)*-sticky[.][0-9]+$ ]]; then
    fail "tag must match v<upstream-version>-sticky.<revision>: $tag"
  fi

  reject_prerelease_version "$tag"
}

check_tag_exists_if_local() {
  local tag="$1"
  if ! git show-ref --verify --quiet "refs/tags/$tag"; then
    warn "tag does not exist locally yet: $tag"
    warn "this is acceptable for planning, but create it only after all checks pass"
  fi
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "required file is missing: $path"
}

check_required_files() {
  local files=(
    "README.md"
    "LICENSE"
    "NOTICE"
    ".sticky/upstream-base.json"
    "docs/codex-sticky/PATCHSET.md"
    "docs/codex-sticky/MAINTENANCE.md"
    "docs/codex-sticky/RELEASING.md"
    "docs/codex-sticky/GITHUB_SETTINGS.md"
    "scripts/sticky/sync-upstream.sh"
    "scripts/sticky/check-release.sh"
    ".github/workflows/sticky-ci.yml"
    ".github/workflows/sticky-release.yml"
    ".github/workflows/upstream-watch.yml"
  )

  local file
  for file in "${files[@]}"; do
    require_file "$file"
  done
}

print_artifact_guidance() {
  local tag="$1"
  cat <<GUIDANCE

Artifact naming guidance only; no build or release was performed:
  - binary inside archive: codex-sticky
  - Linux archive:          codex-sticky-${tag}-x86_64-unknown-linux-gnu.tar.gz
  - checksum file:          SHA256SUMS
  - archive contents:       codex-sticky, LICENSE, NOTICE
GUIDANCE
}

main() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
  fi

  require_git_repo
  require_clean_worktree
  require_attached_head

  local tag
  tag="$(resolve_tag "$@")"
  require_sticky_release_tag "$tag"
  check_tag_exists_if_local "$tag"
  check_required_files

  info "Sticky release preflight passed for $tag"
  print_artifact_guidance "$tag"
  info "No push, tag creation, GitHub release, upload, or build was performed."
}

main "$@"
