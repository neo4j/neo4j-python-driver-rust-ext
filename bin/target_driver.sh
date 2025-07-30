#!/usr/bin/env bash
set -e

version="$1"; shift

sed -i "s/\"\(neo4j *\([\[0-9a-zA-Z, \]\+]\)\? *== *\).*\"/\"\1$version\"/" pyproject.toml
sed -i "s/\(version *= *\)\"[0-9]\+\.[0-9]\+\.[0-9]\+\(.*\)\"/\1\"$version.0\"/" pyproject.toml

cd driver
git fetch origin
git checkout "$version"
git pull origin "$version"
cd ..
cp driver/tests/unit/common/codec/packstream/v1/test_packstream.py tests/codec/packstream/v1/from_driver/test_packstream.py
cp -r driver/tests/unit/common/vector/* tests/vector/from_driver

towncrier create -c "Target driver version ${version}<ISSUES_LIST>.
" "+.feature.md"
echo "=== Please rename the changelog file to match the PR number. ==="
