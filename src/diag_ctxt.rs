use gethostname::gethostname;
use nispor::{Iface, NisporError};

use crate::network::query_network;

pub struct IfaceCtxt {
    pub iface: Iface,
    depth: u32,
}

pub struct DiagramCtxt {
    pub hostname: String,
    pub ifaces: Vec<IfaceCtxt>,
}

impl DiagramCtxt {
    pub fn new() -> Result<DiagramCtxt, NisporError> {
        let host = match gethostname().to_str() {
            Some(v) => v.to_string(),
            None => "host".to_string(),
        };
        let network_state = query_network()?;
        let iface_ctxts = network_state
            .ifaces
            .values()
            .cloned()
            .map(|np_iface| IfaceCtxt {
                iface: np_iface,
                depth: 0,
            })
            .collect();
        Ok(DiagramCtxt {
            hostname: host,
            ifaces: iface_ctxts,
        })
    }
}
