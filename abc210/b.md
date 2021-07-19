use proconio::input;

fn good_bad_card(card: String) -> String {
    // takahashi starts first 
    let mut is_takahaski = true;
    for c in card.chars(){
        if c=='1' {
            if is_takahaski {
                return "Takahashi".to_string();
            } else {
                return "Aoki".to_string();
            }
        }

        is_takahaski = !is_takahaski;
    }
    if is_takahaski {
        "Takahashi".to_string()
    } else{
        "Aoki".to_string()
    }
}


fn main() {
    input! {
        _n: i32,
        s: String
    }

    let loser = good_bad_card(s);
    println!("{}", &loser);
}