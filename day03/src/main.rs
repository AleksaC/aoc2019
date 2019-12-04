fn make_move(mut x: i32, mut y: i32, mv: &str) -> (i32, i32) {
    let direction = &mv[0..1];
    let steps = &mv[1..];

    match direction {
        "U" => y = y + steps.parse::<i32>().unwrap(),
        "R" => x = x + steps.parse::<i32>().unwrap(),
        "D" => y = y - steps.parse::<i32>().unwrap(),
        "L" => x = x - steps.parse::<i32>().unwrap(),
        _ => println!("Incorrect input: {}", direction),
    }

    return (x, y);
}

fn get_coords(moves: &Vec<&str>) -> Vec<((i32, i32), (i32, i32))> {
    let mut coords: Vec<((i32, i32), (i32, i32))> = Vec::with_capacity(moves.len());

    let (mut x, mut y) = (0, 0);

    for mv in moves {
        let (x_new, y_new) = make_move(x, y, mv);
        coords.push(((x, y), (x_new, y_new)));
        x = x_new;
        y = y_new;
    }

    return coords;
}

fn is_vertical(line: ((i32, i32), (i32, i32))) -> bool {
    let ((x11, _), (x12, _)) = line;
    return x11 != x12;
}

fn intersect(vertical: ((i32, i32), (i32, i32)), horizontal: ((i32, i32), (i32, i32))) -> bool {
    let ((x11, y11), (x12, _)) = vertical;
    let ((x21, y21), (_, y22)) = horizontal;

    return (x11 - x21) * (x12 - x21) <= 0 && (y21 - y11) * (y22 - y11) <= 0;
}

fn intersection_distance(line_one: ((i32, i32), (i32, i32)),
                         line_two: ((i32, i32), (i32, i32))) -> (i32, (i32, i32)) {
    let l1_is_vertical = is_vertical(line_one);
    let l2_is_vertical = is_vertical(line_two);

    if l1_is_vertical && l2_is_vertical {
        return (-1, (0, 0));
    }
    if !l1_is_vertical && !l2_is_vertical {
        return (-1, (0, 0));
    }

    let (vertical, horizontal) = if l1_is_vertical { (line_one, line_two) }
                                                    else { (line_two, line_one) };

    if intersect(vertical, horizontal) {
        let ((x11, y11), _) = vertical;
        let ((x21, y21), _) = horizontal;

        return (x21.abs() + y11.abs(), ((x21 - x11).abs(), (y11 - y21).abs()));
    }

    return (-1, (0, 0));
}

fn main() {
    let wires: Vec<&str> = include_str!("../input.txt").trim().split("\n").collect();

    let wire_one: Vec<&str> = wires[0].split(",").collect();
    let wire_two: Vec<&str> = wires[1].split(",").collect();

    let wire_one_coords: Vec<((i32, i32), (i32, i32))> = get_coords(&wire_one);
    let wire_two_coords: Vec<((i32, i32), (i32, i32))> = get_coords(&wire_two);

    assert_eq!(wire_one.len(), wire_one_coords.len());
    assert_eq!(wire_two.len(), wire_two_coords.len());

    let mut min_dist = -1;
    let mut min_steps = -1;

    let mut num_steps_1 = 995;

    for i in 1..wire_one_coords.len() {
        let mut num_steps_2 = 996;
        for j in 1..wire_two_coords.len() {
            let (d, (x, y)) = intersection_distance(wire_one_coords[i], wire_two_coords[j]);
            if d != -1 {
                if min_dist != -1{
                    if d < min_dist {
                        min_dist = d;
                    }
                } else {
                    min_dist = d;
                }
                let num_steps = num_steps_1 + num_steps_2 + x + y;
                if min_steps != -1 {
                    if num_steps < min_steps {
                        min_steps = num_steps;
                    }
                } else {
                    min_steps = num_steps;
                }
            }
            let ((x11, y11), (x12, y12)) = wire_two_coords[j];
            num_steps_2 += (x11 - x12 + y11 - y12).abs();
        }
        let ((x11, y11), (x12, y12)) = wire_one_coords[i];
        num_steps_1 += (x11 - x12 + y11 - y12).abs();
    }

    println!("Part one: {}", min_dist);
    println!("Part two: {}", min_steps);
}
