use std::convert::TryFrom;

pub fn main5() {
    // Day 2
    let examples = [
        (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];
    for example in examples.iter() {
        let (initial, expected) = example;
        let mut memory = initial.to_vec();
        //println!("{:#?}", memory);
        program(&mut memory, &vec![]);
        //println!("{:#?}", memory);
        assert_eq!(&memory, expected);
    }

    // Part 1
    let input = [
        3,225,1,225,6,6,1100,1,238,225,104,0,101,20,183,224,101,-63,224,224,4,224,1002,223,8,223,101,6,224,224,1,223,224,223,1101,48,40,225,1101,15,74,225,2,191,40,224,1001,224,-5624,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1101,62,60,225,1102,92,15,225,102,59,70,224,101,-885,224,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,1,35,188,224,1001,224,-84,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1001,66,5,224,1001,224,-65,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1002,218,74,224,101,-2960,224,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1101,49,55,224,1001,224,-104,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1102,43,46,225,1102,7,36,225,1102,76,30,225,1102,24,75,224,101,-1800,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1101,43,40,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,226,226,224,1002,223,2,223,1005,224,329,1001,223,1,223,8,226,677,224,102,2,223,223,1006,224,344,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,359,101,1,223,223,1008,677,226,224,102,2,223,223,1006,224,374,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,389,1001,223,1,223,107,677,677,224,1002,223,2,223,1006,224,404,101,1,223,223,1007,226,226,224,1002,223,2,223,1006,224,419,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,434,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,449,101,1,223,223,107,226,226,224,1002,223,2,223,1006,224,464,1001,223,1,223,1108,677,677,224,1002,223,2,223,1005,224,479,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,494,101,1,223,223,7,226,677,224,102,2,223,223,1005,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,1001,223,1,223,1108,677,226,224,1002,223,2,223,1005,224,539,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,554,101,1,223,223,108,226,677,224,102,2,223,223,1005,224,569,1001,223,1,223,8,677,677,224,1002,223,2,223,1005,224,584,101,1,223,223,108,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,108,226,226,224,102,2,223,223,1006,224,614,101,1,223,223,1008,677,677,224,102,2,223,223,1006,224,629,1001,223,1,223,107,226,677,224,102,2,223,223,1006,224,644,101,1,223,223,1107,677,677,224,1002,223,2,223,1005,224,659,1001,223,1,223,7,226,226,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226
    ];
    let output = program(&mut input.to_vec(), &vec![1]);
    // println!("{:?}", output);
    assert_eq!(*output.last().unwrap(), 13346482);

    // Examples 2

    // Equal to 8
    let example1 = [3,9,8,9,10,9,4,9,99,-1,8];
    let output = program(&mut example1.to_vec(), &vec![7]);
    // println!("{:?}", output);
    assert_eq!(output[0], 0);
    let output = program(&mut example1.to_vec(), &vec![9]);
    // println!("{:?}", output);
    assert_eq!(output[0], 0);
    let output = program(&mut example1.to_vec(), &vec![8]);
    // println!("{:?}", output);
    assert_eq!(output[0], 1);

    // Less than 8
    let example2 = [3,9,7,9,10,9,4,9,99,-1,8];
    let output = program(&mut example2.to_vec(), &vec![7]);
    assert_eq!(output[0], 1);
    let output = program(&mut example2.to_vec(), &vec![8]);
    assert_eq!(output[0], 0);

    // Equal to 8
    let example3 = [3,3,1108,-1,8,3,4,3,99];
    let output = program(&mut example3.to_vec(), &vec![7]);
    // println!("{:?}", output);
    assert_eq!(output[0], 0);
    let output = program(&mut example3.to_vec(), &vec![9]);
    // println!("{:?}", output);
    assert_eq!(output[0], 0);
    let output = program(&mut example3.to_vec(), &vec![8]);
    // println!("{:?}", output);
    assert_eq!(output[0], 1);

    // Less than 8
    let example4 = [3,3,1107,-1,8,3,4,3,99];
    let output = program(&mut example4.to_vec(), &vec![7]);
    assert_eq!(output[0], 1);
    let output = program(&mut example4.to_vec(), &vec![8]);
    assert_eq!(output[0], 0);

    // Jump tests
    let example5 = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    let output = program(&mut example5.to_vec(), &vec![0]);
    assert_eq!(output[0], 0);
    let output = program(&mut example5.to_vec(), &vec![1]);
    assert_eq!(output[0], 1);
    let output = program(&mut example5.to_vec(), &vec![2]);
    assert_eq!(output[0], 1);

    // Jump tests
    let example6 = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    let output = program(&mut example6.to_vec(), &vec![0]);
    assert_eq!(output[0], 0);
    let output = program(&mut example6.to_vec(), &vec![1]);
    assert_eq!(output[0], 1);
    let output = program(&mut example6.to_vec(), &vec![2]);
    assert_eq!(output[0], 1);

    // Part 2
    let output = program(&mut input.to_vec(), &vec![5]);
    println!("{:?}", output);
    assert_eq!(*output.last().unwrap(), 12111395);
}

fn program(memory: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut input = input.clone();
    let mut output = Vec::new();
    let mut pc = 0_usize;
    input.reverse();
    // println!("input: {:?}, memory: {:?}", input, memory);
    loop {
        let op = memory[pc] % 100;
        let modes = (
            (memory[pc]/10000) % 10,
            (memory[pc]/1000) % 10,
            (memory[pc]/100) % 10,
        );
        match op {
            1 => {
                let (result, (left, right)) = get_3_parameters(memory, pc, modes);
                memory[result] = left + right;
                pc += 4;
            }
            2 => {
                let (result, (left, right)) = get_3_parameters(memory, pc, modes);
                memory[result] = left * right;
                pc += 4;
            }
            3 => {
                let param = usize::try_from(*memory.get(pc + 1).unwrap()).unwrap();
                memory[param] = input.pop().unwrap();
                pc += 2;
            }
            4 => {
                let param = usize::try_from(*memory.get(pc + 1).unwrap()).unwrap();
                output.push(memory[param]);
                pc += 2;
            }
            5 => {
                let (check, target) = get_jump_parameters(memory, pc, modes);
                if check != 0 {
                    pc = target
                } else {
                    pc += 3;
                }
            }
            6 => {
                let (check, target) = get_jump_parameters(memory, pc, modes);
                if check == 0 {
                    pc = target
                } else {
                    pc += 3;
                }
            }
            7 => {
                let (result, (left, right)) = get_3_parameters(memory, pc, modes);
                memory[result] = i32::from(left < right);
                pc += 4;
            }
            8 => {
                let (result, (left, right)) = get_3_parameters(memory, pc, modes);
                memory[result] = i32::from(left == right);
                pc += 4;
            }
            99 => break,
            x => panic!("Unknown op: {} from {:?}", x, memory[pc]),
        }
        // println!("pc: {}, memory: {:?}", pc, memory);
    }
    return output;
}

fn get_jump_parameters(memory: &Vec<i32>, pc: usize, modes: (i32, i32, i32)) -> (i32, usize) {
    let mut condition = *memory.get(pc + 1).unwrap();
    if modes.2 == 0 {
        condition = memory[usize::try_from(condition).unwrap()]
    }
    let mut target = usize::try_from(*memory.get(pc + 2).unwrap()).unwrap();
    if modes.1 == 0 {
        target = usize::try_from(memory[target]).unwrap()
    }
    return (condition, target)
}

fn get_3_parameters(memory: &Vec<i32>, pc: usize, modes: (i32, i32, i32)) -> (usize, (i32, i32)) {
    let mut left = *memory.get(pc + 1).unwrap();
    if modes.2 == 0 {
        left = memory[usize::try_from(left).unwrap()]
    }
    let mut right = *memory.get(pc + 2).unwrap();
    if modes.1 == 0 {
        right = memory[usize::try_from(right).unwrap()]
    }
    let result = usize::try_from(*memory.get(pc + 3).unwrap()).unwrap();
    if modes.0 != 0 {
        panic!("get_3_parameters invalid mode: {:?}", modes);
    }
    return (result, (left, right));
}
