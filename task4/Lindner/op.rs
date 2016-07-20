use std::fmt;

fn main() {
    println!("do math result => {:?}", do_math(6, 3));

    let t = Swagger { object: "yo".to_string() };
    println!("{}", t);

    for i in Fib::new().take(20) {
        println!("{}", i);
    }
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

struct Fib {
    next: usize,
    cur: usize,
}

impl Fib {
    fn new() -> Fib {
        Fib { next: 1, cur: 0 }
    }
}

impl std::iter::Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let temp = self.cur;
        self.cur = self.next;
        self.next += temp;

        Some(temp)
    }
}

