pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.is_empty()
    {
        Vec::new()
    }

    else if len == 0 {
        panic!("length cannot be 0");
    }
    else {
        let mut ans : Vec<String> = Vec::new();
        for i in 0..digits.len(){
            let start = i;
            let end = i+ len;
            if end<digits.len()+1
            {
                ans.push(digits[start..end].to_owned());
            }
            else
            {
                break;
            }
        }
        ans
    }
}
