use std::convert::TryFrom;
use std::collections::HashSet;

pub fn main3(input: String) {
    // Examples 1
    assert_eq!(closest(
        &get_points("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()),
        &get_points("U62,R66,U55,R34,D71,R55,D58,R83".to_string()),
    ), 159);
    assert_eq!(closest(
        &get_points("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string()),
        &get_points("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()),
    ), 135);
    // Part 1
    let wires = get_wires(input);
    part1(&wires);

    // Examples 2
    assert_eq!(best(
        &get_points("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string()),
        &get_points("U62,R66,U55,R34,D71,R55,D58,R83".to_string()),
    ), 610);
    assert_eq!(best(
        &get_points("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string()),
        &get_points("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()),
    ), 410);
    // Part 2
    part2(&wires);
    
}

pub fn get_wires(input: String) -> (Vec<Xy>, Vec<Xy>) {
    let mut wires = input.lines().map(|line| { get_points(line.to_string()) });
    let wire1 = wires.next().unwrap();
    let wire2 = wires.next().unwrap();
    return (wire1, wire2)
}

pub fn part1(wires: &(Vec<Xy>, Vec<Xy>)) {
    let part1 = closest(&wires.0, &wires.1);
    // println!("{}", part1);
    assert_eq!(part1, 293);
}

pub fn part2(wires: &(Vec<Xy>, Vec<Xy>)) {
    let part2 = best(&wires.0, &wires.1);
    // println!("{}", part2);
    assert_eq!(part2, 27306);
}

pub fn _0_intersection_points(first: &Vec<Xy>, second: &Vec<Xy>) -> Vec<Xy> {
    first.to_vec().into_iter()
        .filter(|xy| second.contains(xy))
        .collect::<Vec<Xy>>()
}

pub fn _1_intersection_points(first: &Vec<Xy>, second: &Vec<Xy>) -> Vec<Xy> {
    let mut second: Vec<Xy> = second.to_vec();
    second.sort_unstable();
    return first.iter().cloned()
        .filter(|xy| second.binary_search(xy).is_ok())
        .collect::<Vec<Xy>>()
}

pub fn _2_intersection_points(first: &Vec<Xy>, second: &Vec<Xy>) -> Vec<Xy> {
    let mut first: Vec<Xy> = first.to_vec();
    first.sort_unstable();
    return second.iter()
        .filter(|xy| first.binary_search(xy).is_ok())
        .cloned()
        .collect::<Vec<Xy>>()
}

pub fn _3_intersection_points(first: &Vec<Xy>, second: &Vec<Xy>) -> Vec<Xy> {
    let second: HashSet<Xy> = second.iter().cloned().collect();
    return first.iter().cloned()
        .filter(|xy| second.contains(xy))
        .collect::<Vec<Xy>>()
}

pub fn intersection_points(first: &Vec<Xy>, second: &Vec<Xy>) -> Vec<Xy> {
    let first: HashSet<Xy> = first.iter().cloned().collect();
    return second
        .iter()
        .filter(|xy| first.contains(xy))
        .cloned()
        .collect::<Vec<Xy>>()
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Xy(i32, i32);

enum PathSegmentDirection {
    R,
    D,
    L,
    U,
}

struct PathSegment {
    direction: PathSegmentDirection,
    distance: usize,
}

fn best(first: &Vec<Xy>, second: &Vec<Xy>) -> u32 {
    let int_points = intersection_points(first, second);
    best_steps(first, second, &int_points)
}

fn best_steps(first: &Vec<Xy>, second: &Vec<Xy>, int_points: &Vec<Xy>) -> u32 {
    u32::try_from(
        int_points.iter().map(|int_point| {
            first.iter().position(|p| p==int_point).unwrap() +
            second.iter().position(|p| p==int_point).unwrap() + 2
        }).min().unwrap()
    ).unwrap()
}

fn closest(first: &Vec<Xy>, second: &Vec<Xy>) -> u32 {
    closest_point(&intersection_points(&first, &second))
}

fn closest_point(points: &Vec<Xy>) -> u32 {
    u32::try_from(
        points.iter().map(|point| {
            point.0.abs() + point.1.abs()
        }).min().unwrap()
    ).unwrap()
}

fn get_points(path: String) -> Vec<Xy> {
    points_segments(parse_path(path))
}

fn points_segments(segments: Vec<PathSegment>) -> Vec<Xy> {
    let mut p: Vec<Xy> = Vec::new();
    let mut last = Xy(0,0);
    for segment in segments {
        let delta = segment_delta(&segment);
        for _ in 0..segment.distance {
            last.0 += delta.0;
            last.1 += delta.1;
            p.push(last.clone());
        }
    }
    return p;
}

fn segment_delta(segment: &PathSegment) -> Xy {
    match segment.direction {
        PathSegmentDirection::R => Xy(1,0),
        PathSegmentDirection::D => Xy(0,-1),
        PathSegmentDirection::L => Xy(-1,0),
        PathSegmentDirection::U => Xy(0,1),
    }
}

fn parse_path(path: String) -> Vec<PathSegment> {
    path.split(',')
        .map(|segment| {
            let distance = segment[1..].parse().unwrap();
            match segment.chars().next().unwrap() {
                'R' => PathSegment {
                    direction: PathSegmentDirection::R,
                    distance
                },
                'D' => PathSegment {
                    direction: PathSegmentDirection::D,
                    distance
                },
                'L' => PathSegment {
                    direction: PathSegmentDirection::L,
                    distance
                },
                'U' => PathSegment {
                    direction: PathSegmentDirection::U,
                    distance
                },
                _ => panic!(),
            }
        })
        .collect()
}
