#![windows_subsystem = "windows"]

use std::{env, path::PathBuf};

use mslnk::ShellLink;
use windows::Win32::{
    Foundation::CloseHandle,
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
        TH32CS_SNAPPROCESS,
    },
};

const EXPLORER: &[u8] = b"explorer.exe";

fn is_explorer(buf: &[u16]) -> bool {
    let name = buf.split(|&c| c == 0).next().unwrap_or(buf);
    name.len() == EXPLORER.len()
        && name.iter().zip(EXPLORER).all(|(&wide, &ascii)| {
            let lower = if (b'A' as u16..=b'Z' as u16).contains(&wide) {
                wide + 32
            } else {
                wide
            };
            lower == ascii as u16
        })
}
fn has_explorer() -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut pe32: PROCESSENTRY32W = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        if Process32FirstW(snapshot, &mut pe32).is_ok() {
            loop {
                if is_explorer(&pe32.szExeFile) {
                    CloseHandle(snapshot)?;
                    return Ok(true);
                }

                if Process32NextW(snapshot, &mut pe32).is_err() {
                    break;
                }
            }
        }
        CloseHandle(snapshot)?;
        Ok(false)
    }
}
fn get_reopen_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let appdata = std::env::var_os("APPDATA").ok_or("APPDATA is not set")?;

    Ok(PathBuf::from(appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup"))
}
fn create_auto_reopen(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let sl = ShellLink::new(path)?;

    let link = get_reopen_folder()?.join("rexplorer.lnk");

    println!("link: {}", link.display());
    sl.create_lnk(&link)?;

    Ok(())
}
fn move_to_work_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let local_appdata = std::env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA is not set")?;
    let path = PathBuf::from(local_appdata).join("Rexplorer");
    std::fs::create_dir_all(&path)?;

    let target = path.join("rexplorer.exe");
    let current = env::current_exe()?;

    if current != target {
        std::fs::copy(current, &target)?;
    }

    Ok(target)
}

// TODO

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = move_to_work_dir()?;
    let link = get_reopen_folder()?.join("rexplorer.lnk");

    if !link.try_exists()? {
        create_auto_reopen(&exe_path)?;
    }

    loop {
        if !has_explorer()? {
            std::process::Command::new("explorer.exe").spawn()?;
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}