fn compute(intcode: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
    intcode[1] = noun;
    intcode[2] = verb;

    let n = intcode.len() - intcode.len() % 4;

    for i in {0..=n}.step_by(4) {
        let j = intcode[i+3] as usize;
        let k = intcode[i+1] as usize;
        let l = intcode[i+2] as usize;
        if intcode[i] == 1 {
            intcode[j] = intcode[k] + intcode[l];
        } else if intcode[i] == 2 {
            intcode[j] = intcode[k] * intcode[l];
        } else {
            break;
        }
    }

    return intcode[0];
}

fn main() {
    let initial_memory: Vec<i32> = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|int| int.parse().unwrap())
        .collect();

    let mut intcode: Vec<i32> = initial_memory.clone();
    println!("Part one: {}", compute(&mut intcode, 12, 2));

    'outer: for i in 0..=99 {
        'inner: for j in 0..=99 {
            intcode = initial_memory.clone();
            if compute(&mut intcode, i, j) == 19690720{
                println!("Part 2: {}", 100 * i + j);
                break 'outer;
            }
        }
    }
}
