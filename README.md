# DropZone: Simple Cloud Clipboard

[![crates.io](https://img.shields.io/crates/v/dropzone)](https://crates.io/crates/dropzone)
[![Crates.io](https://img.shields.io/crates/d/dropzone)](https://crates.io/crates/dropzone)

DropZone is a user-friendly CLI tool designed to facilitate seamless data sharing across multiple devices. Whether it's simple text or clipboard content, this lightweight cloud clipboard has got you covered, regardless of your operating system.

### Why DropZone?
I built DropZone to tackle the hassle of sharing text and clipboard data across various devices (Mostly Linux & macOS). The tool achieves this by securely storing your data in a Redis server. For the best experience, I highly recommend leveraging Upstash Redis - a serverless and budget-friendly option, which essentially translates to cost-effectiveness, even free usage in most cases.

### Key Features:

- Share text and clipboard data across multiple machines effortlessly.
- Compatible with most operating systems.
- Configure data expiry (default is 5 minutes).
- Btw, its written in Rust.

## Getting Started:

Install DropZone using Cargo:

```
cargo install dropzone
```

Once you've installed DropZone, setup Redis connection using:

```
dz init
```

## Supported commands

```
  init    Initialize dropzone with a redis connection string
  config  Set a configuration value
  set     Set a value, overwriting if it already exists
  get     Get a value
  yank    Yank clipboard contents
  reset   Reset and clear settings
  help    Print this message or the help of the given subcommand(s)
```

## Usage

### Sync clipboard

1. Copy any text content.
2. Run `dz yank some_key`
3. You can now get this value in any device using `dz get some_key`

### Sync text data

1. Run `dz set some_key your_text_content`
2. You can now get this value in any device using `dz get some_key`

### Configuration

Set a custom expiry time for your data (in seconds):
```
dz config expiry 60
```

Getting a key will automatically add it to your clipboard (true/false):
```
dz config get_to_clipboard true
```
