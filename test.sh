#!/bin/bash

try() {
  option=$1

  # python3 test_preprocess.py
  cargo run -- $option test.vd tmp.s
  if [ "$?" != "0" ]; then
    echo "compiling failed"
    exit 1
  fi
  cargo run -- --dump-ir test.vd tmp.s > tmp.ssa
  gcc tmp.s -o tmp
  ./tmp
  actual=$?
  if [ "$actual" != "0" ]; then
    echo "test: FAILED"
  else
    echo "test: PASSED"
  fi
}

try ""
# try "--optimize"
