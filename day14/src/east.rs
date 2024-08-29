fn move_rock_east(platform: &mut Vec<Vec<char>>, row: usize, col: usize) {
    // Algorithm:
    // 1. Check if the rock can be moved right 
    // 2. If it can, move it right
    // 3. If it can't, do nothing
    // 4. Repeat until rock is stopped
    
    let row = row;
    let mut col = col;

    while col < platform[row].len() - 1 {
        if platform[row][col + 1] == '.' {
            platform[row][col + 1] = 'O';
            platform[row][col] = '.';
            col += 1;
        } else {
            break;
        }
    }
}

pub fn tilt_east(platform: &mut Vec<Vec<char>>) {
    // Algorithm:
    // 1. Iterate over each row
    // 2. Iterate over each item in row, if item is a O, move it right until it hits a wall or another
    //    O or the side of the platform
    
    let platform_clone = platform.clone();
    let mut row_num: usize = 0;
    
    for row in platform_clone {
        let mut col_num: usize = 0;
        for i in row {
            if i == 'O' {
               move_rock_east(platform, row_num, col_num);
            }
            col_num += 1;
        }
        row_num += 1;
    }
}
