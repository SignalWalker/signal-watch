#! /usr/bin/fish

arduino-cli upload -p /dev/ttyACM0 -b arduino:mbed:nano33ble -i target/thumbv7em-none-eabihf/release/signal-watch
