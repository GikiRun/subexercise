// 06 打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束
pub fn get_area<T: HasArea>(shape: &T) -> f64 {
    shape.area()
}
// 图形面积trait，计算并返回不同图形的面积
pub trait HasArea {
    fn area(&self) -> f64;
    fn is_valid(&self) -> bool;
}
// 圆形
#[derive(Debug)]
pub struct Circle {
    radius: f64,
}
// 正方形
#[derive(Debug)]
pub struct Square {
    side: f64,
}
// 三角形
#[derive(Debug)]
pub struct Triangle(f64, f64, f64);

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Self { radius }
    }
}
//计算圆形面积
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn is_valid(&self) -> bool {
        self.radius > 0f64
    }
}

impl Square {
    pub fn new(side: f64) -> Square {
        Self { side }
    }
}
// 计算正方形面积
impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn is_valid(&self) -> bool {
        self.side > 0f64
    }
}

impl Triangle {
    pub fn new(l1: f64, l2: f64, l3: f64) -> Triangle {
        Self(l1, l2, l3)
    }
}
// 计算三角形面积
impl HasArea for Triangle {
    fn area(&self) -> f64 {
        let x = (self.0 + self.1 + self.2) / 2f64;
        (x * (x - self.0) * (x - self.1) * (x - self.2)).sqrt()
    }
    fn is_valid(&self) -> bool {
        let v0 = self.0 + self.1 > self.2;
        let v1 = self.0 + self.2 > self.1;
        let v2 = self.1 + self.2 > self.0;
        let v3 = self.0 > 0f64 && self.1 > 0f64 && self.2 > 0f64;
        v0 && v1 && v2 && v3
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_triangle() {
        assert!(!Triangle::new(0f64, 0f64, 0f64).is_valid());
        assert!(!Triangle::new(1f64, 2f64, 0f64).is_valid());
        assert!(!Triangle::new(1f64, 2f64, 3f64).is_valid());
        assert!(!Triangle::new(1f64, 3f64, 2f64).is_valid());
        assert!(!Triangle::new(2f64, 3f64, 1f64).is_valid());
        assert!(!Triangle::new(2f64, 3f64, 1f64).is_valid());
        assert!(!Triangle::new(3f64, 2f64, 1f64).is_valid());
        assert!(!Triangle::new(3f64, 1f64, 2f64).is_valid());

        let x = Triangle::new(6f64, 8f64, 10f64);
        assert!(x.is_valid());
        assert_eq!(x.area(), 24f64);
    }

    #[test]
    fn test_circle() {
        let circle = Circle::new(1f64);
        assert!(circle.is_valid());
        assert_eq!(circle.area(), std::f64::consts::PI * 1f64 * 1f64);
        assert!(!Circle::new(0f64).is_valid());
    }

    #[test]
    fn test_square() {
        assert!(!Square::new(0f64).is_valid());

        let square = Square::new(5f64);
        assert!(square.is_valid());
        assert_eq!(square.area(), 25f64);
    }
}
