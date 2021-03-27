# About Rovr

## What is Rovr?

Rovr is a port of LÖVR to Rust, with the VR parts removed.

## What is LÖVR?

[LÖVR](https://lovr.org/) is a reimplentation of LOVE2D with additional functionality for VR.

## What is LOVE2D?

[Love2D](https://love2d.org/) is a Lua game engine.

## So you ported LOVE2D to VR, then ported it again to remove the VR?

Well… I didn't write LÖVR myself, but basically, yes.

## Why not just port LOVE2D to Rust?

That is a legitimate question.

# Usage

```
cargo run -- path/to/your/game
```

## API coverage

This is an early experiment. Currently almost none of the [LÖVR API](https://lovr.org/docs/) is supported. The only present modules are `lovr` and `lovr.filesystem`, but many functions on `lovr.filesystem` are missing.

# License

[LICENSE](LICENSE)
