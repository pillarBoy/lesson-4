# Copy & clone

##### 不清晰的地方：

- 自动派生copy或clone，#[derive(Copy,Clone)]
- 一般不需要显式实现Copy
- Clone更慢，clone()不可缺省


Clone 相对与Copy 更慢，因为数据在堆上，堆上的所有数据都需要复制一份出来。(这个地方理解不是很到位，深入了解clone和copy)


- 1.什么类型在传值时会发生 copy, 什么类型在传值时会发生所有权转移？
- 2.什么是reference? 什么是Borrowing? 它们有什么区别?

# 泛型
- 优点 
    * 减少相似代码
    * 通过抽象，增加拓展性
    * 常用于结构体、枚举、函数签名
    * 多个参数时，trait bound 可以保证对应的类型时一致的
- 缺点 
    * 过多的泛型，降低代码可读性
- 注意点 
   * 编译时，泛型会被具体的类型替代，但是这不影响执行效率
   * 函数签名使用了过多的泛型，代码需要优化，提高可读性和可维护性


# 生命周期
- 一般变量是在它声明的大括号`{}`里就是它的生命周期
```rust
{
    let a = 1; 
    // ...
    // end
}
```
- 一个借用的变量在借用之前或者引用之前，就是它的生命周期

```rust
{
    let a = "123".to_string();
    let b = a; // end a 的生命周期
    // 这里之后不可使用a 变量
}
```

# 泛型(数字类型) 乘 一个指定类型 可以使用  Into 或者 FromeInto 去转换类型然后相乘
```rust
use std::convert::Into;

struct Round<T> {
    r: T
}

fn my_into<T: Into<f64>>(v: T) -> f64 {
    T::into(v)
}

pub const PI: f64 = 3.14159265358979323846264338327950288f64;

fn main() {
    // 简单使用
    let value = my_into(3) * 234.0982f64;
    println!("value is {}", value);

    // 或者
    let round1 = Round { r: 2 };
    // 这里转换一下round1.r 的类型
    let area = my_into(round1.r) * PI;
    println!("area is {}", area);
}

```