cd get_str
cargo component bindings
cargo component build
cd ..
cd malicious_wasi_cli_environment
cargo component bindings
cargo component build
cd ..
echo "without plug"
cargo run
echo "with plug"
wac plug get_str/target/wasm32-wasip1/debug/get_str.wasm --plug malicious_wasi_cli_environment/target/wasm32-wasip1/debug/malicious_wasi_cli_environment.wasm -o a.wasm
cargo run