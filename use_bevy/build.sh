cargo build --release --target=wasm32-unknown-unknown
rm -rf ./generated
wasm-bindgen --out-dir generated --web target/wasm32-unknown-unknown/release/use_bevy.wasm
wasm-opt -Oz -o generated/use_bevy_bg.wasm generated/use_bevy_bg.wasm
wasm-strip generated/use_bevy_bg.wasm
cp ./static/index.html ./generated/index.html
