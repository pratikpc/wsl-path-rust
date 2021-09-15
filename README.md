# WSL Path

You can use WSLPath to convert a WSL Path to a Windows Path and vice versa

---

## Implementation

Calls [wslpath](https://github.com/MicrosoftDocs/WSL/releases/tag/17046) which is a Linux based utility created by Microsoft to convert Windows and Linux paths.

We call wslpath, pass arguments, perform a conversion and return the results to the user

---

## Converting Windows Path to WSL Path

```rust
fn main() {
	let path = wslpath::windows_to_wsl("C:\\Users").unwrap();
	println!("Windows Path converted to WSL is {}",path);
}
```
### OUTPUT
> Windows Path converted to WSL is /mnt/c/Users

----

## Converting WSL Path to Windows Path

```rust
fn main() {
	let path = wslpath::wsl_to_windows("/mnt/c/Users").unwrap();
	println!("WSL Path converted to Windows is {}",path);
}
```
### OUTPUT
> WSL Path converted to Windows is C:/Users

----

## Converting Windows Path to WSL Path with a specific distro

In this case we are using Ubuntu

```rust
fn main() {
    let path = wslpath::windows_to_wsl_with_distro("C:\\Users", "Ubuntu".to_string()).unwrap();
    println!("Windows Path converted to WSL is {}", path);
}
```
### OUTPUT
> Windows Path converted to WSL is /mnt/c/Users

----

## Converting WSL Path to Windows Path with a specific distro

In this case we are using Ubuntu

```rust
fn main() {
    let path = wslpath::wsl_to_windows_with_distro("/mnt/c/Users", "Ubuntu".to_string()).unwrap();
    println!("WSL Path converted to Windows is {}", path);
}
```
### OUTPUT
> WSL Path converted to Windows is C:/Users

----
