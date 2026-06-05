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
standard release based on `rust-v0.137.0`. Create standardized Sticky releases
only after explicitly syncing a chosen official stable tag.

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

Recommended first standardized release flow:

```text
wait for a new official stable tag
-> scripts/sticky/sync-upstream.sh <stable-tag>
-> resolve conflicts manually
-> test
-> merge to Sticky main
-> create v<upstream-version>-sticky.1
```

Do not create a release tag for the current initial migration, and do not name
the current code `v0.137.0-sticky.1`.

## Baseline Updates

After a reviewed sync, update `.sticky/upstream-base.json` with the synced ref,
synced upstream commit, GitHub latest stable release tag/version, sync mode, and
new Sticky `origin/main` SHA after the reviewed update is pushed.
