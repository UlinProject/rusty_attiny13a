#! /bin/bash

SOURCE="$1"

# 0x3A + 0xFB
# SPI_EN ON
# SAVE_EEPROM ON
# CPU: 9.6MHZ, 14+64
# BOD: 2.7V

echo "fuses:"
echo "l:0x3A + h:0xFB;"

#mkdir -p ./target/current &> /dev/null

# READ EEPROM
#echo "ReadEeprom"
#IN_CEEPROM_PATH='./target/current/ceeprom_data.bin'
#avrdude -q -C/etc/avrdude.conf -p attiny13a -b 19200 -P /dev/ttyUSB0 -c stk500v1 -U eeprom:r:${IN_CEEPROM_PATH}:r

# READ OSCCAL
#echo "ReadOsccal"
#$COSCCAL_PATH='./target/current/cosccal.bin'
#dd count=1 if=${IN_CEEPROM_PATH} of=${COSCCAL_PATH} bs=1 &> /dev/null

# MAKE EEPROM
#echo "MakeEeprom"
#NEW_EEPROM_PATH='./target/current/new_eeprom.bin'
#dd count=1 if=${COSCCAL_PATH} bs=1 of=${NEW_EEPROM_PATH} &> /dev/null
#dd if=./target/insert_eeprom of=${NEW_EEPROM_PATH} bs=1 skip=1 count=63 oflag=append conv=notrunc &> /dev/null


#echo "WriteProgramAndFuses"
# WRITE PROGRAM
echo "stk500v1 + 19200 + /dev/ttyUSB0:"
avrdude -q -C/etc/avrdude.conf -p attiny13a -b 19200 -P /dev/ttyUSB0 -c stk500v1 -U lfuse:w:0x3A:m -U hfuse:w:0xFB:m -U flash:w:${SOURCE}:e

# WRITE EEPROM
#echo "WriteEeprom"
#exec avrdude -q -C/etc/avrdude.conf -p attiny13a -b 19200 -P /dev/ttyUSB0 -c stk500v1 -U eeprom:w:${NEW_EEPROM_PATH}:r
