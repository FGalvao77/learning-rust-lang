fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y
}

fn sub_numbers(x: i32, y: i32) -> i32 {
    let var_name = x - y;
    return var_name
}

fn mul_numbers(x: i32, y: i32) -> i32 {
    let var_name = x * y;
    var_name
}

fn div_number(x: i32, y: i32) -> i32 {
    x / y
}


fn main() {
    let res_add: i32 = add_numbers(8, 9);
    println!(r#"
    Result: {res_add}"#);

    let res_sub: i32 = sub_numbers(5, 12);
    println!(r#"
    Result: {res_sub}"#);

    let res_mul: i32 = mul_numbers(3, 5);
    println!(r#"
    Result: {res_mul}"#);

    let res_div: i32 = div_number(10, 5);
    println!(r#"
    Result: {res_div}"#);

    println!();
}