use std::process::Command;

/// wslpath arguments
pub enum Settings {
    WindowsToWsl,
    WslToWindows,
    WslToWindowsLinuxStyle,
}

/// Convert Paths using the WSLPath Executable
pub fn convert_path(
    path: &str,
    distro: Option<String>,
    options: Settings,
    force_absolute_path: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check if distro provided
    let mut args: Vec<String> = match distro {
        Some(distro_name) => vec!["-d".to_string(), distro_name],
        None => vec![],
    };

    args.push("-e".to_string());
    args.push("wslpath".to_string());

    // Select path arg
    // Based on this conversion takes place
    args.push(
        match options {
            Settings::WindowsToWsl => "-u",
            Settings::WslToWindows => "-w",
            Settings::WslToWindowsLinuxStyle => "-m",
        }
        .to_string(),
    );

    // force absolute path arg
    if force_absolute_path {
        args.push("-a".to_string());
    }

    let stdout = Command::new("wsl.exe")
        // Specify the distro to select
        .args(args)
        .arg(path.replace('\\', "\\\\"))
        .output()?
        .stdout;
    let wsl_path = std::str::from_utf8(&stdout)?.trim().to_string();
    Ok(wsl_path)
}

/// Convert WSL Path to Windows Path
/// Pass Path and name of distro
pub fn wsl_to_windows_with_distro(
    path: &str,
    distro: String,
) -> Result<String, Box<dyn std::error::Error>> {
    convert_path(path, Some(distro), Settings::WslToWindowsLinuxStyle, false)
}

/// Convert WSL Path to Windows Path
/// Pass path
/// Uses default distro for execution by default
pub fn wsl_to_windows(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    convert_path(path, None, Settings::WslToWindowsLinuxStyle, false)
}

/// Convert Windows Path to WSL Path
/// Pass Path and name of distro
pub fn windows_to_wsl_with_distro(
    path: &str,
    distro: String,
) -> Result<String, Box<dyn std::error::Error>> {
    convert_path(path, Some(distro), Settings::WindowsToWsl, true)
}

/// Convert Windows Path to WSL Path
/// Pass path
/// Uses default distro for execution by default
pub fn windows_to_wsl(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    convert_path(path, None, Settings::WindowsToWsl, true)
}

#[cfg(test)]
mod tests {
    use crate::{windows_to_wsl, wsl_to_windows};

    // These tests may not execute on all machines
    #[test]
    fn test_wsl_to_windows() {
        assert_eq!(wsl_to_windows("/mnt/c").unwrap_or_default(), "C:/");
    }
    #[test]
    fn test_windows_to_wsl() {
        assert_eq!(windows_to_wsl("C:/").unwrap_or_default(), "/mnt/c/");
    }
}
