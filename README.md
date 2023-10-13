# DNS-Sinky

A simple [DNS sinkhole](https://en.wikipedia.org/wiki/DNS_sinkhole) implemented in Rust. It works like a simpler version of [pi-hole](https://github.com/pi-hole/pi-hole) or [AdGuardHome](https://github.com/AdguardTeam/AdguardHome).

## Features

DNS-Sinky has the following features:

* Blocking of DNS requests whose domain names are contained in blacklists.

* Configuration of blacklists in `config.json` file.

* Downloading and local storage of DNS blacklists.

* Logging DNS request activity to terminal.

## Installation

You can build the project yourself using the Rust compiler:

```bash
cargo build --release
```

Alternatively you can download a pre-compiled version under releases.

## Usage

If you have the Rust compiler installed you can simply build and run with: 

```bash
cargo run --release
```

If you downloaded the pre-compiled files, simply run the `dns_sinky.exe` file.

## Configuration

The DNS sinkhole can be configured by changing the values in the `config.json` file.

* You can change the urls of the DNS blacklists by changing the `urls` field. The default blacklists are from [firebog.net](https://firebog.net/) where the ticked lists are used.

* You can automatically update the lists on startup by activating the `update_on_startup` value. The default setting is `false`.

* You can change the upstream DNS server with the `upstream_dns` value. The default address is `1.1.1.1` by [Cloudflare](https://en.wikipedia.org/wiki/1.1.1.1).
