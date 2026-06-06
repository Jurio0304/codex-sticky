# Codex Sticky Maintenance

Codex Sticky is an unofficial lightweight fork of `openai/codex`; it is not
maintained, sponsored, or endorsed by OpenAI. Maintenance should keep a thin
patchset and sync only at planned stable release points.

## Branch Roles

- `origin/main`: Sticky long-term maintenance branch and public fork state.
- `upstream/main`: OpenAI baseline for comparison and tag verification.
- `sync/<tag>`: local temporary branch for one stable upstream tag sync; do not
  push it by default.

## Sync Policy

Prefer official stable release tags instead of continuously chasing
`upstream/main`. Use `upstream/main` to inspect history and conflicts, then merge
the selected stable tag.

The current Sticky `main` is an initial migration from `upstream/main` at
`55aa071b17c825bdb66fac99cde2e7a7acfbdee7`; it should not be presented as a
standard release based on `rust-v0.137.0`. The `0.137.0-sticky.1` release should
be prepared on `release/0.137.0-sticky.1` from `rust-v0.137.0`, with only the
minimal Sticky patchset transplanted; do not tag the current `main` directly.

Enable recorded conflict reuse before the first sync:

```bash
git config rerere.enabled true
```

`git rerere` reduces repeat conflict cost, but every reused resolution still
needs review.

## Stable Tag Flow

From a clean `main` checkout:

```bash
git switch main
git status --short
git fetch upstream --tags
bash scripts/sticky/sync-upstream.sh <official-tag>
```

The script only creates local `sync/<tag>` and runs `git merge --no-ff`. On
conflict it preserves the worktree; resolve manually, review the diff, then
commit the merge locally.

After resolving or completing the merge, run targeted checks:

```bash
just fmt
just test -p codex-tui
cargo build --release --bin codex
```

If common, core, or protocol crates changed, follow the repo's broader test
guidance before updating `origin/main`.

Recommended first formal release flow:

```text
git fetch upstream --tags
-> git switch -c release/0.137.0-sticky.1 rust-v0.137.0
-> transplant the minimal Sticky patchset
-> test and package locally
-> create and push 0.137.0-sticky.1 only after review
-> draft a GitHub Release manually
-> upload the tar.gz archive and SHA256SUMS
-> publish after verifying the draft assets
```

Releases are built locally and uploaded manually. The former GitHub Actions release workflow has been removed. Current published artifacts are limited to the Linux x86_64 GNU
package.

Do not create a release tag from the current initial migration, and do not name
the current `main` code `0.137.0-sticky.1`. Later releases should explicitly
sync an official stable tag before creating `<upstream-version>-sticky.<n>`. Do
not overwrite an already published tag; use the next Sticky revision for rebuilds.

## Baseline Updates

After a reviewed sync, update `.sticky/upstream-base.json` with the synced ref,
synced upstream commit, GitHub latest stable release tag/version, sync mode, and
new Sticky `origin/main` SHA after the reviewed update is pushed.
