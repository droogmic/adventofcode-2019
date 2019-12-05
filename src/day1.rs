pub fn main1(input: String) {
    let total_weight: u32 = input
        .lines()
        .map(|line| {
            let part_weight: u32 = line.parse().expect("Non integer input");
            calc(part_weight).expect("Part too light")
        })
        .sum();
    assert_eq!(3423279, total_weight);
    println!("{}", total_weight);

    let total_weight: u32 = input
        .lines()
        .map(|line| {
            let part_weight: u32 = line.parse().expect("Non integer input");
            let mut fuel_weight = calc(part_weight).expect("Part too light");
            let mut weight: u32 = 0;
            loop {
                weight += fuel_weight;
                fuel_weight = match calc(fuel_weight) {
                    Some(x) => x,
                    None => break,
                };
            }
            return weight;
        })
        .sum();
    assert_eq!(5132018, total_weight);
    println!("{}", total_weight);
}

fn calc(weight: u32) -> Option<u32> {
    (weight / 3).checked_sub(2)
}
