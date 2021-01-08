use std::{f64::consts::PI, convert::Into};
use std::ops::{Mul, Add};
use std::fmt::Display;


// 1.1信号灯枚举
enum TrafficLight {
    Red,
    Green,
    Yellow
}

// 1.2信号灯 时间方法
impl TrafficLight {
    fn time(&self) -> u32 {
        match &self {
            TrafficLight::Red => 10_u32,
            TrafficLight::Green => 20_u32,
            TrafficLight::Yellow => 30_u32,
        }
    }
}


// 2.求和函数
fn sum(list: &[u32]) -> Option<u32> {

    if list.len() as u32 > u32::MAX {
        return None;
    }
    let mut s = 0;
    for item in list {
        s += item;
    }
    return Some(s);
}

// 3.求图形面积

// 求面积函数 Trait
pub trait Summary {
    fn summarize(&self);
}

// 3-1圆形
struct Round<T> {
    r: T,
}

impl<T: Into<f64> + Copy + Mul<Output = T> + Display> Summary for Round<T> {
    fn summarize(&self) {
        let rr = self.r * self.r;
        let area = T::into(rr) * PI;
        println!("{}", area); 
    }
}
// 3-2三角形
struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}
impl<T: Into<f64> + Add<Output = T> + Copy> Summary for Triangle<T> {
    fn summarize(&self) {
        let abc = self.a + self.b + self.c;
        let p = T::into(abc) / 2.0;
        let area = (p * (p - T::into(self.a)) * (p - T::into(self.b)) * (p - T::into(self.c))).sqrt();
        println!("{}", area); 
    }
}

// 3-3矩形
struct Rect<T> {
    width: T,
    height: T,
}

// 如果给 struct 添加了泛型 那在impl 的时候这个泛型到底怎么处理
impl<T: Into<f64> + Mul<Output = T> + Copy + Display> Summary for Rect<T> {
    fn summarize(&self)  {
        let area = self.width * self.height;
        println!("{}", area); 
    }
}

// 打印形状类型的函数
fn print_shape_area<T: Summary>(shape: T) {
    shape.summarize();
} 


fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    // 打印不同颜色的时间
    println!("red time is {}", red.time());
    println!("green time is {}", green.time());
    println!("yellow time is {}", yellow.time());

    // 计算vec 总和
    let list = vec![2,34,5,6,7,8,10];
    println!("{:?}", sum(&list));


    // 矩形
    let rect1 = Rect { width: 2.34, height: 3.45 };
    print_shape_area(rect1);
    // 三角形
    let tri = Triangle {a: 3.0, b: 4.0, c: 5.0};
    print_shape_area(tri);
    // 圆形
    let round1 = Round {r: 2.0};
    print_shape_area(round1);
}