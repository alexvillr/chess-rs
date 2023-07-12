use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Pieces {
    WhiteKing = -100,
    WhiteQueen = -9,
    WhiteRook = -5,
    WhiteKnight = -4,
    WhiteBishop = -3,
    WhitePawn = -1,
    Empty = 0,
    BlackPawn = 1,
    BlackBishop = 3,
    BlackKnight = 4,
    BlackRook = 5,
    BlackQueen = 9,
    BlackKing = 100,
}

impl Pieces {
    fn get_piece(piece: &Pieces) -> &str {
        let pieces_map: HashMap<Pieces, &str> = [
            (Pieces::WhiteKing, "󰡗 "),
            (Pieces::WhiteQueen, "󰡚 "),
            (Pieces::WhiteRook, "󰡛 "),
            (Pieces::WhiteKnight, "󰡘 "),
            (Pieces::WhiteBishop, "󰡜 "),
            (Pieces::WhitePawn, "󰡙 "),
            (Pieces::Empty, "  "),
            (Pieces::BlackPawn, "󰡙 "),
            (Pieces::BlackBishop, "󰡜 "),
            (Pieces::BlackKnight, "󰡘 "),
            (Pieces::BlackRook, "󰡛 "),
            (Pieces::BlackQueen, "󰡚 "),
            (Pieces::BlackKing, "󰡗 "),
        ]
        .iter()
        .cloned()
        .collect();
        let piece_str = pieces_map.get(piece).copied();
        match piece_str {
            Some(v) => return v,
            None => return "Error",
        }
    }
}

fn main() {


    const START_STATE: [[Pieces; 8]; 8] = [
        [Pieces::BlackRook, Pieces::BlackKnight, Pieces::BlackBishop, Pieces::BlackQueen, Pieces::BlackKing, Pieces::BlackBishop, Pieces::BlackKnight, Pieces::BlackRook],
        [Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn, Pieces::BlackPawn],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty, Pieces::Empty],
        [Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn, Pieces::WhitePawn],
        [Pieces::WhiteRook, Pieces::WhiteKnight, Pieces::WhiteBishop, Pieces::WhiteKing, Pieces::WhiteQueen, Pieces::WhiteBishop, Pieces::WhiteKnight, Pieces::WhiteRook],
    ];

    print_board(&START_STATE);
}

fn print_board(state: &[[Pieces; 8]; 8]) {
    println!("╭────┬────┬────┬────┬────┬────┬────┬────╮");
    for i in state.into_iter().enumerate() {
        let (rownum, row) = i;
        if rownum != 0 {
            println!("├────┼────┼────┼────┼────┼────┼────┼────┤");
        }
        print!("│");
        for column in row.into_iter().enumerate() {
            let (_, cell) = column;
            let piece = Pieces::get_piece(&cell);
            print!(" {} ", piece);
            print!("│");
        }
        println!();
    }
    println!("╰────┴────┴────┴────┴────┴────┴────┴────╯");
}

