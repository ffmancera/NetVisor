use gethostname::gethostname;
use nispor::Iface;

struct IfaceCtxt {
    iface: Iface,
    depth: u32,
}

pub struct DiagramCtxt {
    pub hostname: String,
    ifaces: Vec<IfaceCtxt>,
}

impl DiagramCtxt {
    pub fn new() -> DiagramCtxt {
        let host = match gethostname().to_str() {
            Some(v) => v.to_string(),
            None => "host".to_string(),
        };
        DiagramCtxt { hostname: host, ifaces: Vec::new() }
    }
}
