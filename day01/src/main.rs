const INPUT: &str = include_str!("input.txt");
const DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn parse_line(line: &str) -> Option<u32> {
    // For each line, we find the first digit (i.e. the digit with the minimum
    // start position out of all other digits) and then keep track of that
    // digit's index within the DIGITS array which is "sorted" such that the
    // integer value of each digit string is its position % 9 + 1
    let first = DIGITS
        .iter()
        // Enumerate to keep track of digit index
        .enumerate()
        .filter_map(|(digit_index, &digit)| {
            line.find(digit).map(|line_index| (line_index, digit_index))
        })
        // Use minimum LINE index
        .min_by(|&x, &y| x.0.cmp(&y.0))?
        // But use minimum digit index as value
        .1 as u32
        % 9
        + 1;
    // The last digit is the same concept but we use rfind instead to find the
    // last occurrence of the substring (remember repeats) and use the max line
    // index instead of min
    let last = DIGITS
        .iter()
        .enumerate()
        .filter_map(|(digit_index, &digit)| {
            line.rfind(digit)
                .map(|line_index| (line_index, digit_index))
        })
        .max_by(|&x, &y| x.0.cmp(&y.0))?
        .1 as u32
        % 9
        + 1;
    // Combining digits as strings is same as multiplying first digit by base
    // in this case 10
    Some(first as u32 * 10 + last as u32)
}

fn solution(input: &str) -> Option<u32> {
    input.lines().map(|line| parse_line(&line)).sum()
}

fn main() {
    println!("{}", solution(INPUT).unwrap());
}
