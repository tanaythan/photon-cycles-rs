# photon-cycles-rs

Couch co-op game inspired by the custom map in Starcraft 2. This is still WIP.

## How to run

To run the game, run the following command, which defaults to the `metal` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
