# wlprobe
`wayland-info` but intended for computers rather than humans. Emits JSON which [`wayland-info` cannot yet do](https://gitlab.freedesktop.org/wayland/wayland-utils/-/issues/2).

You can get the binary from [release page](https://github.com/PolyMeilex/wlprobe/releases) or build it yourself using `cargo`

Just build and run:
```sh
cargo run
```
Install the binary on the system:
```sh
cargo install --path .

wlprobe > output.json
```
