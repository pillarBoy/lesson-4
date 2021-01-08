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
pub const PI: f64 = 3.14159265358979323846264338327950288f64;

// 求面积函数
pub trait Summary {
    fn summarize(&self) -> String;
}

// 圆形
struct Rect {
    width: f64,
    height: f64,
}

// 如果给 struct 添加了泛型 那在impl 的时候这个泛型到底怎么处理
impl Summary for Rect {
    fn summarize(&self) -> String {
        let area = &self.width * &self.height;
        format!("{}", area)
    }
}


fn main() {
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;

    println!("red time is {}", red.time());
    println!("green time is {}", green.time());
    println!("yellow time is {}", yellow.time());

    let list = vec![2,34,5,6,7,8,10];
    println!("{:?}", sum(&list));

    let rect1 = Rect {
        width: 2.34,
        height: 3.45
    };

    println!("react area is {}", rect1.summarize());
}
