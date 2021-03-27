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

This is an early experiment. Currently almost none of the [LÖVR API](https://lovr.org/docs/) is supported. The only present modules are `lovr` and a partial implementation of `lovr.filesystem`.

### lovr.filesystem

Only the following functions are implemented: `getIdentity`, `getSource`, `isFile`, `isFused`, `read`.

**API divergences:**

* Some paths that are illegal in normal LÖVR are allowed in Rovr.
* `lovr.filesystem.read` will currently only work on files containing valid UTF-8 text (this is to be considered a bug).

# License

[LICENSE](LICENSE)
