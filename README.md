# jsoncrawler

Json web crawler

## Getting Started

To run the program make sure to have [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed.

1. `cargo run -r`

If you need to enable logs add the flag `RUST_LOG=info` ex: `RUST_LOG=info cargo run -r`.

## Installation

A valid C compiler is required to build the crate.

### Ubunutu

On Ubuntu the following is required:

1. build-tools
1. libssl-dev
1. pkg-config

## Building Ubuntu

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

## FS

The following auto-generated files are set inside the `.gitignore`.

```
output.jsonl
headers.txt
all-others.txt
connection_error.txt
ok-not_valid_json.txt
ok-valid_json.txt
```

The config file is also set to be untracked.

```
config.txt
```

## Config

Example configuration file to adjust query type

config.txt:

```
query users
```

## Benches

Run benchmarks by using `cargo bench`.
