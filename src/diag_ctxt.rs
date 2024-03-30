use gethostname::gethostname;

pub struct DiagramCtxt {
    pub hostname: String,
}

impl DiagramCtxt {
    pub fn new() -> DiagramCtxt {
        let host = match gethostname().to_str() {
            Some(v) => v.to_string(),
            None => "host".to_string(),
        };
        DiagramCtxt { hostname: host }
    }
}
