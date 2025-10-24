# kurl

A lightweight, CLI tool to parse and pretty print URL components in plain text or JSON format.

## Installation

Homebrew tap:
```
brew install kzbt/tap/kurl
```

Cargo:
```
git clone https://github.com/kzbt/kurl.git
cd kurl
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

The release binary will be at `target/release/kurl`.

## Usage

### Pretty Print (Default)

```bash
kurl "https://user:pass@example.com:8080/path?key=value&foo=bar#fragment"
```

Output:
```
URL Components
==============
  scheme	  : https
  user		  : user
  password	: pass
  host		  : example.com
  port		  : 8080
  path		  : /path
  fragment	: fragment
  query		  :
    key = value
    foo = bar
```

### JSON Output

```bash
kurl "https://example.com/path?q=1" --json
```

Output:
```json
{"scheme":"https","host":"example.com","path":"/path","query":{"q":"1"}}
```

### From Stdin

```bash
echo "https://example.com/path" | kurl
echo "https://example.com/path" | kurl --json
```

## Flags

| Flag | Alias | Description |
|------|-------|-------------|
| `--json` | `-j` | Output URL components as JSON |
| `--help` | `-h` | Show help message |
| `--version` | `-V` | Show version information |

## License

MIT License - See [LICENSE](LICENSE) file for details.

## Development

Format code:
```bash
cargo fmt
```

Check for issues:
```bash
cargo clippy
```
