```rust
fn main () {
  // if-else
  let num = 13;
  if num == 11 {
    println!("等于11")
  } else if  num > 11{
    println!("大于11")
  } else {
    println!("小于11")
  }
  // if表达式可以有返回值
  // 每个分支的返回类型必须是一样的
  let s = if 5 > 6 { 1 } else { 2 };
  println!("{}", s)
}
```