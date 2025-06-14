pub fn square(s: u32) -> u64 {
    let power = s -1;
    let base : u64 = 2;
    base.pow(power)
}

pub fn total() -> u64 {
    (1..65)
    .map(square)
    .sum()
}
