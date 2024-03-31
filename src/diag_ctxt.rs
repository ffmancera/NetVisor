use gethostname::gethostname;
use nispor::{Iface, NisporError};

use crate::network::query_network;

#[derive(Clone)]
pub struct IfaceCtxt {
    pub iface: Iface,
    pub depth: u32,
}

#[derive(Clone)]
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
                depth: 1,
            })
            .collect();
        Ok(DiagramCtxt {
            hostname: host,
            ifaces: iface_ctxts,
        })
    }

    pub fn count_ifaces_for_depth(self, depth: u32) -> usize {
        self.ifaces
            .iter()
            .filter(|iface| iface.depth == depth)
            .count()
    }
}
