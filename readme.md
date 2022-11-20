# Simple Experiments with Rust and WASM
The goal of this repo is to see how small binaries can be with no outside dependencies.

## Learnings
The main thing I've learned from this is that adding types that can panic or allocate have a large overhead.
`wasm-strip` is very helpful.
