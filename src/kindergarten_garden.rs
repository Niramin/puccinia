/*
what is a &str?
what happens when you convert it to bytes?
what happens when you convert bytes to chars?
*/

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let number_of_students = get_number_of_students(diagram);
    let index_of_student = get_index_from_name(student);

    let row1 = get_row1_index_of_student(index_of_student);
    let row2 = get_row2_index_of_student(index_of_student, number_of_students);
    let mut result: Vec<&'static str> = Vec::new();
    let raw_string: String =  diagram.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = raw_string.as_bytes();
    result.push(get_plant_name_from_character(bytes[row1] as char));
    result.push(get_plant_name_from_character(bytes[row1 + 1] as char));
    result.push(get_plant_name_from_character(bytes[row2] as char));
    result.push(get_plant_name_from_character(bytes[row2 + 1] as char));
    result

}

fn get_index_from_name(name: &str) -> u16 
{
    match name  {
      "Alice" => 0,
      "Bob" => 1,
      "Charlie" => 2,
      "David" => 3,
      "Eve" => 4,
      "Fred" => 5,
      "Ginny" => 6,
      "Harriet" => 7,
      "Ileana" => 8,
      "Joseph" => 9,
      "Kincaid" => 10,
      "Larry" => 11,
       _ => 20  
    }
}

fn get_number_of_students(diagram:&str) -> usize
{
    diagram.len()/4
}

fn get_row1_index_of_student(index: u16) -> usize
{
    let ans = index*2;
    ans.into()
}

fn get_row2_index_of_student(index: u16, no_of_students: usize) -> usize
{
    let student_index : usize = index.into();
    (no_of_students*2) + student_index*2

}

fn get_plant_name_from_character(character: char) -> &'static str 
{
    match character {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "undefined"
    }
}