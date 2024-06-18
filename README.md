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


## How to set local git config 

To set Git configuration for your local project, you need to specify the configuration settings that should apply only to your project, rather than globally across all your projects. Here’s how you can do it:

### Viewing Local Configuration

To view the current configuration settings for your local project, use the following command:
```sh
git config --local --list
```

This command will display all configuration settings that are specific to your current project.

### Editing the Local Configuration File Directly

You can also directly edit the `.git/config` file in your project directory to add or modify settings. This file is in INI format. Here’s an example of what it might look like:

```ini
# ...
[user]
  name = zwpdbh
  email = hyperion_z@outlook.com
```

### Summary

By setting your Git configuration locally, you ensure that these settings apply only to your specific project, allowing you to customize behavior on a per-project basis. Use the `git config --local` command to set these configurations, or edit the `.git/config` file directly for more advanced setups.