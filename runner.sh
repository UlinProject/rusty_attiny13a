#! /bin/bash

SOURCE="$1"

# 0x3A + 0xFB
# SPI_EN ON
# SAVE_EEPROM ON
# CPU: 9.6MHZ, 14+64
# BOD: 2.7V
#
L_FUSE="0x3A"
H_FUSE="0xFB"

echo "fuses:"
echo "l:${L_FUSE} + h:${H_FUSE};"
echo ""

echo "stk500v1 + 19200 + /dev/ttyUSB0:"
avrdude -q -C/etc/avrdude.conf -p attiny13a -b 19200 -P /dev/ttyUSB0 -c stk500v1 -U lfuse:w:${L_FUSE}:m -U hfuse:w:${H_FUSE}:m -U flash:w:${SOURCE}:e
