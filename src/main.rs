pub mod anagram;
pub mod  gigasecond;

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


