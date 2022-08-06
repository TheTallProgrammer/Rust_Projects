use std::io; 
use rand::Rng;

pub fn game_loop(){
    let mut keep_going = true;
    loop {
        if(keep_going){
            println!("\n\n\n\nTime to play Rock, Paper, Scissors!\nWhat will you choose player? Type 'quit' to exit");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            keep_going = player_input(guess);
        } else {break;}
    }
} // End of game_loop

fn player_input(player_gesture: String) -> bool{
    let computer_gesture:i32 = generate_computer_guess();
    let player_gesture:i32 = gesture_value(player_gesture);
    if player_gesture == 0{return false;} 
    else {move_result(player_gesture, computer_gesture); return true;}
} // End of player_input()

fn generate_computer_guess() -> i32{return rand::thread_rng().gen_range(1..=3);} // End of generate_computer_guess()

fn gesture_value(gesture: String) -> i32{
    let new_gesture = gesture.to_lowercase();
    let mut value:i32 = 0;  
    if new_gesture.trim() == "rock"{value = 1;} 
    else if new_gesture.trim() == "paper"{value = 2;} 
    else if new_gesture.trim() == "scissors"{value = 3;} 
    else {value = 0;}
    return value;
} // End of gesture_value()

fn move_result(player_gesture: i32, computer_gesture: i32){
    if player_gesture == computer_gesture{println!("Tie!");} 
    else if player_gesture == 3 && computer_gesture == 1{println!("The computer chooses: rock\nComputer defeats scissors with rock!");} 
    else if player_gesture == 1 && computer_gesture == 3{println!("The computer chooses: scissors\nPlayer defeats scissors with rock!");} 
    else if player_gesture == 2 && computer_gesture == 3{println!("The computer chooses: scissors\nComputer defeats paper with scissors!");} 
    else if player_gesture == 3 && computer_gesture == 2{println!("The computer chooses: paper\nPlayer defeats paper with scissors!");} 
    else if player_gesture == 2 && computer_gesture == 1{println!("The computer chooses: rock\nPlayer defeats rock with paper!");}
    else {println!("The computer chooses: paper\nComputer defeats rock with paper!");}
} // End of move_result()
