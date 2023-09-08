# Learning Rust, coming from Python Experience

## Launch Codespaces

* `rustc --version`
* An alternative is by using the command provided by (Rust Up)[https://rustup.rs/]. Its like homebrew for downloading packages. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Create project using `cargo` command.
* Say you want to do hello world application, you run `cargo new hello`, this will generate a tree structure like following

.
├── hello
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── LICENSE
├── Makefile
└── README.md
* Lets deconstruct the directory structure:

- `Cargo.toml` - Is like the requirements.txt - which describes your dependencies etc.
- To run the main.rs - you CD into the directory and just do `cargo run`