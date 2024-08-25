use time::PrimitiveDateTime as DateTime;


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = time::Duration::SECOND * 1000000000;
    let ans = start + gigasecond;
    return ans
}
