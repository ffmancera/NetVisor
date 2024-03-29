use gethostname::gethostname;

struct DiagramCtxt {
    hostname: String,
}

impl DiagramCtxt {
    fn new() -> DiagramCtxt {
        let host = match gethostname().to_str() {
            Some(v) => v.to_string(),
            None => "host".to_string(),
        };
        DiagramCtxt { hostname: host }
    }
}
