use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const GEAR: char = '*';

fn solution(input: Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut parts = 0;
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        let mut line = line.iter().enumerate().peekable();
        while let Some(&(start, sym)) = line.peek() {
            if !sym.is_ascii_digit() {
                line.next();
                continue;
            }
            let number: String = line
                .by_ref()
                .take_while(|(_, c)| c.is_ascii_digit())
                .map(|t| t.1)
                .collect();
            let end = start + number.len();
            let number: usize = number.parse().ok()?;
            let bbox: Vec<(usize, usize, char)> = (start.max(1) - 1..=end)
                .flat_map(|n| [(row.max(1) - 1, n), (row, n), (row + 1, n)])
                .filter_map(|(r, c)| Some((r, c, *input.get(r).and_then(|line| line.get(c))?)))
                .collect();
            if bbox.iter().any(|t| !t.2.is_ascii_digit() && t.2 != '.') {
                parts += number;
            }
            for &(r, c, _) in bbox.iter().filter(|t| t.2 == GEAR) {
                gears.entry((r, c)).or_default().push(number);
            }
        }
    }
    let gear_ratio: usize = gears
        .values()
        .filter_map(|nums| Some(nums.first()? * nums.get(1)?))
        .sum();
    Some((parts, gear_ratio))
}

fn main() {
    let input = INPUT.lines().map(|line| line.chars().collect()).collect();
    let (s1, s2) = solution(input).unwrap();
    println!("Part 1: {:?}", s1);
    println!("Part 2: {:?}", s2);
}
