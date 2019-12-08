fn main() {
    let image: Vec<u8> = include_str!("../input.txt")
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    let w = 25;
    let h = 6;

    let layer_size = w * h;
    let num_layers = image.len() / layer_size;

    let mut min_zeros = layer_size + 1;
    let mut min_zeros_index = 1;

    for i in 1..=num_layers {
        let mut num_zeros = 0;
        for j in 0..layer_size {
            if image[(i - 1) * layer_size + j] == 0 {
                num_zeros += 1;
            }
        }
        if num_zeros < min_zeros {
            min_zeros = num_zeros;
            min_zeros_index = i;
        }
    }

    let mut num_ones = 0;
    let mut num_twos = 0;

    for i in 0..layer_size {
        let pixel = image[(min_zeros_index - 1) * layer_size + i];
        if pixel == 1 {
            num_ones += 1;
        } else if pixel == 2 {
            num_twos += 1
        }
    }

    println!("Part one: {}", num_ones * num_twos);

    print!("Part two: ");
    'outer: for i in 0..layer_size {
        if i % w == 0 {
            println!();
        }
        for j in 1..=num_layers {
            let pixel = image[(j - 1) * layer_size + i];
            if pixel != 2 {
                print!("{}", if pixel == 1 { "█" } else { " " });
                continue 'outer;
            }
        }
    }
}
