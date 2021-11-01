```rust
// heap上集合有多很多,比如Vector,String,HashMap

/**
 * Vector
 * Vec<T>
 * 由标准库提供
 * 在Vector中可以存储多个值
 * 而且存储的数据类型必须相同
 * 这些值在内存中是连续存放的
 */

fn main() {
    // 使用Vec::new()创建Vec
    let _v: Vec<i32> = Vec::new();
    // 通常我们更习惯使用初始值的方式创建Vec<T>,使用vec!宏
    let _v = vec![1, 2, 3];
    // 向vec中添加元素.push
    let mut _v = Vec::new();
    _v.push(1);
    _v.push(2);
    // 删除vec
    // 当vector离开作用域后,就会被清理,它的所有元素也将被清理

    // 读取vec中的元素
    // 使用索引 &变量名[下标]
    // 索引在下标值越界时会引起恐慌
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", &v[2]); //3
    // 使用get方法
    // get方法会返回一个Options,它可以对index越界情况进行处理
    match v.get(2) {
        Some(v) => println!("{}", v),
        None => println!("没有获取到"),
    };
    // 遍历vec
    // for 循环遍历
    let v = vec![3,4,5];
    for i in &v {
        println!("{}", i);
    }
    // 通过for改变vec
    let mut v = vec![6,7,8];
    for i in &mut v {
        *i += 50; // 这里i是解引用
    }
    for i in &v {
        println!("{}", i);
    }
    /*
     * 在vec中可以使用枚举来存储多种数据类型
     * enum的变体可以附加不同数据类型
     * enum的变体定义在同一个enum类型下
     */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.2),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    println!("{:?}", &row[1])
}
```