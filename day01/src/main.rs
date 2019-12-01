fn fuel_required(mass: i32) -> i32 {
    return mass / 3 - 2;
}

fn main() {
    let fuel_per_module: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|mass| fuel_required(mass.parse::<i32>().unwrap()))
        .collect();

    println!("Total fuel - first part: {}", fuel_per_module.iter().sum::<i32>());

    let mut total_fuel: i32 = 0;
    for mut fuel in fuel_per_module {
        let mut extra_fuel: i32 = fuel_required(fuel);
        while extra_fuel > 0 {
            fuel += extra_fuel;
            extra_fuel = fuel_required(extra_fuel);
        }
        total_fuel += fuel
    }

    println!("Total fuel - second part: {}", total_fuel);
}
