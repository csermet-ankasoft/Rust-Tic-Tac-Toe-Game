mod tic_tac_toe;

fn main() {
    tic_tac_toe::game_start();
    clearscreen::clear().unwrap();
    game_loop();
}

fn game_loop(){
    let mut firstplayerturn = true;
    let mut error = String::new();
    let mut player_shape;
    let mut game_board_shapes = vec![" "," "," "," "," "," "," "," "," "];
    loop {
        tic_tac_toe::game_console_write(&game_board_shapes);

        if firstplayerturn {
            println!("First Player Please Make Your Move (X)      {}",error);
            player_shape = "X";
        }
        else {
            println!("Second Player Please Make Your Move (O)");
            player_shape = "O";
        }
        let inputtext = tic_tac_toe::get_input();
        if inputtext == "end" { break;}

        //Number Or Not Check
        let number = match inputtext.parse::<i32>() {
            Ok(n) => (n),
            Err(..) => (-1) ,
        };

        if number == -1 {
            error = String::from("Please Write Number");
            continue;
        }
        if number<10 && number>0 && game_board_shapes[(number-1) as usize] == " "
        {
            game_board_shapes[(number-1) as usize] = player_shape;
            if firstplayerturn { firstplayerturn = false; }
            else { firstplayerturn = true; }
        }


        if tic_tac_toe::game_finish_check(&game_board_shapes) {
            tic_tac_toe::game_console_write(&game_board_shapes);
            println!("Finish Game Player {} WIN!!",player_shape);
            break;
        }
    }

}
