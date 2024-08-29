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
pub fn tilt_north(platform: &mut Vec<Vec<char>>) {
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
