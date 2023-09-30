#!/bin/bash

cargo build --target ps2.json

mv ../target/ps2/debug/hello-rs ../target/ps2/debug/hello-rs.elf

pcsx2-qt -elf ../target/ps2/debug/hello-rs.elf

