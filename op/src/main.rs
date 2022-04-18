fn main() {
    if let Some(result) = func_ex_div_some(8, 2) {
        println!("{}", result);
    } else {
        println!("None");
    }
}

fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    match y {
        0 => None,
        _ => Some(x / y),
    }
}
