# To Build from Source on a Debian-Based Distro

```
# Used kali-rolling
# First, install rust (hit enter a few times to install defaults)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

# Then restart your shell to pull the new cargo .env file in and then run
apt update
apt install -y libssl-dev pkg-config libsqlite3-dev
```

# Build From Source Using Docker

```
sudo sh ./build-musl.sh
```

# Open Source Client for Check Point VPN Tunnels

[![github actions](https://github.com/ancwrd1/snx-rs/workflows/CI/badge.svg)](https://github.com/ancwrd1/snx-rs/actions)
[![license](https://img.shields.io/badge/License-AGPL-v3.svg)](https://opensource.org/license/agpl-v3)

This project contains the source code for an unofficial client for Check Point VPN, written in Rust.
Currently supported platforms: Linux, Windows.

## Key Features

* IPsec and SSL tunnel support
* Browser-based SSO, username/password, certificate, HSM token and MFA authentication
* GUI frontend with tray icon
* Split DNS for better privacy
* OS keychain integration
* Multiple connection profiles
* Persistent IPsec session for fast reconnect after network drops or suspend/resume — see [`ike-persist`](docs/persistent-ipsec-session.md)

## Package Repository

Signed APT and DNF repositories with the latest release builds are published at [ancwrd1.github.io/snx-rs](https://ancwrd1.github.io/snx-rs/).
The page lists the installation commands for Debian/Ubuntu and Fedora/RHEL/openSUSE.
Only the default (non-webkit) builds are served from the repository; the `-webkit` variant remains available as a direct download from the [Releases page](https://github.com/ancwrd1/snx-rs/releases).

## Documentation

See the full documentation in the [docs](docs/README.md) directory.

## Quick Links

* [Installation](docs/installation.md)
* [Quick Start Guide](docs/quick-start.md)
* [Configuration Options](docs/options.md)
* [Troubleshooting](docs/troubleshooting.md)
* [Building from Sources](docs/building.md)
* [Contributing](docs/contributing.md)

## License

Licensed under the [GNU Affero General Public License version 3](https://opensource.org/license/agpl-v3/).
