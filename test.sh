#!/bin/bash

set -feuo pipefail

DIR=`dirname $0`

rm -rf $DIR/out; mkdir -p $DIR/out

cargo build --quiet

assert() {
    echo -n $1 | $DIR/target/debug/pklsrs > $DIR/out/a.s
    clang -c $DIR/out/a.s -o $DIR/out/a.o
    clang $DIR/out/a.o -o $DIR/out/a

    input=$1
    expected=$2
    actual=$(echo $input | $DIR/out/a; echo $?)
    if [ "$actual" != "$expected" ]; then
        echo "$input => $expected expected, but got $actual"
        echo NG
        exit 1
    else
        echo "$input => $actual"
    fi
}

assert "42" "42"
assert "41" "41"
assert "1 + 1" "2"
assert "1 + 10" "11"
assert "1 + 10 + 100" "111"
assert "1 - 1" "0"
assert "10 - 1" "9"
assert "100 - 10 - 1" "89"
assert "100 - 10 + 1" "91"
assert "2 * 3" "6"
assert "6 / 3" "2"
assert "(1 + 2 * (3 + 4)) / 3" "5"
assert "+ 1" "1"
assert "- 1 + 3" "2"
assert "1 == 1" "1"
assert "1 == 2" "0"
assert "1 != 1" "0"
assert "1 != 2" "1"
assert "1 == 1 == 1" "1"
assert "1 == 1 != 1" "0"
assert "1 < 2" "1"
assert "1 < 1" "0"
assert "2 < 1" "0"
assert "1 <= 2" "1"
assert "1 <= 1" "1"
assert "2 <= 1" "0"
assert "2 > 1" "1"
assert "1 > 1" "0"
assert "1 > 2" "0"
assert "2 >= 1" "1"
assert "1 >= 1" "1"
assert "1 >= 2" "0"
assert "1 + 2 == 3" "1"
assert "1 + 2 != 3" "0"
assert "1 + 2 < 3" "0"
assert "1 + 2 <= 3" "1"
assert "1 + 2 > 3" "0"
assert "1 + 2 >= 3" "1"
assert "1 + 2 == 2" "0"
assert "1 + 2 != 2" "1"
assert "1 + 2 < 2" "0"
assert "1 + 2 <= 2" "0"
assert "1 + 2 > 2" "1"
assert "1 + 2 >= 2" "1"
assert "a = 1" "1"
assert "a = 2" "2"
assert "a = 1 + 2" "3"
assert "(a = 1 + 2) * 3" "9"

echo OK
