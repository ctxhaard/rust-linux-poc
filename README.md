# rust-linux-poc
A POC to check if a basic crate with some basic dependencies is ported easily on Yocto and arm

## Rust installation
If you are running Linux:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
PATH=$HOME/.cargo/bin:$PATH
cd rust-linux-poc
cargo run res/config.json
```
