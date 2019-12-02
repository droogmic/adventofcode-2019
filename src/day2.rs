pub fn main2() {
    let examples = [
        (vec![1,0,0,0,99], vec![2,0,0,0,99]),
        (vec![2,3,0,3,99], vec![2,3,0,6,99]),
        (vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
        (vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]),
    ];
    for example in examples.iter() {
        let (initial, expected) = example;
        let mut memory = initial.to_vec();
        //println!("{:#?}", memory);
        program(&mut memory);
        //println!("{:#?}", memory);
        assert_eq!(&memory, expected);
    }

    // Part 1
    let input = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,1,6,23,27,2,27,9,31,2,6,31,35,1,5,35,39,1,10,39,43,1,43,13,47,1,47,9,51,1,51,9,55,1,55,9,59,2,9,59,63,2,9,63,67,1,5,67,71,2,13,71,75,1,6,75,79,1,10,79,83,2,6,83,87,1,87,5,91,1,91,9,95,1,95,10,99,2,9,99,103,1,5,103,107,1,5,107,111,2,111,10,115,1,6,115,119,2,10,119,123,1,6,123,127,1,127,5,131,2,9,131,135,1,5,135,139,1,139,10,143,1,143,2,147,1,147,5,0,99,2,0,14,0];
    let mut memory = input.to_vec();
    memory[1] = 12; memory[2] = 2;
    program(&mut memory);
    //println!("{:?}", memory);
    assert_eq!(memory[0], 5305097);

    // Part 2
    let target = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = input.to_vec();
            memory[1] = noun; memory[2] = verb;
            program(&mut memory);
            if memory[0] == target {
                println!("{}", 100*noun + verb);
                return;
            }
        }
    }
}

fn program(memory: &mut Vec<usize>) {
    let mut pc = 0_usize;
    loop {
        if op(memory, pc) {
            break;
        }
        pc += 4;
    }
}

fn op(memory: &mut Vec<usize>, pc: usize) -> bool {
    match memory[pc] {
        1 => {
            let (left, right, result) = get_parameters(memory, pc).unwrap();
            memory[result] = memory[left] + memory[right];
        },
        2 => {
            let (left, right, result) = get_parameters(memory, pc).unwrap();
            memory[result] = memory[left] * memory[right];
        },
        99 => return true,
        _ => {
            eprintln!("Unknown op");
        }
    }
    return false;
}

fn get_parameters(memory: &Vec<usize>, pc: usize) -> Option<(usize, usize, usize)> {
    let left = memory.get(pc+1)?;
    let right = memory.get(pc+2)?;
    let result = memory.get(pc+3)?;
    return Some((*left, *right, *result));
}