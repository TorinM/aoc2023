use std::fs::read_to_string;

mod north;
mod west;
mod south;
mod east;

fn get_input(file_name:&str) -> Vec<Vec<char>> {
    let mut file_lines = Vec::new();

    let input = read_to_string(file_name).unwrap();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        file_lines.push(row);
    }

    let mut result = Vec::new();
    for line in file_lines {
        let mut line_vec = Vec::new();

        for c in line {
            line_vec.push(c);
        }
        result.push(line_vec.clone());
    }
    
    result
}

fn cycle_platform(platform: &mut Vec<Vec<char>>) {
    // 1 Cycle = 1 tilt_north + 1 tilt_west + 1 tilt_south + 1 tilt_east
    north::tilt_north(platform);
    west::tilt_west(platform);
    south::tilt_south(platform);
    east::tilt_east(platform);
}

fn count_load(platform: &Vec<Vec<char>>) -> usize {
    let mut total_load = 0;

    let mut row_num: usize = platform.len();

    for row in platform.clone() {
        let mut row_load = 0;
        for i in row {
            if i == 'O' {
                row_load += row_num;
            }
        }
        total_load += row_load;
        row_num -= 1;
    }

    total_load
}

fn print_platform(platform: &Vec<Vec<char>>) {
    for row in platform {
        println!("{:?}", row);
    }
    println!();
}

fn main() {
    let mut platform = get_input("input_small.txt");
    
    for i in 0..1000000000 {
        cycle_platform(&mut platform);
        print!("Finished cycle: {}\r", i);
    }

    println!("Total load: {}", count_load(&platform));
}
