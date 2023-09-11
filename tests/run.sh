#!/bin/bash

ASM="./bin/acme"
EMU="../target/debug/c64emu"
ADDR="0200"

GREEN='\033[0;32m'
RED='\033[0;31m'
LIGHT_BLUE='\033[1;34m'
LIGHT_GRAY='\033[0;37m'
RC='\033[0m' # Reset Color

OK_MSG="[   ${GREEN}OK${RC}   ]"
FAILED_MSG="[ ${RED}Failed${RC} ]"

mkdir -p ./target/

# build rom file for tests
$ASM --cpu 6502 --format plain -o target/rom.p src/roms/tests-rom.asm

run_test() {
    local file="$1"
    local bin_file="./target/${file}.p"
    local addr_dec=$(echo "ibase=16; $ADDR"|bc)
    $ASM --cpu 6502 -f plain --setpc "$addr_dec" -o "$bin_file" "./src/${file}.asm"
    local res=$($EMU --rom ./target/rom.p --ram "$bin_file" --ram-file-addr "$ADDR" --show-status | sed 's/\$//g')
    printf -- "%s" "$res"
}

test() {
    local file="$1"
    echo -ne "Running test for ${LIGHT_BLUE}${file}.a${RC}\t\t\t\t"

    local res=$(run_test "$file")
    local reg_y=$(echo "$res" | sed 's/.*Y:\(..\).*/\1/')

    if [ "00" == $reg_y ]; then
        echo -e "$OK_MSG"
        return 0
    fi

    echo -e "$FAILED_MSG"
    echo ""
    echo -e "                                             ${LIGHT_GRAY}NV-BDIZC${RC}"
    echo "   State: $res"
    echo ""
    echo "   Reg A - test id of the last successfully completed test"
    echo "   Reg Y - error code"
    echo "   Reg P - last status before the error"
    echo ""

    exit 1
}

test "mul1"
test "mul2"
test "cmp" 
test "adc" 
test "sbc" 

