fn print<T>( a: &[T]) where T: std::fmt::Display {
    println!();
    for i in a {
        print!("{}, ", i);
    }
    println!();
}

struct Rectangle {
    width: f32,
    height: f32,
}

struct Triangle {
    base: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.base * self.height / 2.0
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}

trait ExprString {
    fn expr_str(&self) -> String {
        "幅 x 高さ = ".to_string()
    }
}

impl ExprString for Rectangle {}
impl ExprString for Triangle {
    fn expr_str(&self) -> String {
        "底辺 x 高さ / 2".to_string()
    }
}
impl ExprString for Circle {
    fn expr_str(&self) -> String {
        "半径 x 半径 x 3.13".to_string()
    }
}

trait ToNumber {
    fn to_i(&self) -> i32;
}
impl ToNumber for String {
    fn to_i(&self) -> i32 {
        match self.parse::<i32>() {
            Ok(n) => n,
            Err(e) => -1,
        }
    }
}

fn main() {
    let v = [10, 20, 30, 40, 50];
    print::<i32>(&v);
    print(&v);

    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    let tri = Triangle {
        base: 10.0,
        height: 20.0,
    };

    let cir = Circle {
        radius: 10.0,
    };

    println!("Rect area is {} {}", rect.expr_str(), rect.calc_area());
    println!("Tri area is {} {}", tri.expr_str(), tri.calc_area());
    println!("Cir area is {} {}", cir.expr_str(), cir.calc_area());

    let s = String::from("100");
    let n = s.to_i();
    println!("{} = {}", s, n);

}
