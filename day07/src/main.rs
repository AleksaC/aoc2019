fn compute_signal(intcode: &Vec<i32>, phase_params: Vec<i32>) -> i32 {
    let mut codes = vec![intcode.clone(); 5];
    let mut ips = vec![0; 5];

    let mut out = 0;
    let mut first = true;

    let n = intcode.len();

    while ips[0] <= n && ips[0] <= n && ips[1] <= n && ips[2] <= n && ips[3] <= n && ips[4] <= n {
        'outer: for i in 0..5 {
            let intcode = &mut codes[i];
            let mut ip = ips[i];
            let mut inner_first = first;

            while ip <= n {
                let instruction = intcode[ip];
                let opcode = instruction % 100;

                if opcode < 3 {
                    let k = intcode[ip + 1] as usize;
                    let l = intcode[ip + 2] as usize;
                    let j = intcode[ip + 3] as usize;

                    let p = if (instruction % 1000) / 100 == 0 { intcode[k] } else { k as i32 };
                    let q = if (instruction % 10_000) / 1000 == 0 { intcode[l] } else { l as i32 };

                    intcode[j] = if opcode == 1 { p + q } else { p * q };

                    ip += 4;
                } else if opcode < 5 {
                    let j = intcode[ip + 1] as usize;

                    if opcode == 3 {
                        if inner_first {
                            intcode[j] = phase_params[i];
                            inner_first = false;
                        } else {
                            intcode[j] = out;
                        }
                    } else {
                        out = if (instruction % 1000) / 100 == 0 { intcode[j] } else { j as i32 };
                        ips[i] = ip + 2;
                        continue 'outer;
                    }

                    ip += 2;
                } else if opcode < 7 {
                    let mut k = intcode[ip + 1] as usize;
                    let mut l = intcode[ip + 2] as usize;

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
                    let mut k = intcode[ip + 1] as usize;
                    let mut l = intcode[ip + 2] as usize;
                    let j = intcode[ip + 3] as usize;

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
                    ips[i] = n + 1;
                    break;
                }
                ips[i] = ip;
            }
        }
        first = false;
    }

    return out;
}

fn compute_max(initial_memory: &Vec<i32>, l: i32, u: i32) -> i32 {
    let mut max = 0;

    for i in l..u {
        for j in l..u {
            if j == i { continue; }
            for k in l..u {
                if k == i || k == j { continue; }
                for l in l..u {
                    if l == i || l == j || l == k { continue; }
                    for m in l..u {
                        if m == i || m == j || m == k || m == l { continue; }
                        let tot = compute_signal(initial_memory, vec![i, j, k, l, m]);
                        if tot > max {
                            max = tot;
                        }
                    }
                }
            }
        }
    }

    return max;
}

fn main() {
    let initial_memory: Vec<i32> = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("Part one: {}", compute_max(&initial_memory, 0, 5));
    println!("Part two: {}", compute_max(&initial_memory, 5, 10));
}
