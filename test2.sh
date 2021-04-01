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