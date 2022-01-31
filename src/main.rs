extern crate reqwest;
extern crate whoami;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    println!("Starting Installer...");
    
    let mut conf = String::new();
    std::fs::File::open("./esin.conf").unwrap().read_to_string(&mut conf);
    
    if conf.contains("dotnet") {
        dotnet_installer().await;
    }

    if conf.contains("firefox") {
        firefox_installer().await;
    }

    if conf.contains("steam") {
        steam_installer().await;
    }

    if conf.contains("vscode") {
        vscode_installer().await;
    }

    if conf.contains("discord") {
        discord_installer().await;
    }

    println!("Press ENTER to exit");
    let mut xt = String::new();
    std::io::stdin().read_line(&mut xt);
}

async fn discord_installer() {
    println!("Downloading Discord (if it doesn't exist already)...");

    let mut dirtocheck = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/AppData/Roaming/discord/");

    let dircheck = checkexists(String::from(dirtocheck)).await;

    if dircheck.unwrap() == false {
        download(String::from("https://discord.com/api/downloads/distributions/app/installers/latest?channel=stable&platform=win&arch=x86"), String::from("DiscordInstaller.exe"), String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN")).await;
        
        let mut tostart = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN/DiscordInstaller.exe");
        std::process::Command::new(&tostart).spawn();
    }
}

async fn vscode_installer() {
    println!("Downloading Visual Studio Code (if it doesn't exist already)...");

    let mut dirtocheck = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/AppData/Local/Programs/Microsoft VS Code");

    let dircheck = checkexists(String::from(dirtocheck)).await;

    if dircheck.unwrap() == false {
        download(String::from("https://code.visualstudio.com/sha/download?build=stable&os=win32-x64-user"), String::from("VsCode-Installer.exe"), String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN")).await;
        
        let mut tostart = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN/VsCode-Installer.exe");
        std::process::Command::new(&tostart).spawn();
    }
}

async fn steam_installer() {
    println!("Downloading Steam (if it doesn't exist already)...");

    let dircheck = checkexists(String::from("C:/Program Files (x86)/Steam/")).await;

    if dircheck.unwrap() == false {
        download(String::from("https://cdn.cloudflare.steamstatic.com/client/installer/SteamSetup.exe"), String::from("SteamInstaller.exe"), String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN")).await;
        
        let mut tostart = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN/SteamInstaller.exe");
        std::process::Command::new(&tostart).spawn();
    }
}

async fn dotnet_installer() {
    println!("Downloading dotnet framwork...");

    download(String::from("https://download.visualstudio.microsoft.com/download/pr/343dc654-80b0-4f2d-b172-8536ba8ef63b/93cc3ab526c198e567f75169d9184d57/dotnet-sdk-6.0.101-win-x64.exe"), String::from("DotnetInstaller.exe"), String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN")).await;

    let mut tostart = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN/DotnetInstaller.exe");
    std::process::Command::new(&tostart).spawn();
}

async fn download(url: String, file_name: String, dir_name: String) -> Result<()> {
    std::fs::create_dir_all(&dir_name);
    let response = reqwest::get(url).await?;
    let mut nondone = dir_name + "/" + file_name.as_str();
    let mut finale = String::from(&nondone);
    let mut file = std::fs::File::create(finale)?;
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

async fn firefox_installer() {
    println!("Downloading firefox (if it doesn't exist already)...");

    let dircheck = checkexists(String::from("C:/Program Files/Mozilla Firefox/")).await;

    if dircheck.unwrap() == false {
        download(String::from("https://download.mozilla.org/?product=firefox-stub&os=win&lang=en-US"), String::from("FirefoxInstaller.exe"), String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN")).await;
        
        let mut tostart = String::from(String::from("C:/Users/") + whoami::username().as_str() + "/Documents/ESIN/FirefoxInstaller.exe");
        std::process::Command::new(&tostart).spawn();
    }
}

async fn checkexists(dir: String) -> std::io::Result<bool> {
    let metadata = std::fs::metadata(dir)?;
    return Ok(metadata.is_dir());
}