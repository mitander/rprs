struct Config {
    entrypoints: Vec<EntryPoint>,
    applications: Vec<Application>,
    log_path: std::path::Path,
}

struct EntryPoint {
    name: String,
    address: SocketAddr,
    protocol: Protocol,
}

struct Application {
    domain: String,
    port: u32,
    protocol: Protocol,
    path: String,
}

enum Protocol {
    Secure(TLS),
    Insecure,
}

struct TLS {
    cert_file: String,
    key_file: String,
}
