use std::str::FromStr;

fn solve(){
    
}


fn main() {
    let times : usize = single_input();
    for _ in 0..times {
        solve();
    }
}


/**
 * This function reads a single line of input from stdin.
 */
#[allow(dead_code)]
fn input_line() -> String {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Failed to read line").to_string();
    input_text.trim().to_string()
}


/* 
 * This function is used to read a single value from the input
 * stream that implements FromStr trait.
 */
#[allow(dead_code)]
fn single_input<T:FromStr>() -> T {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Failed to read line");
    input_text.trim().parse().ok().unwrap()
}


/**
 * This function is used to read a vector of values from the input
 * stream that implements FromStr trait.
 */
#[allow(dead_code)]
fn input_vector<T:FromStr>() -> Vec<T> {
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let v: Vec<T> = input_text.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
    v
}

// TODO: ADD A DEBUG MACRO THAT PRINTS OUT VARIABLE NAME AND
// ITS VALUE