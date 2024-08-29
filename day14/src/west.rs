fn move_rock_west(platform: &mut Vec<Vec<char>>, row: usize, col: usize) {
    // Algorithm:
    // 1. Check if the rock can be moved left 
    // 2. If it can, move it left
    // 3. If it can't, do nothing
    // 4. Repeat until rock is stopped
    
    let row = row;
    let mut col = col;

    while col > 0 {
        if platform[row][col - 1] == '.' {
            platform[row][col - 1] = 'O';
            platform[row][col] = '.';
            col -= 1;
        } else {
            break;
        }
    }
}

pub fn tilt_west(platform: &mut Vec<Vec<char>>) {
    // Algorithm:
    // 1. Iterate over each row
    // 2. Iterate over each item in row, if item is a O, move it left until it hits a wall or another
    //    O or the side of the platform
    
    let platform_clone = platform.clone();
    let mut row_num: usize = 0;
    
    for row in platform_clone {
        let mut col_num: usize = 0;
        for i in row {
            if i == 'O' {
               move_rock_west(platform, row_num, col_num);
            }
            col_num += 1;
        }
        row_num += 1;
    }
}
