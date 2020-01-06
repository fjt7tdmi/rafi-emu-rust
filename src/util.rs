pub fn sext(count: usize, value: u32) -> u32 {
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
    assert_eq!(sext(15, 0x00008888), 0xffff8888);
    assert_eq!(sext(16, 0x00008888), 0x00008888);
}