use std::process::Command;

/// Convert Paths using the WSLPath Executable
fn wsl_paths(
    path: &str,
    distro: Option<String>,
    to_linux_path: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check if distro provided
    let distro_args: Vec<String> = match distro {
        Some(distro_name) => vec!["-d".to_string(), distro_name],
        None => vec![],
    };
    // Select path arg
    // Based on this conversion takes place
    let path_arg = {
        if to_linux_path {
            "-u".to_string()
        } else {
            // Convert to Windows
            "-m".to_string()
        }
    };
    let stdout = Command::new("wsl.exe")
        // Specify the distro to select
        .args(distro_args)
        .arg("-e")
        .arg("wslpath")
        .arg(path_arg)
        .arg(path.replace("\\", "\\\\"))
        .output()?
        .stdout;
    let wsl_path = std::str::from_utf8(&stdout)?.trim().to_string();
    Ok(wsl_path)
}

// Convert WSL Path to Windows Path
// Pass Path and name of distro
/// You can pass None if there is no distro
/// None distro support added to make it easier
/// For you to add it to your own library
pub fn wsl_to_windows_with_distro(
    path: &str,
    distro: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    wsl_paths(path, distro, false)
}

/// Convert WSL Path to Windows Path
/// Pass path
/// Uses default distro for execution by default
pub fn wsl_to_windows(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    wsl_to_windows_with_distro(path, None)
}

/// Convert Windows Path to WSL Path
/// Pass Path and name of distro
/// You can pass None if there is no distro
/// None distro support added to make it easier
/// For you to add it to your own library
pub fn windows_to_wsl_for_distro(
    path: &str,
    distro: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    wsl_paths(path, distro, true)
}

/// Convert Windows Path to WSL Path
/// Pass path
/// Uses default distro for execution by default
pub fn windows_to_wsl(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    windows_to_wsl_for_distro(path, None)
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
        assert_eq!(windows_to_wsl("C:/").unwrap_or_default(), "/mnt/c");
    }
}
