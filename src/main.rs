use crate::tic_tac_toe::game_move_result;

mod tic_tac_toe;

fn main() {
    tic_tac_toe::game_start();
    clearscreen::clear().unwrap();
    game_loop_playerVScomputer();
}

fn game_loop_2player(){
    let mut firstplayerturn = true;
    let mut player_shape;
    let mut game_board_shapes = vec![" "," "," "," "," "," "," "," "," "];
    loop {
        let mut error = String::new();
        tic_tac_toe::game_console_write(&game_board_shapes);

        if firstplayerturn {
            println!("First Player Please Make Your Move (X)      {}",error);
            player_shape = "X";

        }
        else {
            println!("Second Player Please Make Your Move (O)      {}",error);
            player_shape = "O";
        }
        match tic_tac_toe::game_move(&mut game_board_shapes,player_shape) {
            game_move_result::Ok=> {
                if firstplayerturn { firstplayerturn = false; } else { firstplayerturn = true; }
            },
            game_move_result::Wrong_input => {error = String::from("Please Write Number"); continue;},
            game_move_result::Mismatched_area => {continue;}

        }


        if tic_tac_toe::game_finish_check(&game_board_shapes) {
            tic_tac_toe::game_console_write(&game_board_shapes);
            println!("Finish Game => Player {} WIN!!",player_shape);
            break;
        }
    }

}


fn game_loop_playerVScomputer(){
    let mut player_shape  = "X";
    let mut game_board_shapes = vec![" "," "," "," "," "," "," "," "," "];
    loop {
        let mut error = String::new();
        tic_tac_toe::game_console_write(&game_board_shapes);
        println!("Make Your Move      {}",error);
        match tic_tac_toe::game_move(&mut game_board_shapes,player_shape) {
            game_move_result::Ok=> {},
            game_move_result::Wrong_input => {error = String::from("Please Write Number"); continue;},
            game_move_result::Mismatched_area => {continue;}
        }
        if tic_tac_toe::game_finish_check(&game_board_shapes) {
            tic_tac_toe::game_console_write(&game_board_shapes);
            println!("Finish Game Player {} WIN!!",player_shape);
            break;
        }


        tic_tac_toe::computer_turn(&mut game_board_shapes);
        if tic_tac_toe::game_finish_check(&game_board_shapes) {
            tic_tac_toe::game_console_write(&game_board_shapes);
            println!("Finish Game Player {} WIN!!",player_shape);
            break;
        }
    }

}
