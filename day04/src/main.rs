fn main() {
    let lower_bound = 147981;
    let upper_bound = 691423;

    let mut count1 = 0;
    let mut count2 = 0;

    'outer: for mut i in lower_bound..upper_bound {
        let mut d = i % 10;
        i /= 10;
        let mut has_adjacent = false;
        let mut has_just_two = false;
        'inner: while i != 0 {
            if i % 10 > d {
                continue 'outer;
            } else if i % 10 == d {
                has_adjacent = true;
                let mut t = i / 10;
                if t == 0 || t % 10 != d {
                    has_just_two = true;
                } else {
                    while t % 10 == d && t != 0{
                        t /= 10;
                    }
                    if t == 0 {
                        if has_adjacent {
                            count1 += 1;
                        }
                        if has_just_two {
                            count2 += 1;
                        }
                        continue 'outer;
                    }
                    i = t;
                    continue 'inner;
                }
            }
            d = i % 10;
            i /= 10;
        }
        if has_adjacent {
            count1 += 1;
            if has_just_two {
                count2 += 1;
            }
        }
    }

    println!("Part one: {}", count1);
    println!("Part two: {}", count2);
}
