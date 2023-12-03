# Running RandyX kernel

## To be able to run RandyX kernel you need to:

- have installed rustc nightly
```
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

- install bootimage to be able to run the kernel
```
cargo install bootimage
```

- install llvm-tools-preview
```
rustup component add llvm-tools-preview
```

## Running

- install quemu
```
sudo apt install qemu-system-x86_64
```

- run the kernel
```
cargo run
```