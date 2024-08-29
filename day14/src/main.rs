use std::fs::read_to_string;

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

fn move_rock_north(platform: &mut Vec<Vec<char>>, row: usize, col: usize) {
    // Algorithm:
    // 1. Check if the rock can be moved up
    // 2. If it can, move it up
    // 3. If it can't, do nothing
    // 4. Repeat until rock is stopped 

    let mut row = row;
    let col = col;

    while row > 0 {
        if platform[row - 1][col] == '.' {
            platform[row - 1][col] = 'O';
            platform[row][col] = '.';
            row -= 1;
        } else {
            break;
        }
    }
}

// Tilt the platform to the north so all chars fall to top in accordance with problem statement
fn tilt_north(platform: &mut Vec<Vec<char>>) {
    // Algorithm:
    // 1. Iterate over each row
    // 2. Iterate over each item in row, if item is a O, move it up until it hits a wall or another
    //    O or the top of the platform
    // 3. Repeat until all items are moved up
    
    let platform_clone = platform.clone();
    let mut row_num: usize = 0;
    
    for row in platform_clone {
        let mut col_num: usize = 0;
        for i in row {
            if i == 'O' {
               move_rock_north(platform, row_num, col_num);
            }
            col_num += 1;
        }

        row_num += 1;
    }
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
    let mut platform = get_input("input.txt");
    //print_platform(&platform);

    tilt_north(&mut platform);
    
    print_platform(&platform);

    println!("Total load: {}", count_load(&platform));
}
