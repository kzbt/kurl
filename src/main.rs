use url::Url;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = r#"kurl - URL parser and pretty printer

USAGE:
    kurl [OPTIONS] <URL>
    echo <URL> | kurl [OPTIONS]

OPTIONS:
    --json              Output as JSON instead of formatted text
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
        if let Err(e) = print_json(&url) {
            eprintln!("Failed to write JSON output: {}", e);
            std::process::exit(1);
        }
    } else {
        print_pretty(&url);
    }
}

fn print_pretty(url: &Url) {
    println!("URL Components");
    println!("==============");
    println!("  scheme\t: {}", url.scheme());

    if !url.username().is_empty() {
        println!("  user\t\t: {}", url.username());
    }
    if let Some(p) = url.password() {
        println!("  password\t: {}", p);
    }
    if let Some(h) = url.host_str() {
        println!("  host\t\t: {}", h);
    }
    if let Some(p) = url.port() {
        println!("  port\t\t: {}", p);
    }

    println!("  path\t\t: {}", url.path());

    if let Some(f) = url.fragment() {
        println!("  fragment\t: {}", f);
    }

    if url.query().is_some() {
        println!("  query\t\t:");
        for (key, value) in url.query_pairs() {
            println!("    {} = {}", key, value);
        }
    }
}

fn print_json(url: &Url) -> std::io::Result<()> {
    use std::io::{self, Write};

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"{\"scheme\":\"")?;
    write_json_escaped(&mut handle, url.scheme())?;
    handle.write_all(b"\"")?;

    if !url.username().is_empty() {
        handle.write_all(b",\"user\":\"")?;
        write_json_escaped(&mut handle, url.username())?;
        handle.write_all(b"\"")?;
    }
    if let Some(p) = url.password() {
        handle.write_all(b",\"password\":\"")?;
        write_json_escaped(&mut handle, p)?;
        handle.write_all(b"\"")?;
    }
    if let Some(h) = url.host_str() {
        handle.write_all(b",\"host\":\"")?;
        write_json_escaped(&mut handle, h)?;
        handle.write_all(b"\"")?;
    }
    if let Some(p) = url.port() {
        write!(handle, ",\"port\":{}", p)?;
    }

    handle.write_all(b",\"path\":\"")?;
    write_json_escaped(&mut handle, url.path())?;
    handle.write_all(b"\"")?;

    if let Some(f) = url.fragment() {
        handle.write_all(b",\"fragment\":\"")?;
        write_json_escaped(&mut handle, f)?;
        handle.write_all(b"\"")?;
    }

    if url.query().is_some() {
        handle.write_all(b",\"query\":{")?;
        let mut first = true;
        for (key, value) in url.query_pairs() {
            if !first {
                handle.write_all(b",")?;
            }
            first = false;
            handle.write_all(b"\"")?;
            write_json_escaped(&mut handle, &key)?;
            handle.write_all(b"\":\"")?;
            write_json_escaped(&mut handle, &value)?;
            handle.write_all(b"\"")?;
        }
        handle.write_all(b"}")?;
    }

    handle.write_all(b"}\n")?;
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
