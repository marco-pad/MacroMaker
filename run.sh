./build.sh
scp -O target/riscv64gc-unknown-linux-musl/release/firmware root@milkv:programs/firmware
ssh root@milkv programs/firmware
