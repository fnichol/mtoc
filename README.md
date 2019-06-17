# mtoc

A Markdown ([CommonMark]) Table of Contents generator for your _README_,
_CHANGELOG_, or whatever you want!

|                  |                                                         |
| ---------------: | ------------------------------------------------------- |
|               CI | [![CI Status][badge-overall]][ci]                       |
|   Latest Version | [![Latest version][badge-version]][crate]               |
|    Documentation | [![Documentation][badge-docs]][docs]                    |
|  Crate Downloads | [![Crate downloads][badge-crate-dl]][crate]             |
| GitHub Downloads | [![GitHub downloads][badge-github-dl]][github-releases] |
|          License | [![Crate license][badge-license]][github]               |

**Table of Contents**

<!-- toc -->

- [CI Status](#ci-status)
  * [Build](#build)
  * [Test](#test)
  * [Check](#check)
- [License](#license)
- [Contribution](#contribution)

<!-- tocstop -->

_TODO:_ Document and implement application! In the meantime, check out the
[`mtoc-parser`](mtoc-parser/README.md) library.

## CI Status

### Build

| Operating System | Stable Rust                                                      | Nightly Rust                                                       | Oldest Rust                                                      |
| ---------------: | ---------------------------------------------------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------- |
|          FreeBSD | [![FreeBSD Stable Build Status][badge-stable_freebsd-build]][ci] | [![FreeBSD Nightly Build Status][badge-nightly_freebsd-build]][ci] | [![FreeBSD Oldest Build Status][badge-oldest_freebsd-build]][ci] |
|            Linux | [![Linux Stable Build Status][badge-stable_linux-build]][ci]     | [![Linux Nightly Build Status][badge-nightly_linux-build]][ci]     | [![Linux Oldest Build Status][badge-oldest_linux-build]][ci]     |
|            macOS | [![macOS Stable Build Status][badge-stable_macos-build]][ci]     | [![macOS Nightly Build Status][badge-nightly_macos-build]][ci]     | [![macOS Oldest Build Status][badge-oldest_macos-build]][ci]     |
|          Windows | [![Windows Stable Build Status][badge-stable_windows-build]][ci] | [![Windows Nightly Build Status][badge-nightly_windows-build]][ci] | [![Windows Oldest Build Status][badge-oldest_windows-build]][ci] |

### Test

| Operating System | Stable Rust                                                    | Nightly Rust                                                     | Oldest Rust                                                    |
| ---------------: | -------------------------------------------------------------- | ---------------------------------------------------------------- | -------------------------------------------------------------- |
|          FreeBSD | [![FreeBSD Stable Test Status][badge-stable_freebsd-test]][ci] | [![FreeBSD Nightly Test Status][badge-nightly_freebsd-test]][ci] | [![FreeBSD Oldest Test Status][badge-oldest_freebsd-test]][ci] |
|            Linux | [![Linux Stable Test Status][badge-stable_linux-test]][ci]     | [![Linux Nightly Test Status][badge-nightly_linux-test]][ci]     | [![Linux Oldest Test Status][badge-oldest_linux-test]][ci]     |
|            macOS | [![macOS Stable Test Status][badge-stable_macos-test]][ci]     | [![macOS Nightly Test Status][badge-nightly_macos-test]][ci]     | [![macOS Oldest Test Status][badge-oldest_macos-test]][ci]     |
|          Windows | [![Windows Stable Test Status][badge-stable_windows-test]][ci] | [![Windows Nightly Test Status][badge-nightly_windows-test]][ci] | [![Windows Oldest Test Status][badge-oldest_windows-test]][ci] |

### Check

|        | Status                                     |
| ------ | ------------------------------------------ |
| Lint   | [![Lint Status][badge-check-lint]][ci]     |
| Format | [![Format Status][badge-check-format]][ci] |

## License

Licensed under either of

- The Apache License, Version 2.0 ([LICENSE-APACHE][license-apachev2] or
  http://www.apache.org/licenses/LICENSE-2.0)
- The MIT license ([LICENSE-MIT][license-mit] or
  http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[badge-check-format]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=check&script=format
[badge-check-lint]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=check&script=lint
[badge-crate-dl]: https://img.shields.io/crates/d/mtoc.svg
[badge-docs]: https://docs.rs/mtoc/badge.svg
[badge-github-dl]:
  https://img.shields.io/github/downloads/fnichol/mtoc/total.svg
[badge-license]: https://img.shields.io/crates/l/mtoc.svg
[badge-nightly_freebsd-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_freebsd&script=build
[badge-nightly_freebsd-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_freebsd&script=test
[badge-nightly_linux-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_linux&script=build
[badge-nightly_linux-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_linux&script=test
[badge-nightly_macos-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_macos&script=build
[badge-nightly_macos-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_macos&script=test
[badge-nightly_windows-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_windows&script=build
[badge-nightly_windows-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_nightly_windows&script=test
[badge-oldest_freebsd-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_freebsd&script=build
[badge-oldest_freebsd-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_freebsd&script=test
[badge-oldest_linux-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_linux&script=build
[badge-oldest_linux-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_linux&script=test
[badge-oldest_macos-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_macos&script=build
[badge-oldest_macos-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_macos&script=test
[badge-oldest_windows-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_windows&script=build
[badge-oldest_windows-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_1.34.0_windows&script=test
[badge-overall]: https://api.cirrus-ci.com/github/fnichol/mtoc.svg
[badge-stable_freebsd-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_freebsd&script=build
[badge-stable_freebsd-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_freebsd&script=test
[badge-stable_linux-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_linux&script=build
[badge-stable_linux-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_linux&script=test
[badge-stable_macos-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_macos&script=build
[badge-stable_macos-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_macos&script=test
[badge-stable_windows-build]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_windows&script=build
[badge-stable_windows-test]:
  https://api.cirrus-ci.com/github/fnichol/mtoc.svg?task=test_stable_windows&script=test
[badge-version]: https://img.shields.io/crates/v/mtoc.svg
[ci]: https://cirrus-ci.com/github/fnichol/mtoc
[commonmark]: https://commonmark.org/
[crate]: https://crates.io/crates/mtoc
[docs]: https://docs.rs/mtoc
[github-releases]: https://github.com/fnichol/mtoc/releases
[github]: https://github.com/fnichol/mtoc
[license-apachev2]: https://github.com/fnichol/mtoc/blob/master/LICENSE-APACHE
[license-mit]: https://github.com/fnichol/mtoc/blob/master/LICENSE-MIT
