use chrono::Local;
mod commands;
use commands::Command;
use dialoguer::Select;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use genauai_kernel::{get_db, get_messages, get_plan, reset_database, save_message, Message};
use rustyline::{error::ReadlineError, DefaultEditor};
use std::error::Error as StdError;

const WORKSPACE_ID: i32 = 1;

fn prompt(prompt_text: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt_text)
        .interact()
        .unwrap()
}

fn main() -> Result<(), Box<dyn StdError>> {
    let db = get_db().unwrap();
    let plan = get_plan(&db).unwrap();
    let mut keep_running = true;

    while keep_running {
        let choices = Command::variants();

        let input = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&choices)
            .interact();

        match input {
            Ok(command_str) => {
                let command = Command::from_str(choices[command_str]);
                match command {
                    Some(Command::UpdatePlan) => {
                        let message = prompt("How would you like the plan to be updated?");
                        save_message(
                            &db,
                            &Message {
                                text: message,
                                workspace_id: WORKSPACE_ID,
                                id: -1,
                                created_at: Local::now().to_string(),
                                sender: "user".to_string(),
                            },
                        )?;
                        let messages = get_messages(&db)?;
                        println!("Okay, I'm thinking...");
                        //let updated_plan = update_plan(&messages, &plan);
                        // if let Err(error) = updated_plan {
                        //     save_message(&db, WORKSPACE_ID, "assistant", &format!("{:?}", error))?;
                        //     println!("Error: {:?}", error);
                        // } else {
                        //     let (new_plan, response) = updated_plan.unwrap();
                        //     if let Some(new_plan) = new_plan {
                        //         save_plan(&conn, WORKSPACE_ID, &new_plan)?;
                        //         println!("Plan updated.");
                        //     }
                        //     if let Some(response) = response {
                        //         save_message(&conn, WORKSPACE_ID, "assistant", &response)?;
                        //         println!("{}", response);
                        //     }
                        // }
                    }
                    Some(Command::ExecutePlan) => {
                        println!("Executing plan...");
                    }
                    Some(Command::Show) => {
                        let choice = prompt("What would you like to see (plan, messages)? ");
                        match choice.trim() {
                            "plan" => {
                                println!("{:#?}", plan);
                            }
                            "messages" => {
                                let messages = get_messages(&db)?;
                                for message in &messages {
                                    println!(
                                        "[{}] {}: {}",
                                        message.created_at, message.sender, message.text
                                    );
                                }
                            }
                            _ => {
                                println!("Unknown choice.");
                            }
                        }
                    }
                    Some(Command::Reset) => {
                        let confirm = Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt("Are you sure you want to reset the database?")
                            .default(false)
                            .interact()?;
                        if confirm {
                            reset_database(&db)?;
                            println!("Database reset.");
                        } else {
                            println!("Reset cancelled.");
                        }
                    }
                    Some(Command::Help) => {
                        println!("Help is not yet implemented.");
                    }
                    Some(Command::Exit) => {
                        keep_running = false;
                    }
                    None => {
                        println!("Unknown command.");
                    }
                }
            }
            Err(_) => todo!(),
        }
    }

    Ok(())
}
