use std::io::prelude::*;
use std::fs::File;

enum Cardinals {
    North,
    South,
    East,
    West,
}

fn main() {

    let mut string = String::new();

    File::open("d1p1").unwrap().read_to_string(&mut string);

    let moves = string.lines()
        .map(|x| {
            let (tmp0, tmp1) = x.split_at(1);
            let stride: i32 = tmp1.parse().unwrap();
            (tmp0.to_string(), stride)
        })
        .fold((vec![(0, 0)], Cardinals::North), |mut acc, x| {
            let dir = match acc.1 {
                Cardinals::North => {
                    if x.0 == "L" {
                        Cardinals::West
                    } else {
                        Cardinals::East
                    }
                }
                Cardinals::West => {
                    if x.0 == "L" {
                        Cardinals::South
                    } else {
                        Cardinals::North
                    }
                }
                Cardinals::East => {
                    if x.0 == "L" {
                        Cardinals::North
                    } else {
                        Cardinals::South
                    }
                }
                Cardinals::South => {
                    if x.0 == "L" {
                        Cardinals::East
                    } else {
                        Cardinals::West
                    }
                }
            };

            let end: (i32, i32) = *acc.0.last().unwrap();

            let visited: Vec<(i32, i32)> = (1..x.1 + 1)
                .map(|x| {
                    match dir {
                        Cardinals::North => (end.0, end.1 + x),
                        Cardinals::South => (end.0, end.1 - x),
                        Cardinals::East => (end.0 + x, end.1),
                        Cardinals::West => (end.0 - x, end.1),
                    }
                })
                .collect();

            acc.0.extend(visited);
            (acc.0, dir)
        });

    let path = moves.0;

    let end = path.last().unwrap();

    let repeat: Vec<_> = path.iter().enumerate().filter_map(|(i, point)| {
        match path.split_at(i+1).1.contains(&point) {
            true => Some(point),
            false => None
        }
    }).collect(); 

    let overlap = repeat.first().unwrap();

    println!("Answers! Part 1: {} \n Part 2: {} ", end.0.abs() + end.1.abs(), (overlap.0.abs() + overlap.1.abs()).abs());

}
