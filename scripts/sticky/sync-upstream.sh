#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: scripts/sticky/sync-upstream.sh <official-tag>

Create a local sync/<official-tag> branch and merge the matching upstream tag.
This script never pushes, opens PRs, creates tags, or publishes releases.

Requirements:
  - current branch is not detached
  - working tree is clean
  - <official-tag> exists on the upstream remote
  - prerelease tags are rejected
USAGE
}

fail() {
  echo "error: $*" >&2
  exit 1
}

info() {
  echo "==> $*"
}

require_git_repo() {
  git rev-parse --git-dir >/dev/null 2>&1 || fail "must be run inside a Git repository"
}

require_clean_worktree() {
  if [[ -n "$(git status --porcelain)" ]]; then
    fail "working tree is dirty; commit or stash changes before syncing upstream"
  fi
}

require_attached_head() {
  local branch
  branch="$(git branch --show-current)"
  if [[ -z "$branch" ]]; then
    fail "detached HEAD is not allowed; switch to the maintenance branch first"
  fi
}

require_upstream_remote() {
  git remote get-url upstream >/dev/null 2>&1 || fail "missing upstream remote"
}

require_valid_tag_ref() {
  local tag="$1"
  [[ -n "$tag" ]] || fail "tag must not be empty"
  [[ "$tag" != -* ]] || fail "tag must not start with '-'"
  git check-ref-format "refs/tags/$tag" >/dev/null 2>&1 || fail "invalid tag name: $tag"
}

reject_prerelease_tag() {
  local tag="$1"
  local lower="${tag,,}"

  if [[ "$lower" =~ (^|[^[:alnum:]])(alpha|beta|rc|pre|preview|canary|nightly|dev)([^[:alnum:]]|[0-9]|$) ]]; then
    fail "prerelease tag is not allowed: $tag"
  fi

  if [[ "$lower" =~ v[0-9]+([.][0-9]+)*(a|b|alpha|beta|rc|pre|preview)[0-9]+ ]]; then
    fail "compact prerelease tag is not allowed: $tag"
  fi

  if [[ "$lower" =~ -[^/]*(alpha|beta|rc|pre|preview|canary|nightly|dev) ]]; then
    fail "tag appears to contain prerelease semantics after '-': $tag"
  fi
}

require_official_upstream_tag() {
  local tag="$1"
  if ! git ls-remote --exit-code --tags upstream "refs/tags/$tag" >/dev/null 2>&1; then
    fail "tag '$tag' was not found on the upstream remote"
  fi
}

fetch_upstream_tag() {
  local tag="$1"
  info "Fetching upstream tag $tag"
  git fetch --no-tags upstream "refs/tags/$tag:refs/tags/$tag"
}

create_sync_branch() {
  local tag="$1"
  local branch="sync/$tag"

  git check-ref-format "refs/heads/$branch" >/dev/null 2>&1 || fail "tag cannot be used as sync branch name: $tag"
  if git show-ref --verify --quiet "refs/heads/$branch"; then
    fail "local branch already exists: $branch"
  fi

  info "Creating local branch $branch"
  git switch -c "$branch"
}

merge_upstream_tag() {
  local tag="$1"
  info "Merging upstream tag $tag with --no-ff"

  if git merge --no-ff --no-edit "$tag"; then
    info "Merge completed. Review locally; do not push until the maintenance flow says so."
    return 0
  fi

  cat >&2 <<'CONFLICT'

Merge stopped, likely due to conflicts. The working tree is preserved.
Recommended next steps:
  1. Inspect conflicts with git status.
  2. Resolve files manually; git rerere can help reuse prior resolutions.
  3. Run the targeted checks from the maintenance docs.
  4. Commit the merge locally after conflicts are resolved.

This script did not push, create a PR, tag, or release anything.
CONFLICT
  exit 1
}

main() {
  if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
  fi

  [[ "$#" -eq 1 ]] || { usage >&2; exit 2; }

  local tag="$1"
  require_git_repo
  require_clean_worktree
  require_attached_head
  require_upstream_remote
  require_valid_tag_ref "$tag"
  reject_prerelease_tag "$tag"
  require_official_upstream_tag "$tag"
  fetch_upstream_tag "$tag"
  create_sync_branch "$tag"
  merge_upstream_tag "$tag"
}

main "$@"
