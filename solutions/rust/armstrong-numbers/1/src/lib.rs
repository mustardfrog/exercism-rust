pub fn is_armstrong_number(num: u32) -> bool {
    let s: Vec<_> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let arm = s.iter().map(|x| x.pow(s.len() as u32)).sum::<u32>();

    num == arm
}
