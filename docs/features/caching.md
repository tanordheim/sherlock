# Caching

The `caching` key in Sherlock controls the caching of `.desktop` files. This prevents Sherlock from loading them from your `XDG_DATA_DIRS` on every startup, instead loading them from a `json` file located in your specified directory or, by default, in `~/.cache/sherlock_desktop_cache.json`.<br>

However, Sherlock will still monitor the directory for changes. If a new application is installed, it will be detected and displayed accordingly. Similarly, the `sherlock_alias.json` file is checked for modifications to ensure that your aliases are applied correctly.<br>

As of release `v0.1.6`, caching is **enabled by default**.<br>

## How to Enable/Disable Caching

To enable or disable caching in Sherlock, modify the `caching` key in the `behavior` section of the `config.toml` file, setting it to either `true` or `false` as needed.<br>

