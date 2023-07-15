use std::collections::HashMap;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Alliance {
    Blue,
    Green,
    Empty,
}

// Enum for storing the different piece types
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Pieces {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
    Empty,
}
// For turning our enum of the piece into a string slice for printing
impl Pieces {
    fn get_piece(piece: &Pieces) -> &str {
        let pieces_map: HashMap<Pieces, &str> = [
            (Pieces::King, "󰡗 "),
            (Pieces::Queen, "󰡚 "),
            (Pieces::Rook, "󰡛 "),
            (Pieces::Knight, "󰡘 "),
            (Pieces::Bishop, "󰡜 "),
            (Pieces::Pawn, "󰡙 "),
            (Pieces::Empty, "  "),
        ]
        .into_iter()
        .collect();
        let piece_str = pieces_map.get(piece).copied();
        match piece_str {
            Some(v) => {
                return v;
            }
            None => {
                return "Error";
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Player {
    colour: Alliance,
    score: i128,
}

#[derive(Clone, Copy)]
struct Piece {
    kind: Pieces,
    player: Player,
}

/* main()
 * ------
 * The main method. All functions are called inside of this method
 */
fn main() {
    // A standard set up for the first version of a board. Note green is black
    // and blue is white
    const GREEN_TEAM: Player = Player {
        colour: Alliance::Green,
        score: 0,
    };
    const BLUE_TEAM: Player = Player {
        colour: Alliance::Blue,
        score: 0,
    };
    const NO_TEAM: Player = Player {
        colour: Alliance::Empty,
        score: -1,
    };
    const TEAMS: [Player; 3] = [GREEN_TEAM, BLUE_TEAM, NO_TEAM];
    let mut state: [[Piece; 8]; 8] = reset_board(TEAMS);
    print_board(&state);
    let mut game_over: bool = false;
    let mut blues_turn: bool = true;
    let mut player: Player = TEAMS[0].clone();
    while !game_over {
        game_over = move_available(&state, player);

        // state = make_move(get_player_move());

        blues_turn = !blues_turn;
        if blues_turn {
            player = TEAMS[Alliance::Blue as usize].clone();
        } else {
            player = TEAMS[Alliance::Green as usize].clone();
        }
    }
}

fn reset_board(teams: [Player; 3]) -> [[Piece; 8]; 8] {
    let empty: Piece = Piece {
        kind: Pieces::Empty,
        player: teams[Alliance::Empty as usize].clone(),
    };
    let mut board: [[Piece; 8]; 8] = [[empty.clone(); 8]; 8];
    let mut king_row: usize;
    let mut pawn_row: usize;
    for team in teams.into_iter() {
        if team.colour == Alliance::Blue {
            king_row = 7;
            pawn_row = 6;
            board[king_row][4] = Piece {
                kind: Pieces::King,
                player: team.clone(),
            };
            board[king_row][3] = Piece {
                kind: Pieces::Queen,
                player: team.clone(),
            };
        } else if team.colour == Alliance::Green {
            king_row = 0;
            pawn_row = 1;
            board[king_row][3] = Piece {
                kind: Pieces::King,
                player: team.clone(),
            };
            board[king_row][4] = Piece {
                kind: Pieces::Queen,
                player: team.clone(),
            };
        } else {
            continue;
        }
        for column in 0..8 {
            board[pawn_row][column] = Piece {
                kind: Pieces::Pawn,
                player: team.clone(),
            };
        }
        board[king_row][0] = Piece {
            kind: Pieces::Rook,
            player: team.clone(),
        };
        board[king_row][7] = Piece {
            kind: Pieces::Rook,
            player: team.clone(),
        };
        board[king_row][1] = Piece {
            kind: Pieces::Knight,
            player: team.clone(),
        };
        board[king_row][6] = Piece {
            kind: Pieces::Knight,
            player: team.clone(),
        };
        board[king_row][2] = Piece {
            kind: Pieces::Bishop,
            player: team.clone(),
        };
        board[king_row][5] = Piece {
            kind: Pieces::Bishop,
            player: team.clone(),
        };
    }
    return board;
}

/* print_board()
 * -------------
 * prints the board given a current state. Doesn't do much else but does handle
 * the changing of colours for the output so that the pieces are proper colours
 *
 * state: The current state of the board. A 2d array of Pieces to be iterated
 *          over
 */
fn print_board(state: &[[Piece; 8]; 8]) {
    // initialises a stdout stream as well as a colorspec.
    let mut stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
    let mut color_spec: ColorSpec = ColorSpec::new();

    println!("  A    B    C    D    E    F    G    H");
    println!("╭────┬────┬────┬────┬────┬────┬────┬────╮");
    for (rownum, row) in state.into_iter().enumerate() {
        if rownum != 0 {
            println!("├────┼────┼────┼────┼────┼────┼────┼────┤");
        }
        print!("│");
        for cell in row.into_iter() {
            let piece = Pieces::get_piece(&cell.kind);
            if cell.player.colour == Alliance::Green {
                // then green team
                color_spec.set_fg(Some(Color::Green));
                stdout.set_color(&color_spec).expect("Failed to set color");
                write!(&mut stdout, " {} ", piece).expect("Failed to write");
            } else if cell.player.colour == Alliance::Blue {
                // then blue team
                color_spec.set_fg(Some(Color::Blue));
                stdout.set_color(&color_spec).expect("Failed to set color");
                write!(&mut stdout, " {} ", piece).expect("Failed to write");
            } else {
                print!(" {} ", piece);
            }
            stdout.reset().expect("Failed to reset color");
            print!("│");
        }
        println!(" {}", rownum + 1);
    }
    println!("╰────┴────┴────┴────┴────┴────┴────┴────╯");
}

fn move_available(state: &[[Piece; 8]; 8], player: Player) -> bool {
    return true;
}

fn make_move(location: &str, destination: &str, state: &[[Piece; 8]; 8]) -> [[Piece; 8]; 8] {
    return state.clone();
}
