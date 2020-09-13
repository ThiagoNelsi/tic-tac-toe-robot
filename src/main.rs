use colored::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io;
use termion;

fn main() {
    const SIGNS: [char; 2] = ['O', 'X'];

    let mut user_sign = String::new();

    let mut difficulty: String;

    let mut table: [i8; 9] = [-1; 9];

    let mut turns_counter: i8 = 0;

    let mut player_turn: bool = if rand::thread_rng().gen_range(0, 2) == 1 {
        true
    } else {
        false
    };

    clear_terminal();

    // Intro
    println!("Welcome to the tic-tac-toe game!");
    println!("I'm a robot, try to beat me!!!");

    loop {
        difficulty = "".to_string();
        println!("\nType a level of difficulty 0 - 5: ");

        io::stdin().read_line(&mut difficulty).expect(&format!(
            "{}",
            "Error reading line".red().bold().to_string()
        ));
    
        let difficulty: usize = difficulty.trim().parse().expect(&format!(
            "{}",
            "Error reading line".red().bold().to_string()
        ));

        if difficulty <= 5 {
            break;
        }
    }

    let difficulty: usize = difficulty.trim().parse().expect(&format!(
        "{}",
        "Error reading line".red().bold().to_string()
    ));

    // Choose X or O
    println!("Choose a number: ");
    println!("[0] - {}", "O".green().bold().to_string());
    println!("[1] - {}", "X".red().bold().to_string());

    io::stdin().read_line(&mut user_sign).expect(&format!(
        "{}",
        "Error reading line".red().bold().to_string()
    ));

    let player_sign: i8 = user_sign
        .trim()
        .parse()
        .expect("Please, type a valid number");
    let robot_sign: i8 = if player_sign == 0 { 1 } else { 0 };

    println!("Ok, You've selected the {}", SIGNS[player_sign as usize]);

    clear_terminal();

    // GAME
    println!("{}", "GAME STARTED!!!\n".green().bold().to_string());

    print_table(table);

    loop {
        turns_counter += 1;

        let mut selected_square: String;

        if player_turn {
            println!("\nIt's your turn, select a free space of the table");

            loop {
                selected_square = "".to_string();
                io::stdin().read_line(&mut selected_square).expect(&format!(
                    "{}",
                    "Error reading line".red().bold().to_string()
                ));

                let square: i8 = selected_square.trim().parse().expect(&format!(
                    "{}",
                    "Please type a number".red().bold().to_string()
                ));
                let square: i8 = square - 1;
                if table[square as usize] == -1 {
                    table[square as usize] = player_sign;
                    break;
                } else {
                    println!(
                        "{}",
                        "This square is not free! Try other!"
                            .red()
                            .bold()
                            .to_string()
                    );
                }
            }

            if check_winner(table, player_sign) {
                clear_terminal();
                println!(
                    "{}",
                    "Well Done!!! You won the game!".green().bold().to_string()
                );
                print_table(table);
                break;
            }

            clear_terminal();
        } else {

            println!("\n{}", "It's robot's turn!".blue().bold().to_string());

            let best_position = best_position(table, false, player_sign, robot_sign, difficulty);

            table[best_position as usize] = robot_sign;

            if check_winner(table, robot_sign) {
                clear_terminal();
                println!("{}", "You loose!!!".red().bold().to_string());
                print_table(table);
                break;
            }
        }

        player_turn = !player_turn;

        print_table(table);

        if turns_counter == 9 {
            break;
        }
    }
}

fn best_position(table: [i8; 9], player_turn: bool, player_sign: i8, robot_sign: i8, difficulty: usize) -> i8 {
    let available_positions = get_available_positions(table);

    let mut position_rates: Vec<isize> = vec![0; available_positions.len()];

    for i in 0..available_positions.len() {
        let mut table = table;

        table[available_positions[i as usize] as usize] =
            if player_turn { player_sign } else { robot_sign };

        for _ in 0..difficulty * 5 + 1 {
            let mut table_test = table;
            let mut player_turn_test = player_turn;

            let mut weight = available_positions.len();

            loop {
                weight -= 1;
                player_turn_test = !player_turn_test;

                let available_positions_test = get_available_positions(table_test);
                if available_positions_test == [] {
                    break;
                }

                let random_position = available_positions_test
                    .choose(&mut rand::thread_rng())
                    .expect(&format!("{}", "Robot error".red().bold().to_string()));

                if player_turn_test {
                    table_test[*random_position as usize] = player_sign;

                    if check_winner(table_test, player_sign) {
                        position_rates[i] -= weight as isize;
                        break;
                    }
                } else {
                    table_test[*random_position as usize] = robot_sign;

                    if check_winner(table_test, robot_sign) {
                        position_rates[i] += weight as isize;
                        break;
                    }
                }
            }
        }
    }
    let best = *position_rates
        .iter()
        .max()
        .expect(&format!("{}", "Robot error".green().bold().to_string()));

    let best_position_index = position_rates
        .iter()
        .position(|&s| s == best)
        .expect("Robot Error");

    return available_positions[best_position_index];
}

fn get_available_positions(table: [i8; 9]) -> Vec<i8> {
    let mut available_positions: Vec<i8> = Vec::new();

    // Check available positions
    for i in 0..9 {
        if table[i] == -1 {
            available_positions.push(i as i8);
        }
    }

    available_positions
}

fn print_table(table: [i8; 9]) {
    let mut sign_table: [String; 9] = Default::default();

    for i in 0..9 {
        if table[i] == 0 {
            sign_table[i] = "O".green().bold().to_string();
        } else if table[i] == 1 {
            sign_table[i] = "X".red().bold().to_string();
        } else {
            sign_table[i] = (i + 1).to_string();
        }
    }

    println!("{} | {} | {}", sign_table[0], sign_table[1], sign_table[2]);
    println!("{} | {} | {}", sign_table[3], sign_table[4], sign_table[5]);
    println!("{} | {} | {}", sign_table[6], sign_table[7], sign_table[8]);
}

fn clear_terminal() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

fn check_winner(table: [i8; 9], sign: i8) -> bool {
    // lines
    if table[0] == sign && table[1] == sign && table[2] == sign {
        return true;
    } else if table[3] == sign && table[4] == sign && table[5] == sign {
        return true;
    } else if table[6] == sign && table[7] == sign && table[8] == sign {
        return true;
    }
    // columns
    else if table[0] == sign && table[3] == sign && table[6] == sign {
        return true;
    } else if table[1] == sign && table[4] == sign && table[7] == sign {
        return true;
    } else if table[2] == sign && table[5] == sign && table[8] == sign {
        return true;
    }
    // Cross
    else if table[0] == sign && table[4] == sign && table[8] == sign {
        return true;
    } else if table[2] == sign && table[4] == sign && table[6] == sign {
        return true;
    } else {
        false
    }
}
