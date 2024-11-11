default:
    @just --list

setup:
    @sudo apt install libgtk-4-dev libadwaita-1-dev build-essential

install:
    @cargo build --release
    @sudo cp ./target/release/battery_limiter /usr/bin/battery_limiter

uninstall:
    @sudo rm /usr/bin/battery_limiter
