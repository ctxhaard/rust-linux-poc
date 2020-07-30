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

If you want to cross-compile your rust program follow this useful guide:
https://pagefault.blog/2018/07/04/embedded-development-with-yocto-and-rust/

Basically:
```sh
# check available targets list
rustc --print target-list
# add a target
rustup target add arm-unknown-linux-gnueabihf
# source your Yocto SDK environment file
# add some configurations to .cargo/config taken from CFLAGS variable

# ./cargo/conf
[target.arm-unknown-linux-gnueabihf]
linker="arm-linux-gnueabihf-gcc"
rustflags = [
"-C", "link-arg=-march=armv7-a",
"-C", "link-arg=-marm",
"-C", "link-arg=-mfpu=neon",
"-C", "link-arg=-mfloat-abi=hard",
"-C", "link-arg=--sysroot=/usr/local/arago-2018.04/sysroots/armv7ahf-neon-linux-gnueabi",
]
# build your project in a shell with the Yocto SDK environment set
cargo build --target arm-unknown-linux-gnueabihf
```
