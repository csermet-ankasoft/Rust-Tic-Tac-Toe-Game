use std::io::{stdin, stdout, Write};
pub fn get_input() -> String {
    let mut inputtext= String::new();
    let _=stdout().flush();
    stdin().read_line(&mut inputtext).expect("Did not enter a correct string");
    if let Some('\n')=inputtext.chars().next_back() {
        inputtext.pop();
    }
    if let Some('\r')=inputtext.chars().next_back() {
        inputtext.pop();
    }
    return inputtext;
}

pub fn cube_inside_character(shapes: &Vec<&str>) -> Vec<String> {
    let mut line = vec![String::new(),String::new(),String::new()];
    let mut count = 0;
    for i in 0..3 {
        for _ in 0..3 {
            line[i] = line[i].clone() + "|" + shapes[count].clone() + "| ";
            count = count+1;
        }
    }
    return line;
}

pub fn game_start(){
    let introduce_game_shapes = vec!["1","2","3","4","5","6","7","8","9"];
    game_console_write(&introduce_game_shapes);
    println!("\nWelcome to the X-O game. This game is played with 2 people.
According to the number order you see above, you need to write in which position you will put X or O.
First player controls X shapes and second player controls O shapes. Have a Nice Game\n
Enter any character to start the game");
    stdin().read_line(&mut "".to_string()).expect("Did not enter a correct string");
}

pub fn game_finish_check(game_board_shapes: &Vec<&str>)->bool{
    for i in 0..3 {
        if game_board_shapes[i*3] == game_board_shapes[(i*3)+1] &&
           game_board_shapes[i*3] == game_board_shapes[(i*3)+2] &&
           game_board_shapes[i*3] != " " {
            return true
        }
    }
    for i in 0..3 {
        if game_board_shapes[i] == game_board_shapes[3+i] &&
           game_board_shapes[i] == game_board_shapes[6+i] &&
           game_board_shapes[i] != " "
        {
            return true
        }
    }
    if game_board_shapes[0] == game_board_shapes[4] &&
       game_board_shapes[0] == game_board_shapes[8] &&
       game_board_shapes[0] != " " {
        return true
    }
    else if game_board_shapes[2] == game_board_shapes[4] &&
        game_board_shapes[2] == game_board_shapes[6] &&
        game_board_shapes[4] != " " {
        return true
    }
    return false;
}

pub fn game_console_write(game_board_shapes: &Vec<&str>){
    clearscreen::clear().unwrap();
    let console_text = cube_inside_character(game_board_shapes);
    for i in 0..3 {
        println!("{}",console_text[i]);
    }
}

pub enum game_move_result{
    Ok,
    Wrong_input,
    Mismatched_area,
}

pub fn game_move<'a>(game_board_shapes: &mut Vec<&'a str>, player_shape: &'a str) -> game_move_result{
    let input_text = get_input();
    //Number Or Not Check
    let number = match input_text.parse::<i32>() {
        Ok(n) => (n),
        Err(..) => (-1) ,
    };

    if number == -1 {
        return game_move_result::Wrong_input;
    }
    if number<10 && number>0 && game_board_shapes[(number-1) as usize] == " "
    {
        game_board_shapes[(number-1) as usize] = player_shape;
        return game_move_result::Ok;
    }
    return game_move_result::Mismatched_area;
}

pub fn computer_turn(game_board_shapes: &mut Vec<&str>){

}