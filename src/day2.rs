fn is_legal_p1(i: i32, c: i32) -> bool {
    let moves = match i {
        1 | 3 => vec![2, i + 3],
        2 | 8 => vec![i - 1, i + 1, 5],
        4 | 6 => vec![i - 3, 5, i + 3],
        5 => vec![2, 4, 6, 8],
        7 | 9 => vec![i - 3, 8],
        _ => vec![0], 
    };
    moves.contains(&(c + i))
}

fn is_legal_p2(i: i32, c: i32) -> bool {
    let moves = match i {
        1 => vec![3],
        2 | 4 => vec![3, i + 4],
        3 => vec![1, 2, 4, 7],
        5 => vec![6],
        6 | 7 | 8 => vec![i - 4, i - 1, i + 1, i + 4],
        9 => vec![8],
        10 | 12 => vec![i - 4, 11],
        11 => vec![7, 10, 12, 13],
        13 => vec![11],
        _ => vec![0],
    };
    moves.contains(&(c + i))
}

fn get_value_p1(i: char) -> i32 {
    match i {
        'D' => 3,
        'U' => -3,
        'L' => -1,
        'R' => 1,
        _ => 0,
    }
}

fn get_value_p2(i: char, c: i32) -> i32 {
    match i {
        'D' if c == 1 || c == 11 => 2,
        'U' if c == 3 || c == 13 => -2,
        'D' => 4,
        'U' => -4,
        'L' => -1,
        'R' => 1,
        _ => 0,
    }
}

pub fn bathroom_code() {
    let input = include_str!("d2");

    let mut a = 5;
    let mut b = 5;
    let codes: Vec<_> = input.lines()
        .map(|line| {
            a = line.chars()
                .map(get_value_p1)
                .fold(a, |acc, x| if is_legal_p1(acc, x) { acc + x } else { acc });
            b = line.chars().fold(b, |acc, e| {
                let x = get_value_p2(e, acc);
                if is_legal_p2(acc, x) { acc + x } else { acc }
            });
            (a, b)
        })
        .collect();

    let (p1, p2): (Vec<_>, Vec<_>) = codes.iter().cloned().unzip();

    println!("Answers \nPart 1: {:?} \nPart 2: {:?}", p1, p2);
}
