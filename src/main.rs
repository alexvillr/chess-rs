fn main() {
    let start_state: [[&str; 8]; 8] = [
        ["br", "bk", "bb", "bK", "bq", "bb", "bk", "br"],
        ["bp", "bp", "bp", "bp", "bp", "bp", "bp", "bp"],
        ["na", "na", "na", "na", "na", "na", "na", "na"],
        ["na", "na", "na", "na", "na", "na", "na", "na"],
        ["na", "na", "na", "na", "na", "na", "na", "na"],
        ["na", "na", "na", "na", "na", "na", "na", "na"],
        ["wp", "wp", "wp", "wp", "wp", "wp", "wp", "wp"],
        ["wr", "wk", "wb", "wK", "wq", "wb", "wk", "wr"],
    ];

    print_board(&start_state);
}

fn print_board(state: &[[&str; 8]; 8]) {
    for i in state.into_iter().enumerate() {
        let (_, row) = i;
        println!("|----+----+----+----+----+----+----+----|");
        print!("|");
        for column in row.into_iter().enumerate() {
            let (_, cell) = column;
            print!(" {} |", cell);
        }
        println!();
    }
    println!("|----+----+----+----+----+----+----+----|");
}
