use std::collections::HashMap;

/**
 * HashMap 键值对的形式存储数据
 * 一个键key对应一个值Value
 * Hash函数：决定如何在内存中存放K和V
 * 适用场景：通过K(任何类型)来寻找数据，而不是通过索引
 *
 * hashMap没有在标准库中，使用时需要先导入 use std::collections::HashMap;
 * hashMap是将数据存储在heap上
 * hashMap所有的K，V必须是同一种类型
 *
*/

fn main() {
    // 创建HashMao
    let mut map = HashMap::new();
    // 插入HashMap
    map.insert(String::from("10"), 10);

    // 使用collect方式创建
    // 在元素为Tuple的Vector上使用collect方式，可以组件一个HashMap
    // 要求Tuple有两个值，一个作为K，一个作为V
    // collect方法可以把数据整合成多种集合类型，包括HashMap
    // 返回值需要显式指明类型
    let colors = vec![String::from("yellow"), String::from("blue")];
    let scores = vec![10, 50];
    // 必须指明HashMap类型，K与V可以使用_代替，程序会自动推断KV类型，但
    // let res: HashMap<&String, &i32> = colors.iter().zip(scores.iter()).collect();
    let res: HashMap<&String, &i32> = colors.iter().zip(scores.iter()).collect();
    println!("{:#?}", res);

    // Hash Map的所有权规则
    // 对于实现了copy trait的类型，值会被复制到HashMao中
    // 对于拥有所有权的值，如String。值会被移动，所有权会转交给hashMap

    let key = String::from("key");
    let value = String::from("value");
    let mut map = HashMap::new();
    map.insert(key, value);
    // println!("{}{}", key, value); // 此时key，value的所有权被移交，打印时会报错

    // 如果只将引用插入hashMap 则不会移交所有权
    let key = String::from("key");
    let value = String::from("value");
    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{}{}", key, value); // 因为插入的引用 所以可以被打印

    // 在HashMap有效期间，被引用的值必须保持有效

    // 访问HashMap中的值
    match map.get(&key) {
        Some(v) => println!("{}", v),
        None => println!("没有值"),
    };
    // 遍历hashMap
    // 使用for循环
    for (key, value) in &map {
        println!("{}--{}", key, value);
    }

    // 更新hashMap
    // HashMap长度是可变的
    // 每一个key同时只能对应一个V
    // 更新hashMap中的数据
    // K已经存在，可以替换或保留现有值，或者可以合并现有的V和新的V
    // 如果K不存在，直接添加一对新的K，V

    // 替换
    let mut res = HashMap::new();
    res.insert(String::from("yellow"), 10);
    res.insert(String::from("yellow"), 15);
    println!("{:#?}", res); // 这里K是一样的所以被替换 { "yellow": 15, }

    // 在K值不对应的情况下才插入V
    // entry方法：检查指定的K是否对应一个V

    res.entry(String::from("yellow")).or_insert(16); // 该条不会被插入因为yellow已经存在
    res.entry(String::from("blue")).or_insert(18);
    println!("{:#?}", res); // 这里K是一样的所以被替换 { "yellow": 15, }

    // 案列 计算单词出现的次数
    let s = "i love u but u hate me i am sad";
    let mut map = HashMap::new();
    for i in s.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1
    }
    println!("{:#?}", map);

    // hash函数
    // 默认情况下，HashMap使用加密功能强大的hash函数，可以抵抗拒绝服务Dos攻击。
    // 它不是可用最快的hash算法，但具有更好的安全性
    // 可以指定不同的hasher来切换到另一个函数
    // hasher是实现BuildHasher trait的类型
}
