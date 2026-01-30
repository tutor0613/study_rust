fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(0x1F);
}
