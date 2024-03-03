fn main(){
    let first_name: &str = "Fernando";
    println!("\nFisrt name: {:?}", first_name);

    let last_name: &str = "Galvao";
    println!("Last name: {:?}", last_name);

    println!("\nFull name: {:?} {:?}\n", first_name, last_name);


    let arr: [&str; 3] = [first_name, " ", last_name];
    let name_concatenated: &String = &arr.join("");

    println!("Full name: {:?}\n", name_concatenated);
}