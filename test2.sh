#!/bin/bash

do_try() {
  expected=$1
  source=$2
  option=$3

  echo "$source" | tee tmp.vd
  cargo run -- $option tmp.vd tmp.s 2> /dev/null
  gcc tmp.s -o tmp
  ./tmp
  actual=$?
  if [ "$actual" != "$expected" ]; then
    echo "==> $expected expected, but got $actual"
    exit 1
  else
    echo "==> $expected"
  fi
  echo
}

try() {
  do_try "$1" "$2" ""
}

try 0 "func main(): int { return 0 }"
try 42 "func main(): int { return 42 }"

try 3 "func main(): int { return 1 + 2 }"
try 6 "func main(): int { return 1 + 2 + 3 }"
try 5 "func main(): int { return 6 - 1 }"
try 17 "func main(): int { return 20 - 5 + 2 }"

try 20 "func main(): int { return 2 * 2 * 5 }"
try 5 "func main(): int { return 20 / 4 }"
try 12 "func main(): int { return 1 + 2 * 3 + 5 / 1 }"

try 38 "func main(): int { return 3 + 5 * 7 }"
try 56 "func main(): int { return (3 + 5) * 7 }"
try 1 "func main(): int { return ((1)) }"

try 2 "func main(): int { return --2 }"
try 3 "func main(): int { return -3 * -1 }"

try 0 "func main(): int { return 1 & 0 }"
try 1 "func main(): int { return 1 & 1 }"
try 0 "func main(): int { return 0 | 0 }"
try 1 "func main(): int { return 1 | 0 }"
try 0 "func main(): int { return 1 ^ 1 }"
try 1 "func main(): int { return 1 ^ 0 }"