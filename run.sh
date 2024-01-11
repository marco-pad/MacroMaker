./build.sh
scp target/riscv64gc-unknown-linux-musl/release/milk-v-test root@milk-v:programs/milk-v-test
ssh root@milk-v programs/milk-v-test
