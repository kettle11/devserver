# devserver
An extremely tiny tool to serve a static folder on https://localhost:8000 with a hardcoded self-signed certificate.

This tool is not an example of elegance, security, functionality, or good Rust code.

Likely it is very buggy, but it is useful for people who desire a zero configuration way to serve a local folder over https.

## Installation
```
cargo install devserver
```

## Usage

Open a command line and navigate to the directory you'd like to host then run:
```
devserver
```

Visit https://localhost:8000 to see your hosted content.

