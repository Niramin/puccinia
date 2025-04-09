
pub fn is_armstrong_number(num: u32) -> bool {
    get_sum_of_numbers_raised_to_power(get_all_digits(num), get_number_of_digits(num)) == num
}

fn get_number_of_digits(num: u32) -> u32 
{
    let mut no_of_digits = 0;
    let mut number = num;
    while number != 0 
    {
        no_of_digits += 1;
        number /= 10;
    }
    no_of_digits
}

fn get_all_digits(num: u32) -> Vec<u32>
{
    let mut digits : Vec<u32> = Vec::new();
    let mut number = num;
    while number != 0 
    {
        digits.push(number%10);
        number /= 10;
    }
    digits

}

fn get_sum_of_numbers_raised_to_power(list : Vec<u32>, power:u32) -> u32
{
    let mut sum = 0;
    for digit in list
    {
        sum += digit.pow(power);
    }
    sum

}