# jsoncrawler

Json web crawler

## Getting Started

To run the program make sure to have [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) installed.

1. `cargo run -r`

If you need to enable logs add the flag `RUST_LOG=info` ex: `RUST_LOG=info cargo run -r`.

## About

This is a fs crawler that handles a list of urls to store contents as json.

## FS

The following auto-generated files are set inside the `.gitignore`.

```
output.jsonl
domains.txt
headers.txt
all-others.txt
connection_error.txt
ok-not_valid_json.txt
ok-valid_json.txt
```
