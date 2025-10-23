use url::Url;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = r#"kurl - URL parser and pretty printer

USAGE:
    kurl [OPTIONS] <URL>
    echo <URL> | kurl [OPTIONS]

OPTIONS:
    -j, --json          Output as JSON instead of formatted text
    -h, --help          Show this help message
    -V, --version       Show version information

EXAMPLES:
    kurl "https://user:pass@example.com:8080/path?key=value#fragment"
    echo "https://example.com/path" | kurl --json
"#;

fn main() {
    use std::io::{self, IsTerminal, Read};

    let args: Vec<String> = std::env::args().collect();

    let mut json_output = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{}", HELP_TEXT);
                return;
            }
            "-V" | "--version" => {
                println!("kurl {}", VERSION);
                return;
            }
            "-j" | "--json" => {
                json_output = true;
            }
            _ => {}
        }
    }

    let url = if let Some(url_arg) = args.iter().skip(1).find(|a| a.as_str() != "--json") {
        Url::parse(url_arg).unwrap_or_else(|e| {
            eprintln!("Failed to parse URL: {}", e);
            std::process::exit(1);
        })
    } else if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Failed to read from stdin: {}", e);
            std::process::exit(1);
        });

        let trimmed = buffer.trim();
        if trimmed.is_empty() {
            eprintln!("Error: URL cannot be empty");
            std::process::exit(1);
        }

        Url::parse(trimmed).unwrap_or_else(|e| {
            eprintln!("Failed to parse URL: {}", e);
            std::process::exit(1);
        })
    } else {
        eprintln!("Usage: {} [--json] <url>", args[0]);
        eprintln!("   or: echo <url> | {} [--json]", args[0]);
        eprintln!("\nUse --help for more information.");
        std::process::exit(1);
    };

    if json_output {
        print_json(&url);
    } else {
        print_pretty(&url);
    }
}

fn print_pretty(url: &Url) {
    let _ = print_pretty_impl(&mut std::io::stdout(), url);
}

fn print_json(url: &Url) {
    let _ = print_json_impl(&mut std::io::stdout().lock(), url);
}

fn print_pretty_impl<W: std::io::Write>(writer: &mut W, url: &Url) -> std::io::Result<()> {
    writeln!(writer, "URL Components")?;
    writeln!(writer, "==============")?;
    writeln!(writer, "  scheme\t: {}", url.scheme())?;

    if !url.username().is_empty() {
        writeln!(writer, "  user\t\t: {}", url.username())?;
    }
    if let Some(p) = url.password() {
        writeln!(writer, "  password\t: {}", p)?;
    }
    if let Some(h) = url.host_str() {
        writeln!(writer, "  host\t\t: {}", h)?;
    }
    if let Some(p) = url.port() {
        writeln!(writer, "  port\t\t: {}", p)?;
    }

    writeln!(writer, "  path\t\t: {}", url.path())?;

    if let Some(f) = url.fragment() {
        writeln!(writer, "  fragment\t: {}", f)?;
    }

    if url.query().is_some() {
        writeln!(writer, "  query\t\t:")?;
        for (key, value) in url.query_pairs() {
            writeln!(writer, "    {} = {}", key, value)?;
        }
    }

    Ok(())
}

