DEEP\_ABYSS
----

Nautical roguelite game for multiple platforms written in rust using
[Bevy](https://bevyengine.org/).

> Still in very early development, nothing to see here yet...

## Development

To build a development build, clone and run:
```sh
cargo run --features bevy/dynamic_linking
```
## Releases

Check the [releases](https://github.com/roosta/deep_abyss/releases) page for pre-compiled binaries.


To build a release binary install [dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) and run this:
```
cargo build --release
```
The binary is located in `target/release/deep_abyss`. Currently you need to run
the binary along side the asset directory, else it will produce an error about
missing assets.

This will be improved later, but the releases are packaged with the assets.


## License

- Copyright (c) 2024 Daniel Berg
- Source code distributed under [GNU General Public License v3.0](LICENSE) or later.
- Copyright (c) 2024 for music assets are held by [Per Christian Berg](https://perchristianberg.com).
- The graphical assets is licensed under a <a rel="license"
  href="http://creativecommons.org/licenses/by-nc-sa/4.0/">Creative Commons
  Attribution-NonCommercial-ShareAlike 4.0 International License</a>.
