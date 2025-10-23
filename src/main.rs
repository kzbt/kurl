fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let url = url::Url::parse(&args[1]).unwrap();

    let scheme = url.scheme();
    let user = url.username();
    let password = url.password();
    let host = url.host_str();
    let port = url.port();
    let path = url.path();
    let fragment = url.fragment();
    let query = url.query();

    println!("URL Components:");
    println!("  scheme\t: {}", scheme);
    if user != "" {
        println!("  user\t\t: {}", user);
    }
    if let Some(p) = password {
        println!("  password\t: {}", p);
    }
    if let Some(h) = host {
        println!("  host\t\t: {}", h);
    }
    if let Some(p) = port {
        println!("  port\t\t: {}", p);
    }
    println!("  path\t\t: {}", path);
    if let Some(f) = fragment {
        println!("  fragment\t: {}", f);
    }

    if let Some(q) = query {
        println!("  query\t\t: {}", q);
        for (key, value) in url.query_pairs() {
            println!("    {} = {}", key, value);
        }
    }
}
