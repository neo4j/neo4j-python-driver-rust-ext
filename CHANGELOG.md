Changelog
=========


⚠️ marks breaking changes, pending breaking changes (deprecations), or other critical changes.

<!-- towncrier release notes start -->

## [5.28.2.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.28.2.0) (2025-07-30)
***
### **⭐️ New Features**
* Target driver version 5.28.2 ([#54]).

[#54]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/54

### **👏️ Improvements**
* Update dependencies ([#32]):  
  * ⚠️ Bump minimum supported Rust version (MSRV) from `1.67` to `1.77`.
  * Security fix by bumping PyO3 (Rust binding for Python) from `0.22.4` to `0.24.2`.
  * Update `maturin` (Python package builder) from `~= 1.6.0` to `~= 1.8.3`.
* Harden `Structure` class against memory leak ([#53]).  
  The extensions' implementation of packstream `Structure` could leak memory when being part of a reference cycle.
  In reality this doesn't matter because the driver never constructs cyclic `Structure`s.
  Every packstream value is a tree in terms of references (both directions: packing and unpacking).  
  This change is meant to harden the extensions against introducing effective memory leaks in the driver should the driver's usage of `Structure` change in the future.

[#32]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/32
[#53]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/53

### **📦️ Packaging**
* ⚠️ Change licensing from "Apache-2.0" to "Apache-2.0 AND MIT"  ([#43]).
* Update licensing meta data to be PEP 639 compliant ([#41]).  
  * Update `maturin` (Python package builder) from `~= 1.8.3` to `~= 1.9.0`.
* Update `maturin` (Python package builder) from `~= 1.9.0` to `~= 1.9.1` ([#54]).

[#41]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/41
[#43]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/43
[#54]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/54

### **🧑️‍💻️ Development**
* Introduce [towncrier](https://towncrier.readthedocs.io/) for managing changelog entries ([#43]).

[#43]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/43


## [5.28.1.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.28.1.0) (2025-02-10)
***
* Target driver version 5.28.1


## [5.28.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.28.0.0) (2025-02-05)
***
* Target driver version 5.28.0


## [5.27.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.27.0.0) (2024-11-28)
***
* Target driver version 5.27.0


## [5.26.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.26.0.0) (2024-11-01)
***
* Target driver version 5.26.0
* Add support for Python 3.13
* Bump PyO3 to 0.22.4
* Introduce `isort` and `ruff`


## [5.25.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.25.0.0) (2024-09-26)
***
* Target driver version 5.25.0


## [5.24.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.24.0.0) (2024-08-29)
***
* Target driver version 5.24.0


## [5.23.1.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.23.1.0) (2024-08-05)
***
* Target driver version 5.23.1
* Moved repository:  
  from https://github.com/neo4j-drivers/neo4j-python-driver-rust-ext  
  to https://github.com/neo4j/neo4j-python-driver-rust-ext
* Metadata: removed `Beta` tag, added `Production/Stable`.
* Bump MSRV (minimum supported Rust version) to 1.67.0.
* Clarify installation documentation: `neo4j` and `neo4j-rust-ext` can both be installed at the same time.
  ℹ️ Make sure to specify matching versions if you do so.


## [5.23.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.23.0.0) (2024-07-29)
***
* Target driver version 5.23.0


## [5.22.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.22.0.0) (2024-06-27)
***
* Target driver version 5.22.0


## [5.21.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.21.0.0) (2024-06-11)
***
* Target driver version 5.21.0


## [5.20.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.20.0.0) (2024-04-26)
***
* Target driver version 5.20.0


## [5.19.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.19.0.0) (2024-05-02)
***
* Target driver version 5.19.0


## [5.18.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.18.0.0) (2024-02-29)
***
* Target driver version 5.18.0


## [5.17.0.0b1](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.17.0.0b1) (2024-01-29)
***
* Target driver version 5.17.0


## [5.16.0.0b1](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.16.0.0b1) (2023-12-28)
***
* Target driver version 5.16.0


## [5.15.0.0b1](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.15.0.0b1) (2023-11-28)
***
* Target driver version 5.15.0


## [5.14.1.0a1](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.14.1.0a1) (2023-11-03)
***
* Target driver version 5.14.1
* Initial release.
