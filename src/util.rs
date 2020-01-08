pub fn get_int_reg_name(index: usize) -> String {
    match index {
        0 => "zero".to_string(),
        1 => "ra".to_string(),
        2 => "sp".to_string(),
        3 => "gp".to_string(),
        4 => "tp".to_string(),
        5 => "t0".to_string(),
        6 => "t1".to_string(),
        7 => "t2".to_string(),
        8 => "s0".to_string(),
        9 => "s1".to_string(),
        10 => "a0".to_string(),
        11 => "a1".to_string(),
        12 => "a2".to_string(),
        13 => "a3".to_string(),
        14 => "a4".to_string(),
        15 => "a5".to_string(),
        16 => "a6".to_string(),
        17 => "a7".to_string(),
        18 => "s2".to_string(),
        19 => "s3".to_string(),
        20 => "s4".to_string(),
        21 => "s5".to_string(),
        22 => "s6".to_string(),
        23 => "s7".to_string(),
        24 => "s8".to_string(),
        25 => "s9".to_string(),
        26 => "s10".to_string(),
        27 => "s11".to_string(),
        28 => "t3".to_string(),
        29 => "t4".to_string(),
        30 => "t5".to_string(),
        31 => "t6".to_string(),
        _ => "INVALID".to_string(),
    }
}

pub fn pick(value: &u32, lsb: usize, width: usize) -> u32 {
    (value >> lsb) & ((1 << width) - 1)
}

pub fn sign_extend(count: usize, value: u32) -> u32 {
    if value & (1 << count) != 0 {
        let mask = !((1 << count) - 1);
        return mask | value
    }
    else {
        return value
    }
}

#[test]
fn sext_test() {
    assert_eq!(sign_extend(15, 0x00008888), 0xffff8888);
    assert_eq!(sign_extend(16, 0x00008888), 0x00008888);
}
