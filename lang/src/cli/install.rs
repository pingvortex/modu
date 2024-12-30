use std::io::{Read, Write};

use toml;

pub fn install() {
    let mut content = String::new();
    let file = std::fs::File::open("project.toml");

    if file.is_err() {
        println!("No project.toml found. Run `modu init` to create a new project");
        return;
    }

    file.unwrap().read_to_string(&mut content).unwrap();

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: modu install <name>");
        return;
    }

    let name = &(args[2].clone().split("@").collect::<Vec<&str>>()[0].to_string());
    
    let version = match args[2].clone().split("@").collect::<Vec<&str>>().len() {
        1 => "latest".to_string(),
        _ => args[2].clone().split("@").collect::<Vec<&str>>()[1].to_string()
    };

    let mut client = reqwest::blocking::Client::new();

    let response = client.get(&format!("https://modu-packages.vercel.app/api/v1/packages/{}/{}?isDownload=true", name, version)).send().unwrap();

    if response.status().as_u16() != 200 {
        let text = response.text().unwrap();

        println!("Error: {}", text);

        return;
    }

    let package = response.json::<serde_json::Value>().unwrap();

    println!("Installing package {}@{}", name, package["version"].as_str().unwrap());
    println!("> {}", match package["description"].as_str() {
        Some(description) => description,
        None => "No description"
    });

    let zip = client.get(package["zipUrl"].as_str().unwrap()).send().unwrap().bytes().unwrap();

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip)).unwrap();

    if std::fs::exists(".modu/packages/".to_string() + name).unwrap() {
        std::fs::remove_dir_all(".modu/packages/".to_string() + name).unwrap();
    }

    std::fs::create_dir_all(".modu/packages/".to_string() + name).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let path = ".modu/packages/".to_string() + name + "/" + file.name();

        if file.name().ends_with("/") {
            std::fs::create_dir_all(path).unwrap();
        } else {
            let mut out = std::fs::File::create(path).unwrap();
            std::io::copy(&mut file, &mut out).unwrap();
        }
    }

    let toml = toml::from_str::<toml::Value>(&content).unwrap();
    let mut toml = toml.as_table().unwrap().clone();

    let dependencies = match toml.get_mut("dependencies") {
        Some(dependencies) => dependencies.as_table_mut().unwrap(),

        None => {
            toml.insert("dependencies".to_string(), toml::Value::Table(toml::value::Table::new()));
            toml.get_mut("dependencies").unwrap().as_table_mut().unwrap()
        }
    };

    dependencies.insert(name.clone(), toml::Value::String(package["version"].as_str().unwrap().to_string()));

    let mut file = std::fs::File::create("project.toml").unwrap();
    file.write_all(toml::to_string(&toml).unwrap().as_bytes()).unwrap();

    println!("Package {} installed", name);
}