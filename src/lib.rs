use std::process::Command;

/// wslpath conversion arguments
pub enum Conversion {
    WindowsToWsl,
    WslToWindows,
    WslToWindowsLinuxStyle,
}

/// Convert Paths using the WSLPath Executable
pub fn convert(
    path: &str,
    distro: Option<&str>,
    options: Conversion,
    force_absolute_path: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut args = Vec::new();

    if let Some(distro) = distro {
        args.push("-d");
        args.push(distro);
    }
    args.push("-e");
    args.push("wslpath");

    // Select path arg
    // Based on this conversion takes place
    args.push(
        match options {
            Conversion::WindowsToWsl => "-u",
            Conversion::WslToWindows => "-w",
            Conversion::WslToWindowsLinuxStyle => "-m",
        }
    );

    // force absolute path arg
    if force_absolute_path {
        args.push("-a");
    }

    let cmd = Command::new("wsl.exe")
        .args(args)
        .arg(path.replace('\\', "\\\\"))
        .output()?;

    let code = cmd.status.code().unwrap_or(-1);
    if code != 0 {
        return Err(format!("Error getting wslpath: {}", code).into());
    }

    Ok(std::str::from_utf8(&cmd.stdout)?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use crate::{convert, Conversion};

    // These tests may not execute on all machines as they require WSL

    #[test]
    fn test_wsl_to_windows() {
        assert_eq!(convert("/mnt/c", None, Conversion::WslToWindows, false).unwrap_or_default(), "C:\\");
    }

    #[test]
    fn test_wsl_to_windows_absolute() {
        assert_eq!(convert("/mnt/c", None, Conversion::WslToWindows, true).unwrap_or_default(), "C:\\");
    }

    #[test]
    fn test_wsl_to_windows_linux_style() {
        assert_eq!(convert("/mnt/c", None, Conversion::WslToWindowsLinuxStyle, false).unwrap_or_default(), "C:/");
    }

    #[test]
    fn test_wsl_to_windows_linux_style_absolute() {
        assert_eq!(convert("/mnt/c", None, Conversion::WslToWindowsLinuxStyle, true).unwrap_or_default(), "C:/");
    }

    #[test]
    fn test_windows_to_wsl() {
        assert_eq!(convert("C:/", None, Conversion::WindowsToWsl, false).unwrap_or_default(), "/mnt/c/");
    }

    #[test]
    fn test_windows_to_wsl_absolute() {
        assert_eq!(convert("C:/", None, Conversion::WindowsToWsl, true).unwrap_or_default(), "/mnt/c/");
    }

}
