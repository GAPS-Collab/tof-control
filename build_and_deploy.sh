#! /bin/bash
 
# first delete everything, since there might be remains of a previously issued cargo check
# rm -rf target/armv7-unknown*
# CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
if [ "$1" = "i2c-check" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin i2c-check --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/i2c-check tof-rb00:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/i2c-check tof-rb45:~/
elif [ "$1" = "rb-watchdog" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rb-watchdog --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb01:~/bin
    # scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb02:~/bin
    # scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb03:~/bin
    # scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb04:~/bin
    # scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb05:~/bin
    # scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb06:~/bin
    scp target/armv7-unknown-linux-musleabi/release/rb-watchdog tof-rb22:~/bin
elif [ "$1" = "dac-test" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin dac-test --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/dac-test tof-rb10:~/takeru_dev
elif [ "$1" = "gpioe-watchdog" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin gpioe-watchdog --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/gpioe-watchdog tof-rb00:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/gpioe-watchdog tof-rb12:~/bin
elif [ "$1" = "board-id" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin board-id --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/gpioe-watchdog tof-rb00:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/board-id tof-rb10:~/bin
elif [ "$1" = "sma-input" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin sma-input --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/gpioe-watchdog tof-rb00:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/sma-input tof-rb10:~/takeru_dev
elif [ "$1" = "clk-synth" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin clk-synth --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/gpioe-watchdog tof-rb00:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/clk-synth tof-rb10:~/takeru_dev
elif [ "$1" = "cpc-control" ]; then
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin cpc-control --target=aarch64-unknown-linux-musl
    scp target/aarch64-unknown-linux-musl/release/cpc-control tof-pi3:~/
elif [ "$1" = "cpc-influxdb" ]; then
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin cpc-influxdb --target=aarch64-unknown-linux-musl
    scp target/aarch64-unknown-linux-musl/release/cpc-influxdb tof-pi3:~/
elif [ "$1" = "tcpc-control" ]; then
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin tcpc-control --target=aarch64-unknown-linux-musl
    scp target/aarch64-unknown-linux-musl/release/tcpc-control tof-pi3:~/
elif [ "$1" = "tcpc-influxdb" ]; then
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin tcpc-influxdb --target=aarch64-unknown-linux-musl
    scp target/aarch64-unknown-linux-musl/release/tcpc-influxdb tof-pi3:~/
elif [ "$1" = "tof-influxdb" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin tof-influxdb --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/gpioe-watchdog tof-rb00:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/release/tof-influxdb tof-rb16:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/release/tof-influxdb tof-rb25:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/tof-influxdb tof-rb43:~/takeru_dev
    scp target/armv7-unknown-linux-musleabi/release/tof-influxdb tof-rb44:~/takeru_dev
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

    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb00:~/bin
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
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb17:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb18:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb19:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb20:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb21:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb22:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb23:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb49:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb52-ssl:~/bin

    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb05:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb10:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb21:~/takeru_dev
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb22:~/takeru_dev

    # ## SSL
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb01:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb02:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb03:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb04:~/bin
    # # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb05:~/bin
    # # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb06:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb07:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb08:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb09:~/bin
    # # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb10:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb11:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb12:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb13:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb14:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb15:~/bin
    # # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb16:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb17:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb18:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb19:~/bin
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb20:~/bin
    # # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb21:~/bin
    # # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb22:~/bin
fi
# scp target/armv7-unknown-linux-musleabi/debug/tof-control gaps@10.97.108.39:~/takeru_dev
# scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb52:~/takeru_dev

