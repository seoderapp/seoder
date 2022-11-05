# seoder

A performant marketing tool to determine relevant keywords for your next campaign.

## Getting Started

To run the program make sure to have [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed.

1. `RUST_LOG=info cargo run --package seoder -r`

or example with generated exposed connections for ss.

1. `WS_CONNECTION=ws://123.2423.21.213:8080 cargo run --package seoder -r -- 0.0.0.0:8080 0.0.0.0 3000`

If you need to enable logs add the flag `RUST_LOG=info` ex: `RUST_LOG=info cargo run -r`.

The central `urls-input.txt` file is the base list input for crawls.

The server binds all on the `0.0.0.0` interface. To not not bind all change the `seoder` ports established at `0.0.0.0`.
You can also use `cargo run --package seoder -r -- 127.0.0.1:8080 0.0.0.0 3000` to bind the server locally and not the crawler.

The first param is the socket server:port, web http server, the web http port. Change between `0.0.0.0` and `127.0.0.1` depending on your exposure.

To adjust the client static resources for connecting to the WSS server use the env variable `WS_CONNECTION` followed by the full url like the following

`WS_CONNECTION=ws://123.2423.21.213:8080 cargo run --package seoder -r -- 0.0.0.0:8080 0.0.0.0 3000`.

## Application

Make sure to have `tauri` installed - (ex: cargo install tauri-cli);

If you want to run the native app run `SEODER_PROGRAM=app RUST_LOG=info cargo tauri dev` or `./dev.sh`.

## Installation

A valid C compiler is required to build the crate.

### Ubuntu

On Ubuntu the following is required:

1. `sudo apt update`
1. `apt-get install build-essential libssl-dev pkg-config`
1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. `source ~/.bashrc`
1. `rustup update`

### Debian

In order to run the app on debian the following deps are required:

1. `apt-get update && apt-get install -y --no-install-recommends gcc cmake libc6 openssl libssl-dev npm pkg-config g++ ca-certificates`
1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. `source ~/.bashrc`
1. `rustup update`

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
```

## Config

Example configuration file to adjust query type with.
The left hand of the text line between the space is the
config `key` and the right is the `value`.

`config.txt`

```
query users
timeout 15
license false
buffer false
```

1. query - the API path either like `posts` or `users`.
1. timeout - the max time a req can take.
1. buffer - the channel buffer limit to control memory crashes.

### Env

The `ENGINE_FD` env variable enables the custom engine outside the base json crawler:

example `ENGINE_FD=true cargo run -r`

## Proxies

You can add proxies to the client by using the following:

`proxies.txt`

```
someproxy.com
```

## Benches

Run benchmarks by using `cargo bench`.

## Web

To run the web server use the command `RUST_LOG=info cargo run --package seoder_web`.

### Browser

Navigate to `127.0.0.1:3000` to view the UI panel for the project.

## Releases

We use the `publish.yml` config to release on Github actions across all platforms.

## Deploying

If you need to deploy locally run `./build.sh`. This script will build the Mac m1 and universal builds and copy it to the marketing public directory.
