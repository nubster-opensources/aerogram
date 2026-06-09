#!/usr/bin/env bash
#
# scripts/release.sh: opinionated release preparation wrapper for Aerogram.
#
# Usage:
#   ./scripts/release.sh <patch|minor|major|x.y.z>
#
# What it does (in order):
#   1. Pre-flight: must be on main, working tree must be clean, main must
#      be in sync with origin/main.
#   2. Computes the target version from the requested bump level.
#   3. Creates branch release/v<TARGET>-prep.
#   4. Graduates CHANGELOG.md: moves [Unreleased] body under a new
#      [<TARGET>] - <DATE> section, refreshes the link refs.
#   5. Runs cargo-release to bump every Cargo.toml version field (path-
#      deps included) in a single commit.
#   6. Pre-flight checks: cargo fmt --check, clippy strict, full test suite.
#   7. Pushes the branch and opens a pull request via gh.
#
# After the human reviews and merges the PR, they push the tag manually:
#
#   git tag -a v<TARGET> -m "v<TARGET>"
#   git push origin v<TARGET>
#
# That tag triggers .github/workflows/release.yml which actually publishes
# to crates.io and creates the GitHub Release.
#
# Dependencies on the runner: bash, git, cargo, cargo-release, gh, python3.

set -euo pipefail

LEVEL="${1:-}"
if [[ -z "${LEVEL}" ]]; then
  cat >&2 <<'USAGE'
Usage: scripts/release.sh <patch|minor|major|x.y.z>

Examples:
  scripts/release.sh patch          # 0.1.0 -> 0.1.1
  scripts/release.sh minor          # 0.1.0 -> 0.2.0
  scripts/release.sh major          # 1.2.3 -> 2.0.0
  scripts/release.sh 0.3.0          # explicit
USAGE
  exit 1
fi

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd "${REPO_ROOT}"

# 1. Pre-flight
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "${CURRENT_BRANCH}" != "main" ]]; then
  echo "error: must be on main (current: ${CURRENT_BRANCH})" >&2
  exit 1
fi
if [[ -n "$(git status --porcelain)" ]]; then
  echo "error: working tree must be clean" >&2
  git status --short >&2
  exit 1
fi
git pull --ff-only origin main

# 2. Compute target version
CURRENT_VERSION="$(grep -m1 -E '^version = "[0-9]+\.[0-9]+\.[0-9]+"' Cargo.toml | sed -E 's/^version = "([^"]+)".*/\1/')"
if [[ -z "${CURRENT_VERSION}" ]]; then
  echo "error: cannot read current workspace version from Cargo.toml" >&2
  exit 1
fi

if [[ "${LEVEL}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  NEW_VERSION="${LEVEL}"
else
  case "${LEVEL}" in
    patch)
      NEW_VERSION="$(python3 -c "import sys; v=sys.argv[1].split('.'); v[2]=str(int(v[2])+1); print('.'.join(v))" "${CURRENT_VERSION}")"
      ;;
    minor)
      NEW_VERSION="$(python3 -c "import sys; v=sys.argv[1].split('.'); v[1]=str(int(v[1])+1); v[2]='0'; print('.'.join(v))" "${CURRENT_VERSION}")"
      ;;
    major)
      NEW_VERSION="$(python3 -c "import sys; v=sys.argv[1].split('.'); v[0]=str(int(v[0])+1); v[1]='0'; v[2]='0'; print('.'.join(v))" "${CURRENT_VERSION}")"
      ;;
    *)
      echo "error: unknown level '${LEVEL}' (use patch|minor|major or x.y.z)" >&2
      exit 1
      ;;
  esac
fi

echo "Bumping ${CURRENT_VERSION} -> ${NEW_VERSION}"

# 3. Create release branch
BRANCH="release/v${NEW_VERSION}-prep"
if git show-ref --quiet --verify "refs/heads/${BRANCH}"; then
  echo "error: branch ${BRANCH} already exists locally" >&2
  exit 1
fi
git checkout -b "${BRANCH}"

# 4. Graduate CHANGELOG.md
DATE="$(date -u +%Y-%m-%d)"
python3 - "${NEW_VERSION}" "${DATE}" <<'PYEOF'
import pathlib
import re
import sys

new_version = sys.argv[1]
date = sys.argv[2]
path = pathlib.Path("CHANGELOG.md")
content = path.read_text(encoding="utf-8")

unreleased_re = re.compile(r"## \[Unreleased\]\n\n(.*?)\n## \[", re.DOTALL)
m = unreleased_re.search(content)
if m is None:
    raise SystemExit("CHANGELOG.md does not contain a [Unreleased] section")

placeholder_body = "### Added\n- _Items in flight will be listed here until the next release._\n"
existing_body = m.group(1).strip("\n")

if existing_body in {"", placeholder_body.strip("\n")}:
    graduated_body = "_See diff against the previous tag for details._\n"
else:
    graduated_body = existing_body + "\n"

new_block = (
    "## [Unreleased]\n\n"
    + placeholder_body
    + "\n"
    + "## [" + new_version + "] - " + date + "\n\n"
    + graduated_body
    + "\n## ["
)
content = unreleased_re.sub(lambda _: new_block, content, count=1)

unreleased_link_re = re.compile(r"^\[Unreleased\]: .*$", re.MULTILINE)
new_links = (
    "[Unreleased]: https://github.com/nubster-opensources/aerogram/compare/v"
    + new_version
    + "...HEAD\n["
    + new_version
    + "]: https://github.com/nubster-opensources/aerogram/releases/tag/v"
    + new_version
)
content = unreleased_link_re.sub(new_links, content, count=1)

path.write_text(content, encoding="utf-8")
print(f"CHANGELOG.md graduated: [Unreleased] -> [{new_version}] - {date}")
PYEOF

# 5. Bump every Cargo.toml version (including inter-crate path deps)
cargo release "${LEVEL}" --workspace --execute --no-confirm

# 6. Pre-flight checks
echo "Running cargo fmt --check"
cargo fmt --all -- --check
echo "Running cargo clippy --workspace --all-targets --all-features -- -D warnings"
cargo clippy --workspace --all-targets --all-features -- -D warnings
echo "Running cargo test --workspace --all-features"
cargo test --workspace --all-features

# 7. Push branch and open the pull request
git push -u origin "${BRANCH}"

PR_BODY=$(cat <<EOF
Pre-flight release prep generated by scripts/release.sh.

## Pre-flight (passed locally)

- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings
- cargo test --workspace --all-features

## Tagging instructions

After this PR is merged, push the v${NEW_VERSION} tag to fire .github/workflows/release.yml:

\`\`\`
git tag -a v${NEW_VERSION} -m "v${NEW_VERSION}"
git push origin v${NEW_VERSION}
\`\`\`

The release workflow then publishes the crates to crates.io in dependency order and creates the GitHub Release from the CHANGELOG section.
EOF
)

gh pr create \
  --title "release(v${NEW_VERSION}): bump workspace and finalise release notes" \
  --body "${PR_BODY}" \
  --label "kind:chore,phase:release" \
  --base main \
  --head "${BRANCH}"

cat <<EOF

=========================================
Release prep PR opened for v${NEW_VERSION}.

Next steps:
  1. Review and merge the PR.
  2. git tag -a v${NEW_VERSION} -m "v${NEW_VERSION}"
  3. git push origin v${NEW_VERSION}
=========================================
EOF
