# Rust Portscanner
This is a simple port scanner written in Rust. It is developed to aid in identifying open ports on a given IP address.

How to Use
Make sure you have Rust installed on your machine. You can install it from the official Rust website.

Clone this repository to your local machine:

```console
git clone https://github.com/JonathLima/scanner-open-ports.git
```

Navigate to the project directory:

```console
cd scanner-open-ports
```
Build the project using Cargo:

```console
cargo build
```
Run the program with the desired IP address as an argument:

```console
cargo run <IP>
```
Replace <IP> with the IP address you want to check.

Example:
```console
cargo run 192.168.0.1
``` 
This command will run the port scanner on the 192.168.0.1 IP address.

Contributing
Feel free to contribute to this project. You can open issues or submit pull requests with improvements or fixes.

License
This project is licensed under the MIT License. See the [LICENSE](https://opensource.org/license/mit) for details.
