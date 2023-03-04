use wslpath2::{convert, Conversion};

fn main() {
    // Convert Windows path to WSL
    let path = convert("C:\\Users", None, Conversion::WindowsToWsl, false).unwrap();
    println!("Windows Path converted to WSL is {}", path);
    // OUTPUT is Windows Path converted to WSL is /mnt/c/Users

    // Convert WSL path to Windows
    let path = convert("/mnt/c/Users", None, Conversion::WslToWindows, false).unwrap();
    println!("WSL Path converted to Windows is {}", path);
    // OUTPUT is WSL Path converted to Windows is C:\Users

    // Convert WSL path to Windows Linux Style
    let path = convert("/mnt/c/Users", None, Conversion::WslToWindowsLinuxStyle, false).unwrap();
    println!("WSL Path converted to Windows is {}", path);
    // OUTPUT is WSL Path converted to Windows is C:/Users
}
