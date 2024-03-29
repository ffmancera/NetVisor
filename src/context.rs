use gethostname::gethostname;

struct Context {
    hostname: String,
}

impl Context {
    fn new() -> Context {
        let host = match gethostname().to_str() {
            Some(v) => v.to_string(),
            None => "host".to_string(),
        };
        Context { hostname: host, }
    }
}
