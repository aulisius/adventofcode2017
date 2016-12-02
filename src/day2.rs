fn next_p1(i: i32, c: usize) -> i32 {
  let moves = match i {
    1 => [1, 1, 2, 4],
    2 | 3 => [i - 1, i, 3, i + 3],
    4 | 5 => [4, i - 3, i + 1, i + 3],
    6 | 9 => [i - 1, i - 3, i, 9],
    7 | 8 => [7, i - 3, i + 1, i],
    _ => [0, 0, 0, 0]
  };
  moves[c]
}

fn next_p2(i: i32, c: usize) -> i32 {
  let moves = match i {
    1 => [1, 1, 1, 3],
    2 => [2, 2, 3, 6],
    3 => [2, 1, 4, 7],
    4 => [3, 4, 4, 8],
    5 => [5, 5, 6, 5],
    6 | 7 | 8 => [i - 1, i - 4, i + 1, i + 4],
    9 => [8, 9, 9, 9],
    10 => [10, 6, 11, 10],
    11 => [10, 7, 12, 13],
    12 => [11, 8, 12, 12],
    13 => [13, 11, 13, 13],
    _ => [0, 0, 0, 0]
  };
  moves[c]
}

fn get_value(i: char) -> usize {match i {'L' => 0, 'U' => 1, 'R' => 2, 'D' => 3, _ => 0}}

pub fn bathroom_code() {
  let (p1, p2): (Vec<_>, Vec<_>) = include_str!("d2").lines()
    .map(|line| line.chars()
         .zip(line.chars())
         .map(|(x, y)| (get_value(x), get_value(y)))
         .fold((5, 5), |(a, b), (x, y)| (next_p1(a, x), next_p2(b, y)))
        )
    .unzip();
  println!("Answers\nPart 1: {:?}\nPart 2: {:?}", p1, p2);
}
