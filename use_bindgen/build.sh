cargo build --release --target=wasm32-unknown-unknown
rm -rf ./generated
wasm-bindgen --out-dir generated --web target/wasm32-unknown-unknown/release/use_bindgen.wasm
wasm-opt -Oz -o generated/use_bindgen_bg.wasm generated/use_bindgen_bg.wasm
wasm-strip generated/use_bindgen_bg.wasm
cp ./static/index.html ./generated/index.html
