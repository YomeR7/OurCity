wasm-pack build --target web --out-dir target/web/renderer/
rm -rfd ../client/src/lib/renderer
cp -r target/web/renderer ../client/src/lib/renderer
