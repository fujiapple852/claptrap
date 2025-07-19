---
title: Installation
description: Install Claptrap on your platform.
banner:
  content: 🚧 🚧 🚧 Under construction 🚧 🚧 🚧
sidebar:
  order: 2
---

The following sections provide instructions for installing Claptrap on your platform.

Claptrap runs on Linux, BSD, macOS, and Windows. It can be installed from most common package managers, precompiled
binaries, or source.

## Distributions

Claptrap is available for a variety of platforms and package managers.

### Cargo

[![Crates.io](https://img.shields.io/crates/v/claptrap)](https://crates.io/crates/claptrap/0.1.0)

```shell
cargo install claptrap --locked
```

### PPA (Ubuntu)

[![Ubuntu PPA](https://img.shields.io/badge/Ubuntu%20PPA-0.1.0-brightgreen)](https://launchpad.net/~fujiapple/+archive/ubuntu/claptrap/+packages)

```shell
add-apt-repository ppa:fujiapple/claptrap
apt update && apt install claptrap
```

> ⓘ Note:
>
> Only available for Ubuntu 24.04 (`Noble`) and 22.04 (`Jammy`).

### Snap (Linux)

[![claptrap](https://snapcraft.io/claptrap/badge.svg)](https://snapcraft.io/claptrap)

```shell
snap install claptrap
```

### Homebrew (macOS)

[![homebrew version](https://img.shields.io/badge/homebrew-0.1.0-orange)](https://github.com/fujiapple852/homebrew-claptrap)

```shell
brew tap fujiapple852/claptrap && brew install claptrap
```

### Docker

[![Docker Image Version (latest by date)](https://img.shields.io/docker/v/fujiapple/claptrap)](https://hub.docker.com/r/fujiapple/claptrap/)

```shell
docker run -it fujiapple/claptrap
```

### All Repositories

[![Packaging status](https://repology.org/badge/vertical-allrepos/claptrap.svg)](https://repology.org/project/claptrap/versions)

## Downloads

Download the latest release for your platform.

| OS      | Arch      | Env          | Current (0.1.0)                                                                                                                |
| ------- | --------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| Linux   | `x86_64`  | `gnu`        | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-unknown-linux-gnu.tar.gz)       |
| Linux   | `x86_64`  | `musl`       | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-unknown-linux-musl.tar.gz)      |
| Linux   | `aarch64` | `gnu`        | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-aarch64-unknown-linux-gnu.tar.gz)      |
| Linux   | `aarch64` | `musl`       | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-aarch64-unknown-linux-musl.tar.gz)     |
| Linux   | `arm7`    | `gnueabihf`  | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-armv7-unknown-linux-gnueabihf.tar.gz)  |
| Linux   | `arm7`    | `musleabi`   | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-armv7-unknown-linux-musleabi.tar.gz)   |
| Linux   | `arm7`    | `musleabihf` | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-armv7-unknown-linux-musleabihf.tar.gz) |
| macOS   | `x86_64`  | `darwin`     | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-apple-darwin.tar.gz)            |
| macOS   | `aarch64` | `darwin`     | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-aarch64-apple-darwin.tar.gz)           |
| Windows | `x86_64`  | `msvc`       | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-pc-windows-msvc.zip)            |
| Windows | `x86_64`  | `gnu`        | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-pc-windows-gnu.zip)             |
| Windows | `aarch64` | `msvc`       | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-aarch64-pc-windows-msvc.zip)           |
| FreeBSD | `x86_64`  | n/a          | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-unknown-freebsd.tar.gz)         |
| NetBSD  | `x86_64`  | n/a          | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64-unknown-netbsd.tar.gz)          |
| RPM     | `x86_64`  | `gnu`        | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap-0.1.0-x86_64.rpm)                            |
| Debian  | `x86_64`  | `gnu`        | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap_x86_64-unknown-linux-gnu_0.1.0_amd64.deb)    |
| Debian  | `x86_64`  | `musl`       | [0.1.0](https://github.com/fujiapple852/claptrap/releases/download/0.1.0/claptrap_x86_64-unknown-linux-musl_0.1.0_amd64.deb)   |
