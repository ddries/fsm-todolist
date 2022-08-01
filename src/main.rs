use std::{io, num::IntErrorKind};

enum OwnState {
    Idle,
    ShowEntries,
    AddEntry,
    RemoveEntry,
}

enum MenuOption {
    ShowEntries = 0,
    AddEntry,
    RemoveEntry,
    Exit
}

#[derive(Debug)]
struct TodoEntry {
    description: String
}

fn print_welcome() {
    println!("Hello, there!"); 
    println!("Welcome to the worst rust todo-list");
    println!("Please, follow up the instructions");
}

fn print_menu() {
    println!("MENU");
    println!("1. Show Todo entries");
    println!("2. Add new Todo entry");
    println!("3. Remove existing Todo entry");
    println!("4. Exit");
}

fn read_raw_input() -> Option<i32> {
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    match result {
        Ok(_) => {
            let input = input.trim().parse::<i32>();
            match input {
                Ok(result) => Some(result),
                Err(error) => {
                    match error.kind() {
                        IntErrorKind::Empty => (),
                        _ => println!("could not parse stdin, {:?}", error)
                    };
                    None
                },
            }
        },
        Err(_) => {
            None
        },
    }
}

fn int_to_menu_option(int: i32) -> MenuOption {
    match int {
        1 => MenuOption::ShowEntries,
        2 => MenuOption::AddEntry,
        3 => MenuOption::RemoveEntry,
        _ => MenuOption::Exit,
    }
}

fn state_logic(current_state: &mut OwnState, user_input: Option<i32>) {
    match current_state {
        &mut OwnState::Idle => {
            if user_input.is_some() {
                let user_input: i32 = user_input.unwrap();
                let user_input = int_to_menu_option(user_input);

                match user_input {
                    MenuOption::ShowEntries => {
                        *current_state = OwnState::ShowEntries;
                    },
                    MenuOption::AddEntry => {
                        *current_state = OwnState::AddEntry;
                    },
                    MenuOption::RemoveEntry => {
                        *current_state = OwnState::RemoveEntry;
                    },
                    MenuOption::Exit => {
                        println!("good bye!");
                        std::process::exit(0);
                    }
                };
            }
        },
        _ => *current_state = OwnState::Idle,
    }
}

fn output_logic(current_state: &OwnState, todo_entries: &mut Vec<TodoEntry>) {
    match current_state {
        OwnState::ShowEntries => {
            let mut idx: u8 = 0;
            for entry in todo_entries.iter() {
                println!("entry {}: {}", idx, entry.description);
                idx = idx + 1;
            }
        },

        OwnState::AddEntry => {
            println!("enter entry description");
            let mut description = String::new();
            let result = io::stdin().read_line(&mut description);
            match result {
                Ok(_) => {
                    let new_entry: TodoEntry = TodoEntry { description };
                    println!("added entry {:?}", new_entry);
                    todo_entries.push(new_entry);
                },
                Err(_) => println!("could not read description"),
            };
        },

        OwnState::RemoveEntry => {
            println!("enter entry index");
            let mut idx: u8 = 0;
            for entry in todo_entries.iter() {
                println!("entry {}: {}", idx, entry.description);
                idx = idx + 1;
            }
            let mut idx = String::new();
            let result = io::stdin().read_line(&mut idx);
            match result {
                Ok(_) => {
                    let result = idx.trim().parse::<i32>();
                    match result {
                        Ok(result) => {
                            if (result as usize) > todo_entries.len() || result < 0 {
                                println!("given index is not valid!");
                                return;
                            }

                            todo_entries.remove(result as usize);
                            println!("removed index {}", result);
                        },
                        Err(error) => {
                            match error.kind() {
                                IntErrorKind::Empty => (),
                                _ => println!("could not parse index, {:?}", error)
                            };
                        },
                    };

                },
                Err(_) => println!("could not read index"),
            };
        },

        _ => (),
    };
}

fn main() {
    let mut current_state: OwnState = OwnState::Idle;
    let mut todo_entries: Vec<TodoEntry> = Vec::new();

    print_welcome();
    print_menu();

    loop {
        let mut input: Option<i32> = None;
        match current_state {
            OwnState::Idle => {
                input = read_raw_input();
            },
            _ => ()
        };
        state_logic(&mut current_state, input);
        output_logic(&current_state, &mut todo_entries);
    }

}