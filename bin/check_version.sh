#!/usr/bin/env bash
set -e

version="$1"; shift
version_matches=$(grep -o --perl-regexp '(?m)(?<!.)^\s*version\s*=\s*\"\Q'"$version"'\E\"\s*$(?!.)' pyproject.toml | wc -l)

if [ "$version_matches" -ne 2 ]
then
    echo "Version mismatch in pyproject.toml"
    echo "Trying to release version $version"
    foundVersions=$(sed -nr 's/ *version *= *\"(.*)\"/\1/p' pyproject.toml)
    if [ -z "$foundVersions" ]
    then
        echo "No version found in pyproject.toml"
    else
        echo "Found version(s) in pyproject.toml:"
        echo "$foundVersions"
    fi
    exit 1
fi
