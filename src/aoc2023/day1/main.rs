fn get_calibration_value(s: &str) -> usize {
    let mut chars = s.chars();
    let first = chars.find(|c| c.is_ascii_digit()).unwrap();
    let second = chars.rfind(|c| c.is_ascii_digit()).unwrap_or(first);

    format!("{}{}", first, second).parse().unwrap()
}

#[allow(dead_code)]
pub fn solve1() {
    let file_contents = include_str!("input.txt");

    let sum = file_contents
        .split("\n")
        .map(get_calibration_value)
        .sum::<usize>();

    println!("Answer: {}", sum);
}

fn map_starts_with_valid_digit(s: &str) -> Option<char> {
    match s {
        _ if s.starts_with("one") => Some('1'),
        _ if s.starts_with("two") => Some('2'),
        _ if s.starts_with("three") => Some('3'),
        _ if s.starts_with("four") => Some('4'),
        _ if s.starts_with("five") => Some('5'),
        _ if s.starts_with("six") => Some('6'),
        _ if s.starts_with("seven") => Some('7'),
        _ if s.starts_with("eight") => Some('8'),
        _ if s.starts_with("nine") => Some('9'),
        c if c.chars().nth(0).is_some_and(|x| x.is_ascii_digit()) => c.chars().nth(0),
        _ => None,
    }
}

fn get_real_calibration_value(s: &str) -> usize {
    let first = (0..s.len())
        .map(|idx| &s[idx..])
        .find_map(map_starts_with_valid_digit)
        .unwrap();
    let second = (0..s.len())
        .rev()
        .map(|idx| &s[idx..])
        .find_map(map_starts_with_valid_digit)
        .unwrap();

    format!("{}{}", first, second).parse().unwrap()
}

#[allow(dead_code)]
pub fn solve2() {
    let file_contents = include_str!("input.txt");

    let sum = file_contents
        .split("\n")
        .map(get_real_calibration_value)
        .sum::<usize>();

    println!("Answer: {}", sum);
}
