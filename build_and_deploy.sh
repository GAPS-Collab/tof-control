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
    # CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    # cross build --release --bin cpc-control --target=aarch64-unknown-linux-musl
    # scp target/aarch64-unknown-linux-musl/release/cpc-control tof-pi3:~/
    # scp target/aarch64-unknown-linux-musl/release/cpc-control tof-pi3:~/
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --bin cpc-control --target=x86_64-unknown-linux-musl
    scp target/x86_64-unknown-linux-musl/debug/cpc-control tof-computer:~/takeruhayashi
elif [ "$1" = "cpc-influxdb" ]; then
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin cpc-influxdb --target=aarch64-unknown-linux-musl
    scp target/aarch64-unknown-linux-musl/release/cpc-influxdb tof-pi3:~/
elif [ "$1" = "tcpc-control" ]; then
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin tcpc-control --target=aarch64-unknown-linux-musl
    scp target/aarch64-unknown-linux-musl/release/tcpc-control tof-pi3:~/
    
elif [ "$1" = "rb-clk-nvm" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rb-clk-nvm --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/rb-clk-nvm tof-rb00:~/dev

elif [ "$1" = "rb-control" ]; then
    # rm -rf target/armv7-unknown*
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rb-control --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/rb-control tof-rb18:~/dev
    scp target/armv7-unknown-linux-musleabi/release/rb-control tof-rb09:~/dev
    scp target/armv7-unknown-linux-musleabi/release/rb-control tof-rb37:~/dev
    scp target/armv7-unknown-linux-musleabi/release/rb-control tof-rb43:~/dev

elif [ "$1" = "ltb-control" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin ltb-control --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/release/ltb-control tof-rb18:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/ltb-control tof-rb09:~/dev
    scp target/armv7-unknown-linux-musleabi/release/ltb-control tof-rb37:~/dev
    scp target/armv7-unknown-linux-musleabi/release/ltb-control tof-rb43:~/dev

elif [ "$1" = "pb-control" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin pb-control --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/pb-control tof-rb18:~/dev
    scp target/armv7-unknown-linux-musleabi/release/pb-control tof-rb09:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/pb-control tof-rb37:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/pb-control tof-rb43:~/dev

elif [ "$1" = "preamp-control" ]; then
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin preamp-control --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/preamp-control tof-rb18:~/dev
    scp target/armv7-unknown-linux-musleabi/release/preamp-control tof-rb09:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/preamp-control tof-rb37:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/preamp-control tof-rb43:~/dev

elif [ "$1" = "rb-wd" ]; then
    rm -rf target/
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rb-wd --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/rb-wd tof-computer:/home/gaps/tof-rb/sbin

elif [ "$1" = "rb-clk-rst" ]; then
    rm -rf target/
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rb-clk-rst --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/rb-clk-rst tof-computer:/home/gaps/tof-rb/bin


elif [ "$1" = "rat-init" ]; then
    rm -rf target/
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin rat-init --target=armv7-unknown-linux-musleabi
    scp target/armv7-unknown-linux-musleabi/release/rat-init tof-computer:/home/gaps/tof-rb/bin
    # scp target/armv7-unknown-linux-musleabi/release/rat-init tof-rb41:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/rat-init tof-rb42:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/rat-init tof-rb25:~/dev
    # scp target/armv7-unknown-linux-musleabi/release/rat-init tof-rb16:~/bin

elif [ "$1" = "cpu-control" ]; then
    # rm -rf target/x86_64-unknown*
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin cpu-control --target=x86_64-unknown-linux-musl
    scp target/x86_64-unknown-linux-musl/release/cpu-control tof-computer:/home/gaps/dev
    # CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    # cross build --release --bin cpu-control --target=x86_64-unknown-linux-gnu
    # scp target/x86_64-unknown-linux-gnu/release/cpu-control tof-computer:/home/gaps/dev

elif [ "$1" = "switch-control" ]; then
    # rm -rf target/x86_64-unknown*
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --release --bin switch-control --target=x86_64-unknown-linux-musl
    scp target/x86_64-unknown-linux-musl/release/switch-control tof-computer:/home/gaps/dev

else
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABI_RUSTFLAGS="-C relocation-model=dynamic-no-pic -C target-feature=+crt-static" \
    cross build --bin tof-control --target=armv7-unknown-linux-musleabi
    # scp target/armv7-unknown-linux-musleabi/debug/tof-control tof-rb01:~/takeru_dev
fi

