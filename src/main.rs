use url::Url;

struct UrlComponents<'a> {
    url: &'a Url,
    user: Option<&'a str>,
}

impl<'a> UrlComponents<'a> {
    fn from_url(url: &'a Url) -> Self {
        let user = if url.username().is_empty() {
            None
        } else {
            Some(url.username())
        };

        Self { url, user }
    }

    fn print_pretty(&self) {
        println!("URL Components:");
        println!("  scheme\t: {}", self.url.scheme());

        if let Some(u) = self.user {
            println!("  user\t\t: {}", u);
        }
        if let Some(p) = self.url.password() {
            println!("  password\t: {}", p);
        }
        if let Some(h) = self.url.host_str() {
            println!("  host\t\t: {}", h);
        }
        if let Some(p) = self.url.port() {
            println!("  port\t\t: {}", p);
        }

        println!("  path\t\t: {}", self.url.path());

        if let Some(f) = self.url.fragment() {
            println!("  fragment\t: {}", f);
        }

        if self.url.query().is_some() {
            println!("  query\t\t:");
            for (key, value) in self.url.query_pairs() {
                println!("    {} = {}", key, value);
            }
        }
    }

    fn print_json(&self) -> std::io::Result<()> {
        use std::io::{self, Write};

        fn write_json_escaped<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
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

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(b"{\"scheme\":\"")?;
        write_json_escaped(&mut handle, self.url.scheme())?;
        handle.write_all(b"\"")?;

        if let Some(u) = self.user {
            handle.write_all(b",\"user\":\"")?;
            write_json_escaped(&mut handle, u)?;
            handle.write_all(b"\"")?;
        }
        if let Some(p) = self.url.password() {
            handle.write_all(b",\"password\":\"")?;
            write_json_escaped(&mut handle, p)?;
            handle.write_all(b"\"")?;
        }
        if let Some(h) = self.url.host_str() {
            handle.write_all(b",\"host\":\"")?;
            write_json_escaped(&mut handle, h)?;
            handle.write_all(b"\"")?;
        }
        if let Some(p) = self.url.port() {
            write!(handle, ",\"port\":{}", p)?;
        }

        handle.write_all(b",\"path\":\"")?;
        write_json_escaped(&mut handle, self.url.path())?;
        handle.write_all(b"\"")?;

        if let Some(f) = self.url.fragment() {
            handle.write_all(b",\"fragment\":\"")?;
            write_json_escaped(&mut handle, f)?;
            handle.write_all(b"\"")?;
        }

        if self.url.query().is_some() {
            handle.write_all(b",\"query\":{")?;
            let mut first = true;
            for (key, value) in self.url.query_pairs() {
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
}

fn main() {
    use std::io::{self, IsTerminal, Read};

    let args: Vec<String> = std::env::args().collect();
    let json_output = args.iter().any(|a| a == "--json");

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
        std::process::exit(1);
    };

    let components = UrlComponents::from_url(&url);

    if json_output {
        if let Err(e) = components.print_json() {
            eprintln!("Failed to write JSON output: {}", e);
            std::process::exit(1);
        }
    } else {
        components.print_pretty();
    }
}
