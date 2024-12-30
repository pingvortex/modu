use std::io::{Read, Write};

use toml;
use zip;
use serde_json::json;

static BLOCKLIST: [&str; 3] = [".git", ".gitignore", ".modu"];

fn read_dir(dir: &std::path::Path, archive: &mut zip::ZipWriter<std::fs::File>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let mut do_break = false;

        for item in BLOCKLIST.iter() {
            if path.to_str().unwrap().replace("./", "") == *item {
                println!("Ignoring {}", path.to_str().unwrap());
                do_break = true;
            }
        }

        if do_break {
            continue;
        }

        let mut gitignore_content = String::new();

        let gitignore: Result<_, _> = std::fs::File::open(".gitignore");

        match gitignore {
            Ok(mut file) => {
                file.read_to_string(&mut gitignore_content).unwrap();

                for line in gitignore_content.lines() {
                    if path.to_str().unwrap().replace("./", "") == line {
                        println!("Ignoring {}", path.to_str().unwrap());
                        do_break = true;
                    }
                }
            },

            Err(_) => {}
        }

        if do_break {
            continue;
        }

        if path.is_dir() {
            read_dir(&path, archive);
        } else {
            let name = path.strip_prefix(".").unwrap();

            archive.start_file(name.to_str().unwrap(), zip::write::SimpleFileOptions::default()).unwrap();
            let mut file = std::fs::File::open(path).unwrap();
            let mut contents = String::new();
            
            let r = file.read_to_string(&mut contents);

            match r {
                Ok(_) => {},
                Err(_) => {
                    println!("Could not read file {}", entry.path().to_str().unwrap());
                }
            }

            archive.write_all(contents.as_bytes()).unwrap();
        }
    }
}

pub fn publish() {
    let mut file = std::fs::File::open("project.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let toml: toml::Value = contents.parse().unwrap();
    let package = toml.get("package").unwrap();

    let name = package.get("name").unwrap();
    let version = package.get("version").unwrap();
    let description = match package.get("description") {
        Some(desc) => desc.as_str().unwrap(),
        None => ""
    };

    println!("Publishing {} v{}", name, version);

    print!("Confirm action (y/N): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() != "y" {
        println!("Aborted");
        return;
    }

    let lib_exists = std::path::Path::new("lib.modu").exists();

    if !lib_exists {
        println!("No lib.modu file found, primary package file must be named lib.modu");
        return;
    }

    std::fs::create_dir_all(".modu").unwrap();

    let mut archive = zip::ZipWriter::new(std::fs::File::create(".modu/package.zip").unwrap());
    read_dir(std::path::Path::new("."), &mut archive);
    archive.finish().unwrap();

    println!("[1/2] Package compressed");

    let token: String;

    if cfg!(windows) {
        let home = std::env::var("USERPROFILE").unwrap();
        let path = format!("{}\\.modu\\token", home);
        let mut token_file = std::fs::File::open(path).unwrap();
        let mut token_contents = String::new();
        token_file.read_to_string(&mut token_contents).unwrap();
        token = token_contents;
    } else {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/.modu/token", home);
        let mut token_file = std::fs::File::open(path).unwrap();
        let mut token_contents = String::new();
        token_file.read_to_string(&mut token_contents).unwrap();
        token = token_contents;
    }

    if token.len() == 0 {
        println!("Not logged in, run modu login");
        return;
    }

    let client = reqwest::blocking::Client::new();

    let mut readme = String::new();

    if std::path::Path::new("README.md").exists() {
        let mut file = std::fs::File::open("README.md").unwrap();
        file.read_to_string(&mut readme).unwrap();
    } else if std::path::Path::new("readme.md").exists() {
        let mut file = std::fs::File::open("readme.md").unwrap();
        file.read_to_string(&mut readme).unwrap();
    }

    let body = json!({
        "name": name,
        "version": version,
        "description": description,
        "file": std::fs::read(".modu/package.zip").unwrap(),
        "readme": readme
    });

    let res = client.post("https://modu-packages.vercel.app/api/v1/packages")
        .header("Authorization", token)
        .json(&body)
        .send().unwrap();

    if res.status().as_u16() != 200 {
        let text = res.text().unwrap();

        println!("Error: {}", text);
    } else {
        println!("[2/2] Package uploaded");
    }
}