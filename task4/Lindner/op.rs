use std::ops::Mul;
use std::ops::Add;

use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;

use std::iter::Iterator;

fn main() {
    println!("do math result => {:?}", do_math(6, 3));

    let t = Swagger { object: "yo".to_string() };
    println!("{}", t);

    for i in Fib::new().take(20) {
        println!("{}", i);
    }
}

fn do_math<T: Mul + Add + Copy>
    (x: T,
     y: T)
     -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}

struct Swagger<T: Display> {
    object: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.object)
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

impl Iterator for Fib {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let temp = self.cur;
        self.cur = self.next;
        self.next += temp;

        Some(temp)
    }
}

