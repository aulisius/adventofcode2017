pub fn easter_bunny_hq() {
  let string = include_str!("d1"); // sed -r 's/,\s/\n/g'

  let (path, _) = string.lines().fold((vec![(0, 0)], (0, 1)), |(mut path, dir), e| {
    let (tmp0, tmp1) = e.split_at(1);
    let stride: i32 = tmp1.parse().unwrap();
    let ndir = if tmp0 == "L" { (-dir.1, dir.0) } else { (dir.1, -dir.0) };
    let end: (i32, i32) = *path.last().unwrap();

    let visited: Vec<_> = (1..stride + 1).map(|x| (end.0 + ndir.0 * x, end.1 + ndir.1 * x)).collect();

    path.extend(visited);
    (path, ndir)
  });

  let r = path.iter().enumerate().filter_map(|(i, p)| path.iter().skip(i + 1).find(|&x| x == p)).nth(0).map(|&(x,y)| x.abs() + y.abs()).unwrap();
  let e = path.last().map(|&(x,y)| x.abs() + y.abs()).unwrap();

  println!("Answers! \n Part 1: {} \n Part 2: {} ", e, r);
}

