```rust
use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        x: i32,
        y: i32
    }

    let mut cost = 0;

    if n>a {
        cost += (a*x) + (n-a)*y;
    } else {
        cost += n*x;
    }
    
    println!("{}", &cost);
}

```