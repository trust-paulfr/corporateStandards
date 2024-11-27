extern crate winreg;
use winreg::enums::*;
use winreg::RegKey;
use std::env;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use std::process::Command;
use tokio::runtime::Runtime;
use reqwest::Client;
use tokio::time::{sleep, Duration};

async fn download_image(url: &str, destination: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = File::create(destination)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut file)?;
    Ok(())
}

async fn change_wallpaper(current_uuid: &str, last_uuid: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let downloads_dir = dirs::download_dir().expect("Could not find the Downloads directory");
    let destination_path = downloads_dir.join(format!("{}.jpg", current_uuid));

    if !last_uuid.is_empty() {
        let last_path = downloads_dir.join(format!("{}.jpg", last_uuid));
        if last_path.exists() {
            fs::remove_file(&last_path)?;
        }
    }

    download_image(url, &destination_path).await?;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System", KEY_WRITE)?;
    key.set_value("Wallpaper", &destination_path.to_str().unwrap())?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let output = Command::new("RUNDLL32.EXE")
        .args(&["USER32.DLL,UpdatePerUserSystemParameters"])
        .output()?;

    if output.status.success() {
        println!("Command executed successfully: {:?}", output);
    } else {
        eprintln!("Command failed with error: {:?}", output);
    }


    Ok(())
}

async fn check_uuid_and_update_wallpaper() {
    let client = Client::new();
    let mut last_uuid = String::new();

    loop {
        let response = client.get("https://cachalot.inoctet.fr/uuid").send().await.expect("Failed to get UUID");
        let current_uuid = response.text().await.expect("Failed to read UUID");

        if current_uuid != last_uuid {
            last_uuid = current_uuid.clone();

            if let Err(e) = change_wallpaper(&current_uuid, &last_uuid, "https://cachalot.inoctet.fr/get").await {
                eprintln!("Failed to change wallpaper: {}", e);
            } else {
                println!("The Wallpaper value has been updated successfully.");
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    check_uuid_and_update_wallpaper().await;
}