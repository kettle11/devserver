# devserver [![Crates.io](https://img.shields.io/crates/v/devserver.svg)](https://crates.io/crates/devserver)


An extremely tiny tool to serve a static folder locally.

This tool is only for local development and makes no effort to be secure for other purposes.

**DO NOT USE DEVSERVER IN PRODUCTION**

devserver should only be used for locally hosting files on a trusted network.

devserver does not properly handle the attacks robust servers must withstand on an open network.

## Installation
```
cargo install devserver
```

## Usage
Open a command line and navigate to the directory you'd like to host then run:
```
devserver
```

Visit http://localhost:8080 or https://localhost:8080 to see your hosted content.

## Options
`--reload`  Automatically refresh pages when a file in the hosted folder changes.

`--address` Pass an address like "127.0.0.1:8080" or "localhost:8000" to change the address the server will host on.

`--path`    Changes the directory to be hosted.

`--help`    Explains available options.

