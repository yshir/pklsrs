# pklsrs

**pklsrs** is a toy compiler for arithmetic expressions written in Rust. It reads an arithmetic expression from standard input, tokenizes it, parses it into an abstract syntax tree (AST), and generates arm64 assembly code.

## Usage

Confirm your system and architecture:

```sh
$ uname; uname -m
Darwin
arm64
```

Compile and execute an arithmetic expression:

```sh
$ echo "1 + 2 * (3 + 4)" | cargo run --quiet > ./out/a.s; \
  clang -c ./out/a.s -o ./out/a.o; \
  clang ./out/a.o -o ./out/a; \
  ./out/a; echo $?
15
```

## Test

Run the test script:

```
$ ./test.sh
42 => 42
41 => 41
1 + 1 => 2
1 + 10 => 11
1 + 10 + 100 => 111
1 - 1 => 0
10 - 1 => 9
100 - 10 - 1 => 89
100 - 10 + 1 => 91
2 * 3 => 6
6 / 3 => 2
(1 + 2 * (3 + 4)) / 3 => 5
+ 1 => 1
- 1 + 3 => 2
1 == 1 => 1
1 == 2 => 0
1 != 1 => 0
1 != 2 => 1
1 == 1 == 1 => 1
1 == 1 != 1 => 0
1 < 2 => 1
1 < 1 => 0
2 < 1 => 0
1 <= 2 => 1
1 <= 1 => 1
2 <= 1 => 0
2 > 1 => 1
1 > 1 => 0
1 > 2 => 0
2 >= 1 => 1
1 >= 1 => 1
1 >= 2 => 0
OK
```
