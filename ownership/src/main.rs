fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Using s1.clone() would copy the value, keeping s1 valid.
    // println!("{}, world!", s1); # This in the current code would error, since s1 is deleted automatically.
    println!("{}, world!", s2); 
}
