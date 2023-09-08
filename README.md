# Learning Rust, coming from Python Experience

## Launch Codespaces

* `rustc --version`
* An alternative is by using the command provided by (Rust Up)[https://rustup.rs/]. Its like homebrew for downloading packages. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* Create project using `cargo` command.
* Say you want to do hello world application, you run `cargo new hello`, this will generate a tree structure like following

.<br/>
├── hello<br/>
│   ├── Cargo.toml<br/>
│   └── src<br/>
│       └── main.rs<br/>
├── LICENSE<br/>
├── Makefile<br/>
└── README.md<br/>
* Lets deconstruct the directory structure:

- `Cargo.toml` - Is like the requirements.txt - which describes your dependencies etc.
- To run the main.rs - you CD into the directory and just do `cargo run`

## Using generative AI tools for code completion

- GitHub Copilot is not free, so an alternative is AWSToolkit
- AWS Codewhisperer is a free AI Code Pairing Assistance. Use AWS Builder ID to connect VSCode to AWS Codewhisperer.
- To your email address to sign up for AWS Builder ID for free.
