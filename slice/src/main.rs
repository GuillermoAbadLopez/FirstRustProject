fn main(){
    let s1: String = String::from("hello");
    let s2: String = s1.clone();

    // Without using reference, the ownership is moved to the function.
    printing(s1);
    // printing(s1); // This would error in compile, since s1 is moved to printing function.
    
    // Using reference, the ownership is not moved to the function.
    printing_multiple(&s2);
    printing_multiple(&s2);
}

fn printing(s: String){
    println!("{}", s);
}

fn printing_multiple(s: &String){
    println!("{}", s);
}