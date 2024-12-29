use reqwest;

pub fn login() {
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
}