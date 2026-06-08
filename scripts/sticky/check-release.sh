#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/sticky/check-release.sh [release-version]

Run local preflight checks for a manual Sticky release.
This script never pushes, creates tags, creates releases, uploads files, or builds.

Tag format:
  <upstream-version>-sticky.<revision>

Example:
  0.138.0-sticky.1
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

  if [[ "$lower" =~ ^[0-9]+([.][0-9]+)*(a|b|alpha|beta|rc|pre|preview)[0-9]+-sticky[.] ]]; then
    fail "sticky release must not use compact prerelease notation: $tag"
  fi
}

require_sticky_release_tag() {
  local tag="$1"

  [[ -n "$tag" ]] || fail "tag must not be empty"
  [[ "$tag" != -* ]] || fail "tag must not start with '-'"
  git check-ref-format "refs/tags/$tag" >/dev/null 2>&1 || fail "invalid tag name: $tag"

  if [[ "$tag" == v* ]]; then
    fail "sticky release tags must not use a 'v' prefix: $tag"
  fi

  if [[ ! "$tag" =~ ^[0-9]+[.][0-9]+[.][0-9]+-sticky[.][1-9][0-9]*$ ]]; then
    fail "tag must match <upstream-version>-sticky.<revision> with revision >= 1, for example 0.137.0-sticky.1: $tag"
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
    "scripts/install.sh"
    "scripts/sticky/sync-upstream.sh"
    "scripts/sticky/check-release.sh"
    ".github/workflows/sticky-ci.yml"
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

Artifact naming guidance only; no build, upload, or release was performed:
  - command inside archive: codex-sticky
  - binary payload path:   libexec/codex-sticky-bin
  - Linux archive:          codex-sticky-${tag}-x86_64-unknown-linux-gnu.tar.gz
  - checksum file:          SHA256SUMS
  - archive contents:       codex-sticky, libexec/codex-sticky-bin, LICENSE, NOTICE
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
  info "No push, tag creation, upload, release publication, or build was performed."
}

main "$@"
