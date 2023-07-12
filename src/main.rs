use std::collections::HashMap;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Enum for storing the different piece types
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pieces {
    BlueKing = -100,
    BlueQueen = -9,
    BlueRook = -5,
    BlueKnight = -4,
    BlueBishop = -3,
    BluePawn = -1,
    Empty = 0,
    GreenPawn = 1,
    GreenBishop = 3,
    GreenKnight = 4,
    GreenRook = 5,
    GreenQueen = 9,
    GreenKing = 100,
}
// For turning our enum of the piece into a string slice for printing
impl Pieces {
    fn get_piece(piece: &Pieces) -> &str {
        let pieces_map: HashMap<Pieces, &str> = [
            (Pieces::BlueKing, "󰡗 "),
            (Pieces::BlueQueen, "󰡚 "),
            (Pieces::BlueRook, "󰡛 "),
            (Pieces::BlueKnight, "󰡘 "),
            (Pieces::BlueBishop, "󰡜 "),
            (Pieces::BluePawn, "󰡙 "),
            (Pieces::Empty, "  "),
            (Pieces::GreenPawn, "󰡙 "),
            (Pieces::GreenBishop, "󰡜 "),
            (Pieces::GreenKnight, "󰡘 "),
            (Pieces::GreenRook, "󰡛 "),
            (Pieces::GreenQueen, "󰡚 "),
            (Pieces::GreenKing, "󰡗 "),
        ]
        .into_iter()
        .collect();
        let piece_str = pieces_map.get(piece).copied();
        match piece_str {
            Some(v) => return v,
            None => return "Error",
        }
    }
}

/* main()
 * ------
 * The main method. All functions are called inside of this method
*/
fn main() {
    // A standard set up for the first version of a board. Note red is black
    // and blue is white
    const START_STATE: [[Pieces; 8]; 8] = [
        [ // First Row
            Pieces::GreenRook,
            Pieces::GreenKnight,
            Pieces::GreenBishop,
            Pieces::GreenQueen,
            Pieces::GreenKing,
            Pieces::GreenBishop,
            Pieces::GreenKnight,
            Pieces::GreenRook,
        ],
        [ // Second Row
            Pieces::GreenPawn,
            Pieces::GreenPawn,
            Pieces::GreenPawn,
            Pieces::GreenPawn,
            Pieces::GreenPawn,
            Pieces::GreenPawn,
            Pieces::GreenPawn,
            Pieces::GreenPawn,
        ],
        [ // Third Row
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [ // Fourth Row
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [ // Fifth Row
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [ // Sixth Row
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
            Pieces::Empty,
        ],
        [ // Seventh Row
            Pieces::BluePawn,
            Pieces::BluePawn,
            Pieces::BluePawn,
            Pieces::BluePawn,
            Pieces::BluePawn,
            Pieces::BluePawn,
            Pieces::BluePawn,
            Pieces::BluePawn,
        ],
        [ // Eigth Row
            Pieces::BlueRook,
            Pieces::BlueKnight,
            Pieces::BlueBishop,
            Pieces::BlueKing,
            Pieces::BlueQueen,
            Pieces::BlueBishop,
            Pieces::BlueKnight,
            Pieces::BlueRook,
        ],
    ];
    print_board(&START_STATE);
}

/* print_board()
 * -------------
 * prints the board given a current state. Doesn't do much else but does handle
 * the changing of colours for the output so that the pieces are proper colours
 *
 * state: The current state of the board. A 2d array of Pieces to be iterated
 *          over
*/
fn print_board(state: &[[Pieces; 8]; 8]) {
    // initialises a stdout stream as well as a colorspec.
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut color_spec = ColorSpec::new();

    println!("  A    B    C    D    E    F    G    H");
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
            if cell > &Pieces::Empty { // then green team
                color_spec.set_fg(Some(Color::Green));
                stdout.set_color(&color_spec).expect("Failed to set color");
                write!(&mut stdout, " {} ", piece).expect("Failed to write");
                stdout.reset().expect("Failed to reset color");
            } else if cell < &Pieces::Empty { // then blue team
                color_spec.set_fg(Some(Color::Blue));
                stdout.set_color(&color_spec).expect("Failed to set color");
                write!(&mut stdout, " {} ", piece).expect("Failed to write");
                stdout.reset().expect("Failed to reset color");
            } else {
                print!(" {} ", piece);
            }
            print!("│");
        }
        println!(" {}", rownum + 1);
    }
    println!("╰────┴────┴────┴────┴────┴────┴────┴────╯");
}
