# nu_plugin_kdl
A plugin to add KDL support to Nushell.

## Build the plugin
```nushell
./cargo-setup.nu /path/to/nushell/nushell/
```
```nushell
cargo build --release
```
```nushell
register target/release/nu_plugin_kdl
```

## Examples
see [`examples.md`](examples.md)

## TODO
- [x] implement `from kdl` and support in `open foo.kdl`
- [ ] implement `to kdl` and support in `save foo.kdl`
- [ ] add tests
- [ ] add proper error support
- [ ] support type annotations
- [ ] preserve comments
