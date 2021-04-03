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

try 0 "func main(): int { if 0 == 1 { return 1 } else { return 0 } }"
try 1 "func main(): int { if 1 == 1 { return 1 } else { return 0 } }"
try 1 "func main(): int { if 0 != 1 { return 1 } else { return 0 } }"
try 0 "func main(): int { if 1 != 1 { return 1 } else { return 0 } }"

try 0 "func main(): int { if 1 < 1 { return 1 } else { return 0 } }"
try 1 "func main(): int { if 0 < 1 { return 1 } else { return 0 } }"
try 0 "func main(): int { if 2 <= 1 { return 1 } else { return 0 } }"
try 1 "func main(): int { if 1 <= 1 { return 1 } else { return 0 } }"
try 0 "func main(): int { if 1 > 1 { return 1 } else { return 0 } }"
try 1 "func main(): int { if 2 > 1 { return 1 } else { return 0 } }"
try 0 "func main(): int { if 0 >= 1 { return 1 } else { return 0 } }"
try 1 "func main(): int { if 1 >= 1 { return 1 } else { return 0 } }"

try 1 "func main(): int {
  if true {
    return 1
  } else {
    return 0
  }
}"

try 0 "func main(): int {
  if false {
    return 1
  } else {
    return 0
  }
}"

try 2 "func main(): int {
  if true {
    if true {
      return 2
    }
    return 1
  } else {
    return 0
  }
}"

try 1 "func main(): int {
  if true {
    if false {
      return 2
    }
    return 1
  } else {
    return 0
  }
}"

try 2 "func main(): int {
  if false {
    return 0
  } else if false {
    return 1
  } else {
    return 2
  }
}"

try 10 "func main(): int {
  var a: int = 10
  return a
}"

try 15 "func main(): int {
  var a: int = 10
  var b: int = 5
  return a + b
}"

try 0 "func main(): int {
  var a: int = 10
  var b: int = a
  a = -10
  return a + b
}"

try 20 "func main(): int {
  val a: int = 10
  val b: int = a
  return a + b
}"