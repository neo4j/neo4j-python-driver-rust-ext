#!/usr/bin/env bash
set -e

version="$1"; shift

sed -i "s/\"\(neo4j\(\[[^]]\+\]\)\? *== *\)[^\"]\+\"/\"\1$version\"/" pyproject.toml
sed -i "s/\(version *= *\)\"[0-9]\+\.[0-9]\+\.[0-9]\+\(.*\)\"/\1\"$version.0\"/" pyproject.toml

cd driver
git fetch origin
git checkout "$version"
git pull origin "$version"
cd ..

towncrier create -c "Target driver version ${version}<ISSUES_LIST>.
" "+.feature.md"
echo "=== Please rename the changelog file to match the PR number. ==="
