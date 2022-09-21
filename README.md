# jsoncrawler

Json web crawler

## Getting Started

To run the program make sure to have [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed.

1. `cargo run -r`

If you need to enable logs add the flag `RUST_LOG=info` ex: `RUST_LOG=info cargo run -r`.

## Installation

A valid C compiler is required to build the crate.

### Ubuntu

On Ubuntu the following is required:

1. build-tools
1. libssl-dev
1. pkg-config

### Debian

In order to run the app on debian the following deps are required:

1. apt-get update && apt-get install -y --no-install-recommends gcc cmake libc6 openssl libssl-dev npm pkg-config g++ ca-certificates
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
1. rustup update

For feature jemalloc:

1. apt-get install cmake make
1. `export JEMALLOC_SYS_WITH_MALLOC_CONF="background_thread:true,narenas:1,tcache:false,dirty_decay_ms:0,muzzy_decay_ms:0,abort_conf:true"`

#### System Reqs Ubuntu/Debian

In order to perform high concurrency on ubuntu we need to increase some limits.

Run the following command to generate a sh script to excute. If limits are not increased
the application may fail.

```sh
# [required] increase u limits for crawl mode
ulimit -n 999999
# all other options not required but, may be adjusted
ulimit -t unlimited
ulimit -m unlimited
ulimit -f unlimited
ulimit -s unlimited
ulimit -v unlimited
```

## About

This is a fs crawler that handles a list of urls to store contents as json.

## Program Files

The following auto-generated files are set inside the `.gitignore`.

```
output.jsonl
all-others.txt
connection_error.txt
ok-not_valid_json.txt
ok-valid_json.txt
```

## Config

Example configuration file to adjust query type with.
The left hand of the text line between the space is the
config `key` and the right is the `value`.

`config.txt`

```
query users
timeout 15
buffer 100
```

1. query - the API path either like `posts` or `users`.
1. timeout - the max time a req can take.
1. buffer - the channel buffer limit to control memory crashes.

## Proxies

You can add proxies to the client by using the following:

`proxies.txt`

```
someproxy.com
```

## Benches

Run benchmarks by using `cargo bench`.
