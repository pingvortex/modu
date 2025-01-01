use reqwest;
use std::{env, io::Write};


pub fn login() {
    let mut path = String::new();

    if cfg!(windows) {
        let home = env::var("USERPROFILE").unwrap();

        path = format!("{}\\.modu\\token", home);

        std::fs::create_dir_all(format!("{}\\.modu", home)).unwrap();
    } else {
        let home = env::var("HOME").unwrap();

        path = format!("{}/.modu/token", home);

        std::fs::create_dir_all(format!("{}/.modu", home)).unwrap();
    }

    let mut token_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    if token_file.metadata().unwrap().len() > 0 {
        use std::io::Write;

        println!("Already logged in");
        print!("Overwrite? (y/N) ");
        std::io::stdout().flush().unwrap();

        let mut overwrite = String::new();
        std::io::stdin().read_line(&mut overwrite).unwrap();

        if overwrite.trim() != "y" {
            println!("Aborted");
            return;
        }
    }

    println!("Paste the code from https://modu-packages.vercel.app/token");

    let mut token = String::new();
    std::io::stdin().read_line(&mut token).unwrap();

    let token = token.trim();

    let client = reqwest::blocking::Client::new();
    let res = client.get("https://modu-packages.vercel.app/api/v1/code/verify")
        .header("Authorization", token)
        .send().unwrap();

    if res.status().as_u16() != 200 {
        println!("Invalid code");
        return;
    }

    let user_id = res.text().unwrap();

    println!("Authenticated as user with ID {}", user_id);

    token_file.write_all(token.as_bytes()).unwrap();
}