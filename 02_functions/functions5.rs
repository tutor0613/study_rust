// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    println!("Input : {num}");
    num * num
}

fn main() {
    let num = 6;
    let answer = square(num);
    println!("The square of {num} is {answer}");
}
