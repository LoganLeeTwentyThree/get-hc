
use std::{env::home_dir};

use winreg::enums::*;
use winreg::{RegKey};

const HCC_URL : &str = "https://github.com/LoganLeeTwentyThree/gup/releases/latest/download/hcc.exe";
const GUP_URL : &str = "https://github.com/LoganLeeTwentyThree/gup/releases/latest/download/gup.exe";

fn get_hcc() -> Result<String, String>{
    let hc_dir = home_dir().unwrap().join(".hc");
    let hcc_file_name = hc_dir.join("hcc.exe");

    let resp = reqwest::blocking::get(HCC_URL).expect("request failed");
    let body = resp.text().expect("body invalid");
    std::fs::write(&hcc_file_name, body)
        .map_err(|e|e.to_string())?;
    Ok(format!("File downloaded to {}", hcc_file_name.to_str().unwrap()))
}

fn get_gup() -> Result<String, String>{
    let hc_dir = home_dir().unwrap().join(".hc");
    let hcc_file_name = hc_dir.join("gup.exe");

    let resp = reqwest::blocking::get(GUP_URL).expect("request failed");
    let body = resp.text().expect("body invalid");
    std::fs::write(&hcc_file_name, body)
        .map_err(|e|e.to_string())?;
    Ok(format!("File downloaded to {}", hcc_file_name.to_str().unwrap()))
}

fn add_path_var() -> Result<String, String>
{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
        .map_err(|e|e.to_string())?;
    let current_path: String = env_key.get_value("Path")
        .map_err(|e|e.to_string())?;

    let hc_dir = home_dir().unwrap().join(".hc");
    if !current_path.contains(hc_dir.to_str().unwrap()){
        let new_path = format!("{};{}", current_path, hc_dir.to_str().unwrap());
        env_key.set_value("Path", &new_path)
            .map_err(|e|e.to_string())?;
        Ok("PATH updated successfully!".into())
    }else {
        Ok("Path already contained hc direcotry".into())
    }
    
    
}

fn main() {
    
    match add_path_var() {
        Ok(msg)=> println!("{msg}"),
        Err(msg) => println!("{msg}"),
    }

    match get_hcc()
    {
        Ok(msg) => println!("{msg}"),
        Err(e) => println!("{e}"),
    }

    match get_gup()
    {
        Ok(msg) => println!("{msg}"),
        Err(e) => println!("{e}"),
    }


    
}
