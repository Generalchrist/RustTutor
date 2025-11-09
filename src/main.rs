fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array index 0: {:?}", fruits[0]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, true);
    println!("Human Tuple: {:?}", human);

    // Slices
    let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices: &[&str; 2] = &["Dog", "Cat"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String; 2] = &[&"It".to_string(), &"Zen".to_string()];
    println!("Book Slices: {:?}", book_slices);

    // Strings & String Slices (&str)
    let mut  stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah");
    println!("Stone Cold Says: {:?}",stone_cold);

}
