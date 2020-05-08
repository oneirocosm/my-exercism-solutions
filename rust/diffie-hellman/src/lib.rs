use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

fn mod_exp(mut base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let mut output = 1;
    while exp > 0 {
        if (exp & 1) == 1 {
            output = (output * base) % m;
        }
        exp >>= 1;
        base = (base * base) % m;
    }
    output
}
