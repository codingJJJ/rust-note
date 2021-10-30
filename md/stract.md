```rust
#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}
struct RectSup {
    width: i32,
    height: i32,
}
impl RectSup {
    fn get_area (&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    // 使用struct关键字并为整个stract命名
    // 在花括号内，为所有字段feild定义名称和类型
    // 例如
    struct User {
        name: String,
        active: bool,
        count: u64,
    }

    let user = User {
        name: String::from("_"),
        active: true,
        count: 64,
    };
    // 可以通过点标记法直接获取struct直接获取
    println!("{},{},{}", user.active, user.name, user.count);

    //一旦struct的实例是可变的，那么实例中的所有字段都是可以变的
    // struct可以作为函数的返回值
    let _return = get_struct(String::from("_"), false, 41);
    fn get_struct(name: String, active: bool, count: u64) -> User {
        User {
            name,
            active,
            count,
        }
    }
    // 字段初始化的简写
    // 当字段名与字段值相同时，可以使用字段初始化简写方式
    let name = String::from("_");
    let _user1 = User {
        name, // 这里可以简写name
        active: false,
        count: 1,
    };
    // struct可以使用更新语法 .. 注意 这是两个点
    let test1 = User {
        active: true,
        name: String::from("_"),
        count: 10,
    };

    let _test2 = User {
        active: false,
        ..test1
    };

    // Tuple Struct
    // 可以定义一个类似于tuple的struct
    // Tuple struct整体有个名，但是里面的元素没有名
    // 使用：想给整个tuple起名，并让他不同于其他tuple，而且也不需要给每个元素起名
    // 例如
    struct Color(i32, i32, i32);
    let _color = Color(255, 255, 255);
    struct Point(i32, i32, i32);
    let Point(x, y, z) = Point(1, 2, 3);
    println!("{}{}{}", x, y, z);

    // unit like Struct(没有任何字段)
    // 可以定义没有任何字段的struct,叫做unit-like struct(因为与（）单元类型类似)
    // 适用于需要在某个类型上实现某个trait,但是在里面又没有想要存储的数据

    // Struct的字段使用了String而不是&str
    // 该struct拥有其所有数据
    // 只要struct实列是有效的，那么里面的所有字段数据也是有效的
    // struct里也可以放引用，但这需要使用声明周期

    // struct案例
    // 求长方形的面积



    let rect = Rect { width: 30, height: 50 };
    let area = get_area(&rect);
    fn get_area(Rect{width, height}: &Rect) -> i32 {
        width * height
    }
    println!("area---{}", area);
    // 关于struct的打印
    // 方法一： 使用debug
        // 在首行添加#[derive(Debug)]， struct声明应在外层，然后使用println!("{:?}", x);
        // derive是派生的意思
    println!("{:?}", rect);
    // 方法二： 使用:#?
        // :#?的方法会使显示结果更为直观
    println!("{:#?}", rect);

    // struct方法
    // struct的方法和函数类似 fn关键字 名称 参数 返回值
    // 方法与函数的区别
        // 方法是在struct（或者enum, trait对象）的上下文定义
        // 第一个参数是self，即表示方法被调用的struct实例 
    // impl块内定义方法
    // 方法第一个参数是&self， 也可以获得其所有权或可变借用 和其他参数一样
    // 列子：现在我们把求长方形的面积优化

    // struct 见第6-14行
    let rect = RectSup {
        height: 30,
        width: 50
    };

    println!("RectSupArea{}", rect.get_area());

    // 关联函数
    // 可以在impl块内定义不把self作为第一个参数的函数，它们叫做关联函数，不是方法
    // 关联函数的访问使用::（::也可以创建模块命名空间）
    // 可以把关联函数理解为静态方法
    // 每个struct允许拥有多个impl块
}
```