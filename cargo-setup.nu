#!/usr/bin/env nu

def main [nushell: path]: nothing -> nothing {
    let version = open ($nushell | path join "Cargo.toml") | get package.version

    open Cargo.toml
        | update dependencies.nu-plugin {
            path: ($nushell | path join "crates" "nu-plugin")
            version: $version
        }
        | update dependencies.nu-protocol {
            path: ($nushell | path join "crates" "nu-protocol")
            version: $version
            features: ["plugin"]
        }
        | save --force Cargo.toml
}
