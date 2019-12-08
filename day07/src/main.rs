fn compute(intcode: &mut Vec<i32>, input_1: i32, input_2: i32, mut first: bool) -> i32 {
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
                if first {
                    intcode[j] = input_1;
                    first = false;
                } else {
                    intcode[j] = input_2;
                }
            } else {
                return if (instruction % 1000) / 100 == 0 { intcode[j] } else { j as i32 };
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
    return -1;
}

fn compute_signal(intcode: &Vec<i32>, phase_params: Vec<i32>) -> i32 {
    let mut prev = 0;

    for i in 0..5 {
        prev = compute(&mut intcode.clone(), phase_params[i], prev, true);
    }

    return prev;
}

fn compute_with_loop(intcode: &Vec<i32>, phase_params: Vec<i32>) -> i32 {
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

fn main() {
    let initial_memory: Vec<i32> = include_str!("../input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut max = 0;

    for i in 0..5 {
        for j in 0..5 {
            if j == i { continue; }
            for k in 0..5 {
                if k == i || k == j { continue; }
                for l in 0..5 {
                    if l == i || l == j || l == k { continue; }
                    for m in 0..5 {
                        if m == i || m == j || m == k || m == l { continue; }
                        let tot = compute_signal(&initial_memory, vec![i, j, k, l, m]);
                        if tot > max {
                            max = tot;
                        }
                    }
                }
            }
        }
    }

    println!("Part one: {}", max);

    max = 0;

    for i in 5..10 {
        for j in 5..10 {
            if j == i { continue; }
            for k in 5..10 {
                if k == i || k == j { continue; }
                for l in 5..10 {
                    if l == i || l == j || l == k { continue; }
                    for m in 5..10 {
                        if m == i || m == j || m == k || m == l { continue; }
                        let tot = compute_with_loop(&initial_memory, vec![i, j, k, l, m]);
                        if tot > max {
                            max = tot;
                        }
                    }
                }
            }
        }
    }

    println!("Part two: {}", max);
}
