fn main(){
    let mut console_high_score = 8_999;

    let y = &mut console_high_score;
    *y += 2;

    println!("\nConsole high socore is now {}\n", console_high_score);
}