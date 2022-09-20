# jsoncrawler

Json web crawler

## Getting Started

To run the program make sure to have [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed.

1. `cargo run -r`

If you need to enable logs add the flag `RUST_LOG=info` ex: `RUST_LOG=info cargo run -r`.

## Installation

On Ubuntu the following is required:

1. build-tools
1. libssl-dev
1. pkg-config

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

## Building Ubuntu

In order to perform high concurrency on ubuntu we need to increase some limits.

Run the following command to generate a sh script to excute.

```sh
 rustc sys_config.rs && ./sys_config
 # unlimit the system without constraints
 ./unlimit
```
