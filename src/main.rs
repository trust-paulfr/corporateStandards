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
use windows::{
    core::PCWSTR,
    Win32::System::Registry::{RegSetValueExW, RegOpenKeyExW, HKEY_CURRENT_USER, REG_SZ, KEY_SET_VALUE},
    Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_UPDATEINIFILE, SPIF_SENDCHANGE},
};

async fn change_wallpaper(current_uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let downloads_dir = dirs::download_dir().expect("Could not find the Downloads directory");
    let destination_path = downloads_dir.join(format!("{}.jpg", current_uuid));

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System", KEY_WRITE)?;
    key.set_value("Wallpaper", &destination_path.to_str().unwrap())?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    println!("Changing wallpaper to {}", destination_path.to_str().unwrap());


    Ok(())
}

//#[tokio::main]
//async fn main() {
//    check_uuid_and_update_wallpaper().await;
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // make an bool, by default it's false, if it's false change wallpaper to uuidvar 1 also change bool to true and if it's true change wallpaper to uuidvar 2
    let mut bool = false;
    let mut uuidvar1 = "1";
    let mut uuidvar2 = "2";

    loop {
        let mut current_uuid = uuidvar1;
        if bool {
            current_uuid = uuidvar2;
        }

        change_wallpaper(current_uuid).await?;

        bool = !bool;
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
