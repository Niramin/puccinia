pub mod anagram;
pub mod  gigasecond;
pub mod clock;
pub mod diffie_hellman_key_exchange;
pub mod series;
pub mod armstrong;
pub mod kindergarten_garden;

pub fn reverse(input: &str) -> String {
    let mut revString = String::new();
    let revVector = input.chars().rev();
    for c in revVector
    {
        revString.push(c);
    }

    return revString;

}

fn main() {
    println!("Hello, world!");
}


