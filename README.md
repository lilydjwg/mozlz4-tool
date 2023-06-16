# mozlz4-tool

A simple tool to decompress and compress files into the mozlz4 format used by Firefox.

To build, install cargo and run `cargo build --release`.

To use, find the compiled binary (default `target/release/mozlz4-tool`) and run it with your mozlz4 file path.

Use `--help` to see the command usage.

## Nix and NixOS users

The program is available as a flake, so it can be run with

```bash
nix run github:lilydjwg/mozlz4-tool
```
