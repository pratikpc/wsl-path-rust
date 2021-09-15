fn main() {
    // Convert Windows path to WSL
    let path = wslpath::windows_to_wsl("C:\\Users").unwrap();
    println!("Windows Path converted to WSL is {}", path);
    // OUTPUT is
    // Windows Path converted to WSL is /mnt/c/Users

    // Convert WSL path to Windows
    let path = wslpath::wsl_to_windows("/mnt/c/Users").unwrap();
    println!("WSL Path converted to Windows is {}", path);
    // OUTPUT is
    // WSL Path converted to Windows is C:/Users

    // Converting Windows Path to WSL Path with a specific distro
    let path = wslpath::windows_to_wsl_with_distro("C:\\Users", "Ubuntu".to_string()).unwrap();
    println!("Windows Path converted to WSL is {}", path);
    // OUTPUT is
    // Windows Path converted to WSL is /mnt/c/Users

    // Converting WSL Path to Windows Path with a specific distro
    let path = wslpath::wsl_to_windows_with_distro("/mnt/c/Users", "Ubuntu".to_string()).unwrap();
    println!("WSL Path converted to Windows is {}", path);
    // OUTPUT is
    // WSL Path converted to Windows is C:/Users
}
