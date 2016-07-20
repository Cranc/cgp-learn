use std::fmt;

fn main() {
    println!("do math result => {:?}", do_math(6, 3));

    let t = Swagger { object: "yo".to_string() };
    println!("{}", t);
}

fn do_math<T: std::ops::Mul + std::ops::Add + Copy>
    (x: T,
     y: T)
     -> (<T as std::ops::Add>::Output, <T as std::ops::Mul>::Output) {
    (x + y, x * y)
}

struct Swagger<T: fmt::Display> {
    object: T,
}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "swag {} yolo", self.object)
    }
}
