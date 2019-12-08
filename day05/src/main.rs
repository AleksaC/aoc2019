fn compute(intcode: &mut Vec<i32>, input: i32) {
    let n = intcode.len();
    let mut ip = 0;

    while ip <= n {
        let instruction = intcode[ip];

        let opcode = instruction % 100;

        if opcode  < 3 {
            let k = intcode[ip +1] as usize;
            let l = intcode[ip +2] as usize;
            let j = intcode[ip +3] as usize;

            let p = if (instruction % 1000) / 100 == 0 { intcode[k] } else { k as i32 };
            let q = if (instruction % 10_000) / 1000 == 0 { intcode[l] } else { l as i32 };

            intcode[j] = if opcode == 1 { p + q } else { p * q };

            ip += 4;
        } else if opcode < 5 {
            let j = intcode[ip +1] as usize;

            if opcode == 3 {
                intcode[j] = input;
            } else {
                let t = if (instruction % 1000) / 100 == 0 { intcode[j] } else { j as i32 };
                if t != 0 {
                    println!("{}", t);
                }
            }

            ip += 2;
        } else if opcode < 7 {
            let mut k = intcode[ip +1] as usize;
            let mut l = intcode[ip +2] as usize;

            if (instruction % 1000) / 100 == 0 {
                k = intcode[k] as usize;
            }
            if (instruction % 10_000) / 1000 == 0 {
                l = intcode[l] as usize;
            }

            let z = k == 0;

            if opcode == 5 && !z || opcode == 6 && z {
                ip = l;
            } else {
                ip += 3;
            }
        } else if opcode < 9 {
            let mut k = intcode[ip +1] as usize;
            let mut l = intcode[ip +2] as usize;
            let j = intcode[ip +3] as usize;

            if (instruction % 1000) / 100 == 0 {
                k = intcode[k] as usize;
            }
            if (instruction % 10_000) / 1000 == 0 {
                l = intcode[l] as usize;
            }

            if opcode == 7 && k < l || opcode == 8 && k == l {
                intcode[j] = 1;
            } else {
                intcode[j] = 0;
            }

            ip += 4;
        } else {
            break;
        }
    }
}

fn main() {
    let mut initial_memory: Vec<i32> = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    print!("Part one: ");
    compute(&mut initial_memory.clone(), 1);
    print!("Part two: ");
    compute(&mut initial_memory, 5);
}
