# mtoc

A Markdown ([CommonMark]) Table of Contents generator for your _README_,
_CHANGELOG_, or whatever you want!

|                  |                                                         |
| ---------------: | ------------------------------------------------------- |
|               CI | [![CI Status][badge-ci-overall]][ci]                    |
|   Latest Version | [![Latest version][badge-version]][crate]               |
|    Documentation | [![Documentation][badge-docs]][docs]                    |
|  Crate Downloads | [![Crate downloads][badge-crate-dl]][crate]             |
| GitHub Downloads | [![GitHub downloads][badge-github-dl]][github-releases] |
|          License | [![Crate license][badge-license]][github]               |

**Table of Contents**

<!-- toc -->

- [CI Status](#ci-status)
  - [Build (master branch)](#build-master-branch)
  - [Test (master branch)](#test-master-branch)
  - [Check (master branch)](#check-master-branch)
- [Code of Conduct](#code-of-conduct)
- [License](#license)
  - [Contribution](#contribution)

<!-- tocstop -->

_TODO:_ Document and implement application! In the meantime, check out the
[`mtoc-parser`](mtoc-parser/README.md) library.

## CI Status

### Build (master branch)

| Operating System | Stable Rust                                                             | Nightly Rust                                                              | Oldest Rust                                                             |
| ---------------: | ----------------------------------------------------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
|          FreeBSD | [![FreeBSD Stable Build Status][badge-stable_freebsd-build]][ci-master] | [![FreeBSD Nightly Build Status][badge-nightly_freebsd-build]][ci-master] | [![FreeBSD Oldest Build Status][badge-oldest_freebsd-build]][ci-master] |
|            Linux | [![Linux Stable Build Status][badge-stable_linux-build]][ci-master]     | [![Linux Nightly Build Status][badge-nightly_linux-build]][ci-master]     | [![Linux Oldest Build Status][badge-oldest_linux-build]][ci-master]     |
|            macOS | [![macOS Stable Build Status][badge-stable_macos-build]][ci-master]     | [![macOS Nightly Build Status][badge-nightly_macos-build]][ci-master]     | [![macOS Oldest Build Status][badge-oldest_macos-build]][ci-master]     |
|          Windows | [![Windows Stable Build Status][badge-stable_windows-build]][ci-master] | [![Windows Nightly Build Status][badge-nightly_windows-build]][ci-master] | [![Windows Oldest Build Status][badge-oldest_windows-build]][ci-master] |

### Test (master branch)

| Operating System | Stable Rust                                                           | Nightly Rust                                                            | Oldest Rust                                                           |
| ---------------: | --------------------------------------------------------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------- |
|          FreeBSD | [![FreeBSD Stable Test Status][badge-stable_freebsd-test]][ci-master] | [![FreeBSD Nightly Test Status][badge-nightly_freebsd-test]][ci-master] | [![FreeBSD Oldest Test Status][badge-oldest_freebsd-test]][ci-master] |
|            Linux | [![Linux Stable Test Status][badge-stable_linux-test]][ci-master]     | [![Linux Nightly Test Status][badge-nightly_linux-test]][ci-master]     | [![Linux Oldest Test Status][badge-oldest_linux-test]][ci-master]     |
|            macOS | [![macOS Stable Test Status][badge-stable_macos-test]][ci-master]     | [![macOS Nightly Test Status][badge-nightly_macos-test]][ci-master]     | [![macOS Oldest Test Status][badge-oldest_macos-test]][ci-master]     |
|          Windows | [![Windows Stable Test Status][badge-stable_windows-test]][ci-master] | [![Windows Nightly Test Status][badge-nightly_windows-test]][ci-master] | [![Windows Oldest Test Status][badge-oldest_windows-test]][ci-master] |

### Check (master branch)

|        | Status                                            |
| ------ | ------------------------------------------------- |
| Lint   | [![Lint Status][badge-check-lint]][ci-master]     |
| Format | [![Format Status][badge-check-format]][ci-master] |

## Code of Conduct

This project follows the [Rust Code of Conduct][code-of-conduct].

## License

Licensed under either of

- The Apache License, Version 2.0 ([LICENSE-APACHE][license-apachev2] or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- The MIT license ([LICENSE-MIT][license-mit] or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[badge-check-format]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=check&script=format
[badge-check-lint]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=check&script=lint
[badge-ci-overall]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square
[badge-crate-dl]: https://img.shields.io/crates/d/mtoc.svg?style=flat-square
[badge-docs]: https://docs.rs/mtoc/badge.svg?style=flat-square
[badge-github-dl]:
  https://img.shields.io/github/downloads/fnichol/mtoc/total.svg?style=flat-square
[badge-license]: https://img.shields.io/crates/l/mtoc.svg?style=flat-square
[badge-nightly_freebsd-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_freebsd&script=build
[badge-nightly_freebsd-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_freebsd&script=test
[badge-nightly_linux-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_linux&script=build
[badge-nightly_linux-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_linux&script=test
[badge-nightly_macos-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_macos&script=build
[badge-nightly_macos-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_macos&script=test
[badge-nightly_windows-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_windows&script=build
[badge-nightly_windows-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_nightly_windows&script=test
[badge-oldest_freebsd-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_freebsd&script=build
[badge-oldest_freebsd-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_freebsd&script=test
[badge-oldest_linux-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_linux&script=build
[badge-oldest_linux-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_linux&script=test
[badge-oldest_macos-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_macos&script=build
[badge-oldest_macos-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_macos&script=test
[badge-oldest_windows-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_windows&script=build
[badge-oldest_windows-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_1.34.0_windows&script=test
[badge-stable_freebsd-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_freebsd&script=build
[badge-stable_freebsd-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_freebsd&script=test
[badge-stable_linux-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_linux&script=build
[badge-stable_linux-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_linux&script=test
[badge-stable_macos-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_macos&script=build
[badge-stable_macos-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_macos&script=test
[badge-stable_windows-build]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_windows&script=build
[badge-stable_windows-test]:
  https://img.shields.io/cirrus/github/fnichol/mtoc.svg?style=flat-square&task=test_stable_windows&script=test
[badge-version]: https://img.shields.io/crates/v/mtoc.svg?style=flat-square
[ci]: https://cirrus-ci.com/github/fnichol/mtoc
[ci-master]: https://cirrus-ci.com/github/fnichol/mtoc/master
[code-of-conduct]: https://www.rust-lang.org/policies/code-of-conduct
[commonmark]: https://commonmark.org/
[crate]: https://crates.io/crates/mtoc
[docs]: https://docs.rs/mtoc
[github-releases]: https://github.com/fnichol/mtoc/releases
[github]: https://github.com/fnichol/mtoc
[license-apachev2]: https://github.com/fnichol/mtoc/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/fnichol/mtoc/blob/master/LICENSE-MIT
