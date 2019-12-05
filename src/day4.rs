pub fn main4() {
    let mut part1_count = 0;
    let mut part2_count = 0;
    for n in 125730..579381 {
        let d = digits(n);
        // println!("{:?}", d);
        if adjacent(&d) && increase(&d) {
            part1_count += 1;
        }
        if strict_adjacent(&d) && increase(&d) {
            part2_count += 1;
        }
    }
    // println!("{}", part1_count);
    assert_eq!(part1_count, 2081);
    // println!("{}", part2_count);
    assert_eq!(part2_count, 1411);
}

fn _strict_adjacent(digits: &Vec<usize>) -> bool {
    let zip_inner = digits.iter().zip(digits.iter().skip(1));
    let zip_outer = std::iter::once(&std::usize::MAX)
        .chain(digits.iter())
        .zip(
            digits.iter()
                .skip(2)
                .chain(std::iter::once(&std::usize::MAX))
        );
    for (inner, outer) in zip_inner.zip(zip_outer) {
        if outer.0 != inner.0 && inner.0 == inner.1 && inner.1 != outer.1 {
            return true;
        }
    }
    return false;
}

fn strict_adjacent(digits: &Vec<usize>) -> bool {
    let mut digit0 = &std::usize::MAX;
    let mut digit1 = digits.get(0).unwrap();
    let mut digit2 = digits.get(1).unwrap();
    for digit3 in digits.iter().skip(2).chain(
            std::iter::once(&std::usize::MAX))
    {
        if digit0 != digit1 && 
            digit1 == digit2 && 
            digit2 != digit3 {
            return true;
        }
        digit0 = digit1;
        digit1 = digit2;
        digit2 = digit3;
    }
    return false;
}

fn increase(digits: &Vec<usize>) -> bool {
    let mut prev = digits.first().unwrap();
    for digit in digits.iter().skip(1) {
        if digit < prev {
            return false;
        }
        prev = digit;
    }
    return true;
}

fn adjacent(digits: &Vec<usize>) -> bool {
    let mut prev = digits.first().unwrap();
    for digit in digits.iter().skip(1) {
        // println!("{}, {}", prev, digit);
        if digit == prev {
            return true;
        }
        prev = digit;
    }
    return false;
}

fn digits(n: usize) -> Vec<usize> {
    fn digits_inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            digits_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    digits_inner(n, &mut xs);
    return xs;
}