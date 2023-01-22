#! /bin/sh
 
# first delete everything, since there might be remains of a previously issued cargo check
rm -rf target/armv7-unknown*
CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
cross build --bin tof-control --target=armv7-unknown-linux-musleabi
scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb51:~/takeru_dev
scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb52:~/takeru_dev

