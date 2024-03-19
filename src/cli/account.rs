use clap::Args;
use dialoguer;
use reqwest;


#[derive(Args)]
pub struct Cli {}

pub fn account() {
    let theme = dialoguer::theme::ColorfulTheme::default();

    let actions = vec!["create", "delete", "login", "logout"];
    let action_selection = dialoguer::Select::with_theme(&theme)
        .with_prompt("What would you like to do")
        .items(&actions)
        .interact()
        .unwrap();

   
}


fn create() {}

fn delete() {}

fn login() {
    let theme = dialoguer::theme::ColorfulTheme::default();

    let username: String = dialoguer::Input::with_theme(&theme)
        .with_prompt("Username")
        .interact_text()
        .unwrap();

        let password: String = dialoguer::Password::with_theme(&theme)
        .with_prompt("Password")
        .interact()
        .unwrap();

        

println!("{}, {}", username, password)
}

fn logout() {}



