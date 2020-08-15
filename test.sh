#!/bin/bash

try() {
  expected=$1
  source=$2

  cargo run "$source" 2> /dev/null > tmp.s
  gcc -m32 tmp.s -o tmp
  ./tmp
  actual=$?
  if [ "$actual" != "$expected" ]; then
    echo "$source => $expected expected, but got $actual"
    exit 1
  else
    echo "$source => $expected"
  fi
}

try 0 "0"
try 42 "42"

try 3 "1 + 2"
try 6 "1 + 2 + 3"
try 5 "6 - 1"
try 17 "20 - 5 + 2"
