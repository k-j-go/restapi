```shell
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
```

#### when get below error
```text
cargo build --target x86_64-unknown-linux-musl
error: linker `cc` not found
error: could not compile `cross-compile-sample`
```

Adding rust-lld as the linker for our musl target in our .cargo/config.toml file will switch from cc to Rust's lld:

```text
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
```
##### Build for linux
```shell
cargo build --target x86_64-unknown-linux-musl  --release

```