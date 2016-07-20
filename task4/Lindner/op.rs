fn main() {
    println!("do math result => {:?}", do_math(6, 3));
}

fn do_math<T: std::ops::Mul + std::ops::Add + Copy>
    (x: T,
     y: T)
     -> (<T as std::ops::Add>::Output, <T as std::ops::Mul>::Output) {
    (x + y, x * y)
}
