const INPUT: &str = include_str!("input.txt");

fn main() {
    let wins = INPUT.lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|line| line.1.split_once(" | "))
        .map(|t| (t.0.split_whitespace().collect::<Vec<&str>>(), t.1.split_whitespace()))
        .map(|(wins, nums)| nums.filter(|num| wins.contains(num)).count())
        .collect::<Vec<usize>>();
    let part1: usize = wins.iter().map(|&n| if n == 0 { 0 } else { 1 << (n - 1) }).sum();
    let mut coeffs = vec![1; wins.len()];
    for (i, &v) in wins.iter().enumerate() {
        for j in 0..v {
            coeffs[j+i+1] += coeffs[i];
        }
    }
    let part2: usize = wins.iter().zip(coeffs).map(|(p, c)| 1 + p * c).sum();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
