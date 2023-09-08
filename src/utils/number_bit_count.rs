pub fn number_bit_count(mut num: usize) -> usize {
    if num == 0 {
        return 1;
    }

    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    return count;
}
