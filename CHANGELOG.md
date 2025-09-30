Changelog
=========

⚠️ marks breaking changes, pending breaking changes (deprecations), or other critical changes.

<!-- towncrier release notes start -->

## [6.0.0.0](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/6.0.0.0) (2025-09-30)
***
### **⭐️ New Features**
* Target driver version 6.0.0 ([#67]).

[#67]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/67

### **🔧️ Fixes**
* Fix decoding of map keys of certain sizes ([#59]).

[#59]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/59

### **👏️ Improvements**
* Bump dependency PyO3 (Rust binding for Python) from `0.25.1` to `0.26.0` ([#66]).

[#66]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/66

### **🧹️ Clean-up**
* Improve packstream `Structure` class ([#63]).
  * Implement `repr` to match Python implementation.
  * Remove `__hash__` implementation to match Python implementation.
  * Implement `__getitem__` and `__setitem__` to be on par with Python implementation.
  * Copy tests for `Structure` from the driver project.

[#63]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/63

### **🧑️‍💻️ Development**
* Fix broken `cp` command in `bin/target_driver.sh` ([#67]).

[#67]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/67


## [6.0.0.0a1](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/6.0.0.0a1) (2025-07-29)
***
### **⭐️ New Features**
* Add extension for the `Vector` type ([#45]).
  * Speed up endian conversion (byte flipping).
  * Speed up conversion from and to native python types.
* Target driver version 6.0.0a1 ([#47]).

[#45]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/45
[#47]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/47

### **👏️ Improvements**
* Update dependencies ([#32]):  
  * ⚠️ Bump minimum supported Rust version (MSRV) from `1.67` to `1.77`.
  * Security fix by bumping PyO3 (Rust binding for Python) from `0.22.4` to `0.24.2`.
  * Update `maturin` (Python package builder) from `~= 1.6.0` to `~= 1.8.3`.
* Harden `Structure` class against memory leak ([#50]).  
  The extensions' implementation of packstream `Structure` could leak memory when being part of a reference cycle.
  In reality this doesn't matter because the driver never constructs cyclic `Structure`s.
  Every packstream value is a tree in terms of references (both directions: packing and unpacking).  
  This change is meant to harden the extensions against introducing effective memory leaks in the driver should the driver's usage of `Structure` change in the future.
* Optimize packing of `bytearray` ([#51]).
  By special-casing `bytearray`, we can avoid an allocation and complete extra copy of the data when packing it.
  This speeds up packing of `bytearray`s by roughly 1/3.

[#32]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/32
[#50]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/50
[#51]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/51

### **🧹️ Clean-up**
* ⚠️ Drop support for Python 3.7, 3.8, and 3.9 ([#37]).

[#37]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/37

### **📦️ Packaging**
* Update licensing meta data to be PEP 639 compliant ([#38]).
  * Update `maturin` (Python package builder) from `~= 1.8.3` to `~= 1.9.0`.
* ⚠️ Change licensing from "Apache-2.0" to "Apache-2.0 AND MIT"  ([#40]).
* Update dependencies ([#46]).
  * `PyO3`: `0.24.2` -> `0.25.1`
  * `maturin`: `1.9.0` -> `1.9.1`

[#38]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/38
[#40]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/40
[#46]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/46

### **🧑️‍💻️ Development**
* Introduce [towncrier](https://towncrier.readthedocs.io/) for managing changelog entries ([#40]).
* Use dependency groups in `pyproject.toml` for development dependencies ([#44]).

[#40]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/40
[#44]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/44


## [5.28.2.1](https://github.com/neo4j/neo4j-python-driver-rust-ext/tree/5.28.2.1) (2025-08-15)
***
### **🔧️ Fixes**
* Fix decoding of map keys of certain sizes ([#60]).

[#60]: https://github.com/neo4j/neo4j-python-driver-rust-ext/pull/60


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
