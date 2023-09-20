cargo build --release --target wasm32-unknown-unknown --package ic_jwt
ic-wasm target/wasm32-unknown-unknown/release/ic_jwt.wasm -o target/wasm32-unknown-unknown/release/ic_jwt.wasm shrink
gzip -f target/wasm32-unknown-unknown/release/ic_jwt.wasm

cargo build --release --target wasm32-unknown-unknown --package factory
ic-wasm target/wasm32-unknown-unknown/release/factory.wasm -o target/wasm32-unknown-unknown/release/factory.wasm shrink
gzip -f target/wasm32-unknown-unknown/release/factory.wasm