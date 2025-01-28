// Define the `call_me` function with a parameter `num` of type `i32`
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // Call the `call_me` function with the argument `3`
    call_me(3);
}