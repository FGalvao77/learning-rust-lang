fn main(){
    /*
    declarete variables
     */
    
    let arr: [&str; 3] = ["Hello", ", Fernando", "!"];
    let concatenated: &String = &arr.join("");

    println!("\n{:?}\n", concatenated);
}