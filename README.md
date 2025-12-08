# morse-rs

A minimal and fast command-line tool for learning and translating Morse code, written in Rust.

This project aims to provide:
- A simple CLI interface for encoding text to Morse.
- Decoding Morse back to text.
- Optional practice mode for learning and memorizing Morse patterns.

---

## Features

- Encode plain text to Morse code.
- Decode Morse code to plain text.
- Practice mode (planned).
- Fast and safe Rust implementation.

---

## Installation

### From source

```bash
git clone https://github.com/hosseini-rtr/morse-rs.git
cd morse-rs
cargo build --release
````

The compiled binary will be in:

``` bash
target/release/morse-rs
```

Add it to your PATH if needed.

---

## Usage

### Encode text

```bash
morse-rs encode "hello world"
```

### Encode text from file

```bash
morse-rs encode -f <path/to/file>
```

### Decode text

```bash
morse-rs decode ".... . .-.. .-.. ---"
```

### Decode text from file

```bash
morse-rs encode -f <path/to/file>
```

### Practice mode (planned)

```bash
morse-rs practice
```

---

## Roadmap

* [ ] Complete encoding logic
* [ ] Complete decoding logic
* [ ] Complete read file logic
* [ ] Add interactive practice mode
* [ ] Add configuration options (speed, spacing)
* [ ] Publish to crates.io

---

## Contributing

Pull requests are welcome.
---

## License

MIT

