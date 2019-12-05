// use std::convert::TryFrom;
// use std::collections::HashSet;
use std::collections::HashMap;

pub fn main6(input: String) {
    // Example 1
    let map = String::from("
        COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
    ");
    let map = parse_input(&map);
    let planets = get_planets(&map);
    let orbits = get_orbits(&planets, "COM", 0);
    // println!("{}", orbits);
    assert_eq!(orbits, 42);

    // Part 1
    let map = parse_input(&input);
    let planets = get_planets(&map);
    let orbits = get_orbits(&planets, "COM", 0);
    // println!("{}", orbits);
    assert_eq!(orbits, 144909);

    // Part 1
    let map = parse_input(&input);
    let orbiting = get_orbiting(&map);
    let mut you_path = get_path(&orbiting, "YOU");
    let mut san_path = get_path(&orbiting, "SAN");
    while you_path.pop().unwrap() == san_path.pop().unwrap() {
    }
    let transfers = you_path.len() + 1 + san_path.len() + 1;
    // println!("{}", transfers);
    assert_eq!(transfers, 259)
}

fn parse_input(input: &String) -> Vec<(&str, &str)> {
    let map: Vec<(&str, &str)> = input
        .lines()
        .filter_map(|line| {
            let relation = line.trim();
            if relation.is_empty() {
                return None;
            }
            let mut relation: Vec<&str> = relation.split(")").collect();
            if relation.len() != 2 {
                panic!("Invalid relationship");
            }
            let orbiting = relation.pop().unwrap();
            let orbited = relation.pop().unwrap();
            Some((orbited, orbiting))
        })
        .collect();
    return map;
}

fn get_planets<'a>(map: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut planets = HashMap::new();
    for relation in map.into_iter() {
        let planet = planets.entry(relation.0).or_insert(Vec::new());
        planet.push(relation.1);
    }
    // println!("{:?}", planets);
    return planets;
}

fn get_orbiting<'a>(map: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, &'a str> {
    let mut planets = HashMap::new();
    for relation in map.into_iter() {
        planets.insert(relation.1, relation.0);
    }
    // println!("{:?}", planets);
    return planets;
}

fn get_orbits(planets: &HashMap<&str, Vec<&str>>, com: &str, depth: usize) -> usize {
    let mut orbits = 0;
    for orbiting in planets.get(com).unwrap_or(&Vec::new()) {
        orbits += depth + 1;
        orbits += get_orbits(&planets, orbiting, depth+1);
    }
    orbits
}

fn get_path<'a>(orbiting: &HashMap<&'a str, &'a str>, start: &str) -> Vec<&'a str> {
    let mut path: Vec<&str> = Vec::new();
    loop {
        let key = path.last().unwrap_or(&start);
        match orbiting.get(key) {
            Some(val) => path.push(val),
            None => break,
        }
    }
    path
}
