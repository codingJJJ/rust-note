```rust
/**
 * use关键字
 * 可以使用use关键字将路径导入到作用域内
 * 它仍然遵循私有性规则
 *
 * use的习惯用法
 * 函数一般将函数的父级引入到作用域
 * 而类似于struct, enmu,其他：指定完整路径（即指定到本身）
 * 关于同名条目需要指定到父级
 *      fn f1 -> fmt::Result {}
 *      fn f2 -> io::Result {}
 */

mod first_deep {
    pub mod secend_deep {
        pub fn to_do() {
            println!("我执行了")
        }
        fn _not_to_do() {
            println!("这里外面拿不到")
        }
    }
}

use crate::first_deep::secend_deep::to_do;

use std::cmp::Ordering;
/**
 * as关键字
 * 可以为引入的路径指定本地别名
 * use std::io::Result as ioResult;
 */

/**
 * pub use
 * 使用use将路径导入到作用域后，该名称在此作用域内是私有的
 * 而pub use既可以将条目引入当前作用域
 * 也可以使该条目被外部代码引入到他们的作用域中
 */

/**
 * 使用外部包(package)
 * 1.在cargo.toml文件中添加依赖包
 *  cargo会从http://crates.io/中下载该包到本地
 * 2.use将特定的条目引入到作用域
 */

// 我们在cargo.toml文件dependencies下添加一个模块包
// [dependencies]
// rand = "0.5.5"
// 如果下载太慢我们可以用镜像下载
/**
 * 使用清华大学镜像下载
 * 先关闭vscode中rust stop the rust server
 * 使用ctrl + shift + p打开弹窗并关闭rust server
 * cmd中输入where cargo会显示当前cargo包路径
 * C:\Users\Administrator\.cargo\bin\cargo.exe
 * 进入刚目录并删掉\.cargo\下.package-cache
 * 进入.cargo文件夹建立一个config文件
 * 内容见cargo镜像.md
 */

// use rand::Rng;

/**
 * 标准库也会被当作外部的包
 * 但是它已经内置在rust所以我们不需要在cargo.toml中修改关于std相关
 */

/**
 * 可以使用嵌套路径在清理大量的use语句
 * 如果使用同一个包或模块的多个条目
 * 可以使用嵌套路径在同一行内将上述条目进行引入
 * 格式
 * 路径相同的部分::{路径不同的部分}
 */
// use std::io;
// use std::cmp::Ordering;
// ===> 上述写法可以简写如下
// use std::{cmp::Ordering, io};

// 如果在嵌套路径中想引入自身可以使用self
// use std::io;
// use std::io::Write;
// ===>
// use std::io::{self, Write};

// 通配符 *
// 使用*可以把路径中所有的公共条目都引入到作用域中
// use std::*;
// 但是这种需要谨慎使用
// 应用场景如下
// 测试的时候将所有的测试代码引入到tests模块
// 有时被用于预导入模块


fn main() {
    to_do();
}
```