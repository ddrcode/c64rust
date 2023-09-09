#!/bin/bash

ASM="./bin/acme"
EMU="../target/debug/c64emu"
ADDR="0000"

GREEN='\033[0;32m'
RED='\033[0;31m'
LIGHT_BLUE='\033[1;34m'
RC='\033[0m' # Reset Color

OK_MSG="   ${GREEN}OK${RC}"
FAILED_MSG="   ${RED}Failed!${RC}"

mkdir -p ./asm-out/

run_test() {
    local file="$1"
    $ASM --cpu 6502 --setpc $ADDR -o "./asm-out/${file}.o" "./asm/${file}.a"
    local res=$($EMU --ram "./asm-out/${file}.o" --ram-file-addr $ADDR --start-addr $ADDR --show-status | sed 's/\$//g')
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
test "test"               "00" "00110010"
test "mul1"               "1e" "00110000"
test "mul2"               "40" "00110000"
test "cmp"                "42" "00110000"

