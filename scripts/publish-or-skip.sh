#!/usr/bin/env bash
#
# publish-or-skip.sh: idempotent `cargo publish` wrapper used by the
# release workflow.
#
# Usage:
#   scripts/publish-or-skip.sh <crate-name> [extra args] -- <token>
#
# The script:
#   1. Reads the local version of <crate-name> via `cargo pkgid`.
#   2. Checks the crates.io API for that exact version.
#   3. Skips the publish if the version is already on crates.io.
#   4. Otherwise runs `cargo publish -p <crate-name> [extra args] --token <token>`.
#
# Why: the release workflow needs to be re-runnable after a partial
# failure (mid-pipeline timeout, missing intermediate crate, ...).
# Without idempotence, every retry would fail on the already-published
# crates and require a manual version bump.

set -euo pipefail

if [[ $# -lt 3 ]]; then
  echo "usage: $0 <crate-name> [extra-args ...] -- <token>" >&2
  exit 2
fi

crate="$1"
shift

extra_args=()
while [[ $# -gt 0 ]]; do
  if [[ "$1" == "--" ]]; then
    shift
    break
  fi
  extra_args+=("$1")
  shift
done

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <crate-name> [extra-args ...] -- <token>" >&2
  exit 2
fi
token="$1"

# Resolve the local version through `cargo pkgid`. Output format depends
# on the cargo release in use; handle both `name@version` and `...#version`.
pkgid="$(cargo pkgid -p "$crate")"
if [[ "$pkgid" =~ @([0-9]+\.[0-9]+\.[0-9]+([0-9A-Za-z.+-]*)) ]]; then
  version="${BASH_REMATCH[1]}"
elif [[ "$pkgid" =~ \#([0-9]+\.[0-9]+\.[0-9]+([0-9A-Za-z.+-]*))$ ]]; then
  version="${BASH_REMATCH[1]}"
else
  echo "Unable to extract version from cargo pkgid output: $pkgid" >&2
  exit 1
fi

echo "Resolved $crate@$version (pkgid: $pkgid)"

# Probe crates.io for this exact version. The crates.io API rejects
# requests without an explicit User-Agent header (HTTP 403), so we set
# a descriptive one identifying the release pipeline.
url="https://crates.io/api/v1/crates/${crate}/${version}"
user_agent="aerogram-release (https://github.com/nubster-opensources/aerogram)"
status="$(curl -s -A "$user_agent" -o /dev/null -w "%{http_code}" "$url")"

if [[ "$status" == "200" ]]; then
  echo "$crate@$version already published on crates.io, skipping"
  exit 0
fi

# Treat 404 as "not yet published". Anything else is unexpected and
# worth surfacing in the log; we still attempt the publish so that
# `cargo publish` itself gives the authoritative failure.
echo "$crate@$version not on crates.io (HTTP $status), publishing..."
cargo publish -p "$crate" "${extra_args[@]}" --token "$token"
