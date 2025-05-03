/*
For Problem at,
https://exercism.org/tracks/rust/exercises/bottle-song
 */

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut ans = "".to_string();
    for i in 1..take_down+1
    {
        ans.push_str(&get_single_poem_phrase(start_bottles - i + 1));
        ans.push_str("\n");
    }
    ans
}

fn convert_int_to_name_of_number(num : u32) -> String
{
    match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "undefined"

    }.to_string()
}

fn capitalize_first_letter(value : &str) -> String
{
    let mut chars : Vec<char> = value.chars().collect();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    chars.into_iter().collect()
}

fn get_single_poem_phrase(num :u32) -> String
{
    let number_name = convert_int_to_name_of_number(num);
    let reduced_num_name = convert_int_to_name_of_number(num-1);
    let mut line_initial = capitalize_first_letter(&number_name) + " green bottles hanging on the wall,\n";
    let middle_line = "And if one green bottle should accidentally fall,\n".to_string();
    let mut last_line = "There'll be ".to_string() + &reduced_num_name+" green bottles hanging on the wall.\n";
    if num == 1
    {
        line_initial = "One green bottle hanging on the wall,\n".to_string();
        last_line = "There'll be no green bottles hanging on the wall.\n".to_string();
    }

    if num == 2
    {
        last_line = "There'll be one green bottle hanging on the wall.\n".to_string();
    }
    
    line_initial.push_str(&line_initial.clone());
    line_initial.push_str(&middle_line);
    line_initial.push_str(&last_line);
    line_initial
}
