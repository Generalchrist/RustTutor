fn main() {
    hello_world();
    tell_height(195);
    human_id("Joel", 55, 182.3);
}

fn hello_world() {
    println!("Hello, World!");
}

fn tell_height(height: u32) {
    println!("Height is cm: {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm.",
        name, age, height
    );
}