fn print_json_impl<W: std::io::Write>(writer: &mut W, url: &Url) -> std::io::Result<()> {
    writer.write_all(b"{\"scheme\":\"")?;
    write_json_escaped(writer, url.scheme())?;
    writer.write_all(b"\"")?;

    if !url.username().is_empty() {
        writer.write_all(b",\"user\":\"")?;
        write_json_escaped(writer, url.username())?;
        writer.write_all(b"\"")?;
    }
    if let Some(p) = url.password() {
        writer.write_all(b",\"password\":\"")?;
        write_json_escaped(writer, p)?;
        writer.write_all(b"\"")?;
    }
    if let Some(h) = url.host_str() {
        writer.write_all(b",\"host\":\"")?;
        write_json_escaped(writer, h)?;
        writer.write_all(b"\"")?;
    }
    if let Some(p) = url.port() {
        write!(writer, ",\"port\":{}", p)?;
    }

    writer.write_all(b",\"path\":\"")?;
    write_json_escaped(writer, url.path())?;
    writer.write_all(b"\"")?;

    if let Some(f) = url.fragment() {
        writer.write_all(b",\"fragment\":\"")?;
        write_json_escaped(writer, f)?;
        writer.write_all(b"\"")?;
    }

    if url.query().is_some() {
        writer.write_all(b",\"query\":{")?;
        let mut first = true;
        for (key, value) in url.query_pairs() {
            if !first {
                writer.write_all(b",")?;
            }
            first = false;
            writer.write_all(b"\"")?;
            write_json_escaped(writer, &key)?;
            writer.write_all(b"\":\"")?;
            write_json_escaped(writer, &value)?;
            writer.write_all(b"\"")?;
        }
        writer.write_all(b"}")?;
    }

    writer.write_all(b"}\n")?;
    Ok(())
}

fn write_json_escaped<W: std::io::Write>(writer: &mut W, s: &str) -> std::io::Result<()> {
    for c in s.chars() {
        match c {
            '"' => writer.write_all(b"\\\"")?,
            '\\' => writer.write_all(b"\\\\")?,
            '\n' => writer.write_all(b"\\n")?,
            '\r' => writer.write_all(b"\\r")?,
            '\t' => writer.write_all(b"\\t")?,
            c if c.is_control() => write!(writer, "\\u{:04x}", c as u32)?,
            c => write!(writer, "{}", c)?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_pretty_basic() {
        let url = Url::parse("https://example.com/path").unwrap();
        let mut output = Vec::new();

        let result = print_pretty_impl(&mut output, &url);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("scheme"));
        assert!(output_str.contains("https"));
        assert!(output_str.contains("example.com"));
        assert!(output_str.contains("/path"));
    }

    #[test]
    fn test_print_pretty_with_query() {
        let url = Url::parse("https://example.com?key=value&foo=bar").unwrap();
        let mut output = Vec::new();

        let result = print_pretty_impl(&mut output, &url);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("query"));
        assert!(output_str.contains("key = value"));
        assert!(output_str.contains("foo = bar"));
    }

    #[test]
    fn test_print_pretty_with_credentials() {
        let url = Url::parse("https://user:pass@example.com").unwrap();
        let mut output = Vec::new();

        let result = print_pretty_impl(&mut output, &url);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("user"));
        assert!(output_str.contains("password"));
    }

    #[test]
    fn test_print_json_basic() {
        let url = Url::parse("https://example.com/path").unwrap();
        let mut output = Vec::new();

        let result = print_json_impl(&mut output, &url);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("\"scheme\":\"https\""));
        assert!(output_str.contains("\"host\":\"example.com\""));
        assert!(output_str.contains("\"path\":\"/path\""));
    }

    #[test]
    fn test_print_json_with_query() {
        let url = Url::parse("https://example.com?key=value").unwrap();
        let mut output = Vec::new();

        let result = print_json_impl(&mut output, &url);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("\"query\""));
        assert!(output_str.contains("\"key\":\"value\""));
    }

    #[test]
    fn test_write_json_escaped_quotes() {
        let mut output = Vec::new();
        let result = write_json_escaped(&mut output, "test\"quote");
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, "test\\\"quote");
    }

    #[test]
    fn test_write_json_escaped_backslash() {
        let mut output = Vec::new();
        let result = write_json_escaped(&mut output, "test\\path");
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, "test\\\\path");
    }

    #[test]
    fn test_write_json_escaped_newline() {
        let mut output = Vec::new();
        let result = write_json_escaped(&mut output, "test\nline");
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, "test\\nline");
    }
}
