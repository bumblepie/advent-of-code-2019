use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Unable to open file");
    let (lines, errs): (Vec<_>, Vec<_>) = BufReader::new(file).lines().partition(Result::is_ok);
    if !errs.is_empty() {
        for err in errs.into_iter().filter_map(Result::err) {
            eprintln!("{}", err);
        }
        process::exit(1);
    }
    let lines: Vec<String> = lines.into_iter().filter_map(Result::ok).collect();
    println!("Part 1: {:?}", part_1(&lines));
    println!("Part 2: {:?}", part_2(&lines));
}

fn part_1(input: &Vec<String>) -> Option<i32> {
    let wires: Vec<_> = input
        .into_iter()
        .map(|wire_str| parse_wire_segments(wire_str))
        .collect();
    intersections(&wires)
        .into_iter()
        .flat_map(|(_seg1, _seg2, intersection)| {
            (intersection.left..intersection.right + 1)
                .cartesian_product(intersection.bottom..intersection.top + 1)
                .collect::<Vec<_>>()
        })
        .filter(|&pos| pos != (0, 0))
        .map(|(x, y)| i32::abs(x) + i32::abs(y))
        .min()
}

fn part_2(input: &Vec<String>) -> Option<i32> {
    let wires: Vec<_> = input
        .into_iter()
        .map(|wire_str| parse_wire_segments(wire_str))
        .collect();
    intersections(&wires)
        .into_iter()
        .flat_map(|(seg1, seg2, intersection)| {
            (intersection.left..intersection.right + 1)
                .cartesian_product(intersection.bottom..intersection.top + 1)
                .map(|(x, y)| {
                    let total_length = seg1.prev_length
                        + seg2.prev_length
                        + i32::abs(x - seg1.starting_pt.0)
                        + i32::abs(x - seg2.starting_pt.0)
                        + i32::abs(y - seg1.starting_pt.1)
                        + i32::abs(y - seg2.starting_pt.1);
                    ((x, y), total_length)
                })
                .collect::<Vec<_>>()
        })
        .filter(|(pos, _length)| *pos != (0, 0))
        .map(|(_pos, length)| length)
        .min()
}

#[derive(Clone, Debug)]
struct WireSegment {
    bounds: BoundingBox,
    prev_length: i32,
    starting_pt: (i32, i32),
}

#[derive(Clone, Debug)]
struct BoundingBox {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

type Wire = Vec<WireSegment>;

fn parse_wire_segments(input: &String) -> Wire {
    let (_final_pos, _length, segments) = input
        .split(',')
        .map(|s| s.split_at(1))
        .map(|(a, b)| (a.to_string(), b.parse::<i32>().unwrap()))
        .fold(
            ((0, 0), 0, Vec::new()),
            |(position, prev_length, mut segments), (dir, length)| {
                let mut next_segment = WireSegment {
                    bounds: BoundingBox {
                        left: position.0,
                        right: position.0,
                        top: position.1,
                        bottom: position.1,
                    },
                    prev_length,
                    starting_pt: position,
                };
                let mut new_pos = position.clone();
                match dir.as_ref() {
                    "U" => {
                        next_segment.bounds.top -= length;
                        new_pos.1 -= length
                    }
                    "D" => {
                        next_segment.bounds.bottom += length;
                        new_pos.1 += length
                    }
                    "L" => {
                        next_segment.bounds.left -= length;
                        new_pos.0 -= length
                    }
                    "R" => {
                        next_segment.bounds.right += length;
                        new_pos.0 += length
                    }
                    _ => unreachable!(),
                };
                segments.push(next_segment);
                (new_pos, prev_length + length, segments)
            },
        );
    segments
}

fn intersections(wires: &Vec<Wire>) -> Vec<(WireSegment, WireSegment, BoundingBox)> {
    wires
        .into_iter()
        // get all different combos of wires
        .combinations(2)
        .flat_map(|wire_combo| {
            let wire1 = wire_combo[0];
            let wire2 = wire_combo[1];
            // get all combos of wire segments for this combo
            wire1.into_iter().cartesian_product(wire2)
        })
        .filter_map(|(seg1, seg2)| {
            let intersection = BoundingBox {
                top: i32::max(seg1.bounds.top, seg2.bounds.top),
                bottom: i32::min(seg1.bounds.bottom, seg2.bounds.bottom),
                left: i32::max(seg1.bounds.left, seg2.bounds.left),
                right: i32::min(seg1.bounds.right, seg2.bounds.right),
            };
            if intersection.top <= intersection.bottom && intersection.left <= intersection.right {
                Some((seg1, seg2, intersection))
            } else {
                None
            }
        })
        .map(|(seg1, seg2, intersection)| (seg1.clone(), seg2.clone(), intersection))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1_1() {
        assert_eq!(
            part_1(&vec![
                "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
                "U62,R66,U55,R34,D71,R55,D58,R83".to_owned()
            ]),
            Some(159)
        );
    }
    #[test]
    fn example_p1_2() {
        assert_eq!(
            part_1(&vec![
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned()
            ]),
            Some(135)
        );
    }
    #[test]
    fn example_p2_1() {
        assert_eq!(
            part_2(&vec![
                "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
                "U62,R66,U55,R34,D71,R55,D58,R83".to_owned()
            ]),
            Some(610)
        );
    }

    #[test]
    fn example_p2_2() {
        assert_eq!(
            part_2(&vec![
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned()
            ]),
            Some(410)
        );
    }
}
