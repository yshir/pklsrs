#!/bin/bash

set -euo pipefail

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

echo OK
