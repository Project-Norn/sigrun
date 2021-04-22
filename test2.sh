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

try 55 "func main(): int {
  var sum: int = 0
  var i: int = 0
  while i <= 10 {
    sum = sum + i
    i = i + 1
  }
  return sum
}"

try 110 "func main(): int {
  var sum: int = 0
  var i: int = 0
  var j: int = 0
  while i < 2 {
    j = 0
    while j <= 10 {
      sum = sum + j
      j = j + 1
    }
    i = i + 1
  }
  return sum
}"

try 1 "func // hoge
main(): // bool
int {
  /*/ return 0
  /*/
  return 1
}"

try 2 "func hoge(): int { return 1 }
func fuga(): int { return hoge() }
func main(): int { return hoge() + fuga() }"

try 40 "func hoge(): int { return 1 * 2 + 3 * 4 }
func fuga(): int { return 3 * 4 + 1 * hoge() }
func main(): int { return hoge() + fuga() }"

try 1 "func hoge() { }
func main(): int { hoge()hoge()hoge()hoge()hoge() return 1 }"

try 0 "func hoge() {
  return
  if false {
    return
  }
}

func main(): int {
  hoge()
  return 0
}"

try 3 "func add(a: int, b: int): int { return a + b }
func main(): int { return add(1, 2) }"

try 5 "func fib(n: int): int {
  if n <= 1 {
    return n
  } else {
    return fib(n - 1) + fib(n - 2)
  }
}
func main(): int { return fib(5) }"

try 0 "func main(): int {
  val a: int
  return a
}"

try 10 "func main(): int {
  val a: int
  val b: int = a + 10
  return b
}"