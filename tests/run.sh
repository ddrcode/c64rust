#!/bin/bash

ASM="./bin/acme"
EMU="../target/debug/c64emu"
ADDR="0200"

GREEN='\033[0;32m'
RED='\033[0;31m'
LIGHT_BLUE='\033[1;34m'
RC='\033[0m' # Reset Color

OK_MSG="   ${GREEN}OK${RC}"
FAILED_MSG="   ${RED}Failed!${RC}"

mkdir -p ./asm-out/

# build rom file for tests
$ASM --cpu 6502 --format plain -o asm-out/rom.p asm/rom.asm

run_test() {
    local file="$1"
    local bin_file="./asm-out/${file}.p"
    local addr_dec=$(echo "ibase=16; $ADDR"|bc)
    $ASM --cpu 6502 -f plain --setpc "$addr_dec" -o "$bin_file" "./asm/${file}.asm"
    local res=$($EMU --rom ./asm-out/rom.p --ram "$bin_file" --ram-file-addr "$ADDR" --show-status | sed 's/\$//g')
    printf -- "%s" "$res"
}

assert() {
    local expected="$1"
    local actual="$2"
    if [[ "$expected" == "$actual" ]]; then return 1; fi
    return 0
}

test() {
    local file="$1"
    local expected_a="$2"
    local expected_p="$3"

    echo ""
    echo -e "Running test for ${LIGHT_BLUE}${file}.a${RC}"
    local res=$(run_test "$file")
    local reg_a=$(echo "$res" | sed 's/.*A:\(..\).*/\1/')
    local reg_p=$(echo "$res" | sed 's/.*P:\(.*\)/\1/')
    local cmp1=$(assert "$expected_a" "$reg_a")

    echo "   State: $res"
    echo "               NV-BDIZC"
    echo "   Expected P: ${expected_p}, Expected A: ${expected_a}"
    echo "   Actual P:   ${reg_p}, Actual A:   ${reg_a}"

    assert "$expected_a" "$reg_a"  && echo -e "$FAILED_MSG" && exit 1
    assert "$expected_p" "$reg_p"  && echo -e "$FAILED_MSG" && exit 1
    echo -e "$OK_MSG"
    return 0
}

#                               NV-BDIZC
test "mul1"               "1e" "00110000"
test "mul2"               "40" "00110000"
test "cmp"                "42" "00110010"

