```rust
/**
 * String类型
 * 开发者经常会被字符串困扰的原因
 * Rust倾向暴露可能的错误
 * 字符串数据结构复杂
 * rust字符串使用了UTF-8编码
 *
 * 字符串是基于byte的一个集合
 * 提供了一些方法可以将byte解析为文本
 *
 * 字符串是什么?
 * 在Rust核心语言层面,只有一个字符串类型:字符串切片str(&str)
 * 字符串切片是对存储在其他地方或UTF-8编码的字符串引用
 * 字符串字面值:存储在二进制文件中,也是字符串切片
 */

/*
 * String类型
 * 它来自于标准库,而不是核心语言
 * 可增长,修改,可拥有所有权
 * 同样采用UTF-8编码形式\
 * 我们通常手的字符串是指String和&str
 *
 * 在标准库中也提供了其他字符串类型,例如OsString,OsString,CString,CStr
 */

fn main() {
    /*
     * 创建一个新的字符串
     * 1.String::new()
     * let s = String::new();
     * 
     * 2.使用初始值来创建String
     * to_string()方法,可用于实现了display trait的类型,包括字符串字面值
     * let s = "test".to_string();
     * 3.使用String::from()函数从字面值创建String
     * let s = String::from("test");
     */
    
    // 更新String
    // push_str()方法: 把一个字符串切片附加到String
    // push方法不会获取参数的所有权
    let mut s = "123".to_string();
    s.push_str("456");
    println!("{}", s);
    // push()方法:将单个字符附加到String
    s.push('7'); //这里使用的单引号 表示ch类型
    println!("{}", s);
    // 拼接字符串
    // + 链接字符串
    // 注意使用+连接符这个方法类似于 fn add(self, s: &str) -> String {...}
    let s1 = String::from("123");
    let s2 = String::from("456");
    let s3 = s1 + &s2; // 拼接之后s1所有权消失, s2所有权还存在
    println!("{}", s3);
    // format!:链接多个字符串
    // format类似于println,不过format会将该文本内容返回
    // 在不使用format的情况下
    let s1 = String::from("i");
    let s2 = String::from("love");
    let s3 = String::from("you");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s);
    // 使用format
    // 使用format不会获取参数的所有权 !!!
    let s1 = String::from("i");
    let s2 = String::from("love");
    let s3 = String::from("you");
    let s = format!("{} {} {}", s1, s2, s3);
    println!("{}", s);

    // 对String按索引的形式进行访问会直接报错
    // Rust字符串不支持索引的语法访问
    // String是对Vec<u8>的包装

    // len()方法可以获取字符串长度
    // 对于某些特定语言会使用unicode标量值,它得每一个len()为2
    let len = String::from("hello").len();
    println!("{}", len); // 5

    // Rust有三种看待字符串的方式
    //字节(bytes),标量值(Scalar Values),字型戳(Grapheme Clusters)

    // 字节
    let s = String::from("我爱你");
    for b in s.bytes() {
        println!("{}", b); // 打印9次
    }
    // 标量值
    for c in s.chars() {
        println!("{}", c); // 打印三次
    }
    // 字型戳
    // 标准库没有提供

    // Rust不允许对String进行索引的最后一个原因
        // 索引操作消耗一个常量时间(O(1))
        // 而String无法保证,需要遍历所有的内容,来确定有多少个合法字符

    // 切割String
    // 可以使用[]和一个范围来创建字符串切片
    // 虽然允许切割字符串,但是需要谨慎使用
    // 如果切割时,跨越了字符边界,程序就会panic

    let s = "一个范围来创建字符串切片";
    let s = &s[3..6]; // 如果写&[3..5]会报恐慌,因为它不是一个边界,中文字符没3个字节算一个标量值边界
    println!("{}", s);
    
    // 遍历String
    // 对于字节可以使用bytes()
    // 对于标量值使用charts()
    // 对于字形戳,标准库未提供

    /*
     * String不简单
     * 
     * Rust选择将正确处理String数据作为所有Rust程序的默认行为
     *      程序员必须在处理UTF-8编码数据之前投入更多精力
     *      不过优势防止在于在开发后期处理涉及非ASCII字符的错误
     * 
     */
}
```