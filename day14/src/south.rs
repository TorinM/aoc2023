fn move_rock_south(platform: &mut Vec<Vec<char>>, row: usize, col: usize) {
    // Algorithm:
    // 1. Check if the rock can be moved down 
    // 2. If it can, move it down
    // 3. If it can't, do nothing
    // 4. Repeat until rock is stopped

    let mut row = row;
    let col = col;

    while row < platform.len() - 1 {
        if platform[row + 1][col] == '.' {
            platform[row + 1][col] = 'O';
            platform[row][col] = '.';
            row += 1;
        } else {
            break;
        }
    }
}

pub fn tilt_south(platform: &mut Vec<Vec<char>>) {
    // Algorithm:
    // 1. Iterate over each row
    // 2. Iterate over each item in row, if item is a O, move it down until it hits a wall or another
    //    O or the bottom of the platform
    // 3. Repeat until all items are moved up
    
    let platform_clone = platform.clone();
    let mut row_num: usize = 0;
    
    for row in platform_clone {
        let mut col_num: usize = 0;
        for i in row {
            if i == 'O' {
               move_rock_south(platform, row_num, col_num);
            }
            col_num += 1;
        }
        row_num += 1;
    }
}

