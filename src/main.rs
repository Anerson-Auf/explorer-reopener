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

fn process_name(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());

    String::from_utf16_lossy(&buf[..end])
}

fn has_explorer() -> Result<bool, Box<dyn std::error::Error>> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut pe32: PROCESSENTRY32W = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let mut has = false;
        if Process32FirstW(snapshot, &mut pe32).is_ok() {
            loop {
                let name = process_name(&pe32.szExeFile);

                if name.eq_ignore_ascii_case("explorer.exe") {
                    has = true;
                    break;
                }

                if Process32NextW(snapshot, &mut pe32).is_err() {
                    break;
                }
            }
        }
        CloseHandle(snapshot)?;
        Ok(has)
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
fn create_auto_reopen(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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
    let file = get_reopen_folder()?.join("rexplorer.lnk");
    if !file.try_exists()? {
        let exe_path = move_to_work_dir()?;
        create_auto_reopen(exe_path)?;
    }
    loop {
        let has = has_explorer()?;
        if !has {
            std::process::Command::new("explorer.exe").spawn()?;
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
