minor bug n. 小bug
budget n. 预算
hotfix n. 热修复补丁
index n. 索引 v. 做索引
registery n. 仓库
bulk memory 大块内存
fuzz testing 模糊测试
ToC = Table of Content 目录

值语义(对应rust中的复制(copy)语义): 按位复制以后, 与原始对象无关
引用语义(对应rust中的移动(move)语义): 也叫指针语义. 一般是将数据存储于堆内存中,
通过栈内存的指针来管理堆内存的数据, 并且引用语义禁止按位复制.
浅复制: 栈复制(按位复制)
深复制: 栈复制+堆复制
let x = Box::new(5);
// x对Box<i32>拥有所有权
let y = x;
// x将Box<i32>通过移动转移给了y
// y拥有了Box<i32>的所有权
// 所以由y来释放Box<i32>的堆内存
// 一个值的所有权被转移给另外一个变量绑定的过程, 就叫做所有权转移
// 所有者 -控制-> 内存, 所有者负责内存的释放和读写权限
// 所有者唯一
// 所有者权限:
// 1. 控制的资源的释放
// 2. 出借所有权, 包括不可变(共享)的和可变的(独占)
// 3. 转移所有权
// let mut x = Sting::from("hello");
// let rx = &x; // 出借(不可变)
// let rx2 = &mut x; // 出借(可变)
// let y = x; // 转移
// 禁止实现Copy(栈复制)的类型, Box<T>, String...
// rust默认不会为成员全部是copy语义类型的struct, enum实现Copy, 但tuple会
// let a = "hello".to_string(); // a拥有字符串"hello"的所有权, let绑定了标识符a和存储字符串"hello"的那块内存
// let b = a; // 对a进行解绑, 然后重新绑定给b
// <变量绑定, 也叫绑定>的两个属性:
// 1. 空间属性
// 2. 时间属性
// {}创建词法作用域
// 所有权借用(borrow)
// 使用可变借用的前提, 出借所有权的绑定变量必须是一个可变绑定
// &x也被称为x的借用(borrowing)
// 通过&操作符来完成所有权租借
// 引用并不会造成绑定变量所有权的转移
// 引用在离开作用域之时, 就是其归还所有权之时
// 函数参数的模式匹配, Pt -> &Pt的自动转换
// 值(出借方)不可以跨越词法作用域
// 引用可以跨越词法作用域
// borrow checker: 借用检查器
// 函数或方法参数的生命周期叫做输入生命周期(input lifetime)
// 而返回值的生命周期被称为输出生命周期(output lifetime)
// output lifetime <= input lifetime
// 生命周期参数, 消除悬垂指针
// 智能指针拥有资源的所有权, 而普通引用只是对所有权的借用
// std::rc::Rc多个所有权
// weak<T>保留对Rc<T>中值的引用
// std::cell{Cell,RefCell}实现内部可变性
// NLL(non-lexical lifetime): 非词法作用域生命周期
// 装箱: 将值托管到堆内存
/*
drone架构:
1. git server
2. drone server(dispatch to runners(clusters))
3. drone runner
1 notify 2
3 poll 2
2 <=> 3 use manually gen secret to communicate

git clone to /drone/src/ directory
volume: /drone/src/target:/root/drone/target
*/
// reaper n. 收割者; 死神
// zombie n. 僵尸
// feedback n. 反馈
// internship n. 实习
// docker build cache的问题:
// FROM rust AS builder
// WORKDIR hello
// RUN echo 'fn main() {}' > dummy.rs
// COPY Cargo.toml .
// RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
// RUN cargo build --release
// COPY . .
// RUN cargo build --release
//
// FROM ubuntu
// COPY --from=builder /hello/target/release/hello /bin
// CMD ["/bin/hello"]
//
// 闭包特性:
// 1. 延迟执行
// 2. 捕获环境变量
//
// 1. 偏序(partial order): f32/f64
// 2. 全序(total order): i32/i64
// PartialEq: 部分等价(eq/ne)
// Eq: 等价
// PartialOrd: 偏序(partial_cmp, lt, le, gt, ge)
// Ord: 全序(cmp, max, min)
// std::cmp::Ordering;
// cheat sheet: n. 备忘单
// bloat: v. 膨胀
// lint: n. 静态分析(static analysis)
// wall-time: n. 实际时间
// moderate a. 适度的
// necessarily ad. 必然的
// a great deal of 大量的
// 通过&操作符来完成所有权租借, 引用并不会造成绑定变量所有权转移
// 1. 在不可变借用期间, 所有者不能修改资源, 并且也不能再进行可变借用
// 2. 在可变借用期间, 所有者不能访问资源, 并且也不能再出借所有权
// 引用在离开作用域之时, 就是其归还所有权之时
// 借用规则
// 1. 借用的生命周期不能长于出借方(拥有所有权的对象)的生命周期
// 2. rwlock
// 解引用操作会获得所有权
// Rust中的隐式类型转换: 自动解引用
// 如果一个类型T实现了Deref<Target=U>, 则该类型T的引用(或智能指针)在应用的时候会自动转换为类型U
// 单态化: monomorphization
// 代码膨胀: code bloat
// iterator adaptor
// wrapper design pattern
// 竞态条件(race condition): 当某个计算的正确性取决于多个线程交替执行的顺序时, 就会产生竞态条件
// 1. read-modify-write
// g_cnt += 1;
// 2. check-then-act
// if flag g_cnt += 1;
// 数据竞争(data race):