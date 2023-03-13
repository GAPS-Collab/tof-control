#! /bin/bash
 
# first delete everything, since there might be remains of a previously issued cargo check
rm -rf target/armv7-unknown*
# CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
if [ "$1" = "i2c-check" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin i2c-check --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/i2c-check tof-rb01:~/takeru_dev
elif [ "$1" = "rb-watchdog" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rb-watchdog --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb01:~/bin
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb02:~/bin
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb03:~/bin
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb04:~/bin
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb05:~/bin
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb06:~/bin
elif [ "$1" = "dac-test" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin dac-test --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/dac-test tof-rb13:~/takeru_dev
else
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --bin tof-control --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb01:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb02:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb03:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb04:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb05:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb06:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb03:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb52:~/takeru_dev

    scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb00:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb01:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb02:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb03:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb04:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb05:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb06:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb07:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb08:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb09:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb10:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb11:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb12:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb13:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb14:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb15:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb16:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb49:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb52-ssl:~/bin
fi
# scp target/armv7-unknown-linux-musleabi/debug/tof-control gaps@10.97.108.39:~/takeru_dev
# scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb52:~/takeru_dev

