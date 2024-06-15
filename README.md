# Workspace for setup frontend

## How to proxy crates installtion

In workspace `.cargo/config.toml`, edit to add: 

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

## Troubleshooting 

- "Blocking waiting for file lock on package cache"

```sh
cargo clean
rm -rf ~/.cargo/registry/index/*
rm -rf ~/.cargo/.package-cache
```
