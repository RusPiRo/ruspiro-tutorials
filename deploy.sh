#!/bin/bash
#set -ev
# Deploy image to Raspberry Pi.
# Prerequisite:
# - Raspberry UART1 (miniUart - GPIO 14/15) connected through TTL-USB dongle to PC
# - Raspberry runs a small custom bootloader

# get the folder of script execution
CURRENT="$( cd "$( dirname "$(readlink -f "$0")" )" >/dev/null 2>&1 && pwd )"

# first prepare the file to be transmitted to Raspberry Pi
# the binarry is surrounded by "DEADBEEF" string which are the tokens recognized by the boot-loader
cat deadbeef.txt "./$1/target/kernel7.img" deadbeef.txt > transfer.img

# now use the terminal windows app "teraterm" to transfer the file
( exec "C:/Program Files (x86)/teraterm/ttpmacro.exe" "$CURRENT/tterm.ttl" "$CURRENT" )
