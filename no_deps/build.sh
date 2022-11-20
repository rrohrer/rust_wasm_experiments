cargo build --release --target=wasm32-unknown-unknown
rm -rf ./dist
mkdir dist
wasm-opt -Oz -o dist/gl.wasm target/wasm32-unknown-unknown/release/rust_wasm_experiment.wasm
wasm-strip dist/gl.wasm
cp ./static/index.html ./dist/index.html
