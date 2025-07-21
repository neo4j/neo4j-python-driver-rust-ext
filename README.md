# Rust Extensions for a Faster Neo4j Bolt Driver for Python

This project contains Rust extensions to speed up the [official Python driver for Neo4j](https://github.com/neo4j/neo4j-python-driver).

The exact speedup depends on the use-case but has been measured to be up to 10x faster.
Use-cases moving only few but big records out of the DBMS tend to benefit the most.


## Installation
Adjust your dependencies (`requirements.txt`, `pyproject.toml` or similar) like so:
```
# remove:
# neo4j == X.Y.Z  # needs to be at least 5.14.1 for a matching Rust extension to exist
# add:
neo4j-rust-ext == X.Y.Z.*
```

I.e., install the same version of `neo4j-rust-ext` as you would install of `neo4j` (except for the last segment which is used for patches of this library).  
That's it!
You don't have to change your code but can use the driver as you normally would.
This package will install the driver as its dependency and then inject itself in a place where the driver can find it and pick it up.

N.B., since the driver is a simple Python dependency of this package, you can also manually install/specify both packages at the same time without issues.
However, make sure the versions match if you do so or leave the version of one of the two unspecified to let the package manager pick a compatible version for you (resolution might be slow, however).

If you experience issues with the driver, consider troubleshooting without the Rust extension first.
For that, simply make sure you haven't installed `neo4j-rust-ext` but *only* `neo4j`.

> [!NOTE]
> For pre-release versions of the driver (alpha, beta, etc.), the version scheme is slightly different:  
> `neo4j == X.Y.ZaA` (for alpha - `a` is not a variable number but a literal) needs to be replaced with `neo4j-rust-ext == X.Y.Z.NaA` where `N` needs to be explicitly specified and cannot be `*`.
> This is a limitation of allowed version identifiers in the Python packaging ecosystem.


## Requirements
For many operating systems and architectures, the pre-built wheels will work out of the box.
If they don't, pip (or any other Python packaging front-end) will try to build the extension from source.
Here's what you'll need for this:
 * Rust 1.77 or later:  
   https://www.rust-lang.org/tools/install
 * Further build tools (depending on the platform).  
   E.g., `gcc` on Ubuntu: `sudo apt install gcc`
