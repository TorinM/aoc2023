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

fn main() {
    let input = get_input("input.txt");
    
    println!("{:?}", input);
}
