use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    base_raised_to_exponent_mod_number(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    base_raised_to_exponent_mod_number(b_pub, a, p)
}


fn base_raised_to_exponent_mod_number(base: u64, exponent: u64, modulus: u64) -> u64
{
    let mut ans = 1;
    for _i in 0..exponent  {
        ans = ans * (base % modulus);
    }
    ans = ans % modulus;
    ans
}

fn base_raised_to_exponent_mod_number_fast(base: u64, exponent: u64, modulus: u64) -> u64
{
    if modulus == 1 
    {
        return 0;
    }

    let mut Result = 1;

    let mut Base =  base % modulus;

    let mut Exponent = exponent;

    while Exponent >0 {
        if Exponent % 2 == 1
        {
            Result = (Result * Base) % modulus;
        }
        Exponent = Exponent >> 1;
        Base = (Base * Base) % modulus;
    }
    Result
}