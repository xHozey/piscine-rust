pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
let players = ['X', 'O'];
    for player in players {
        if horizontal(player, table) || diagonals(player, table) || vertical(player, table) {
            return format!("player {} won", player);
        }
    }
    return "tie".to_string();
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
     (table[0][0] == player && table[1][1] == player && table[2][2] == player)
        || (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table.iter().any(|row| row.iter().all(|&cell| cell == player))
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (0..3).any(|col| (0..3).all(|row| table[row][col] == player))
}