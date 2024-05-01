use std::collections::HashMap;

use gethostname::gethostname;
use nispor::{Iface, NisporError};

use crate::network::query_network;

#[derive(Clone)]
pub struct IfaceCtxt {
    pub iface: Iface,
    pub depth: u32,
}

impl IfaceCtxt {
    fn calculate_iface_depth(np_iface: &Iface) -> u32 {
        //TODO: this only calculates two different levels of depth, but the idea is to make it
        //extend it in the future
        match np_iface.controller {
            Some(_) => 2,
            None => 1,
        }
    }
}

#[derive(Clone)]
pub struct DiagramCtxt {
    pub hostname: String,
    pub ifaces: HashMap<String, IfaceCtxt>,
}

impl DiagramCtxt {
    pub fn new() -> Result<DiagramCtxt, NisporError> {
        let host = match gethostname().to_str() {
            Some(v) => v.to_string(),
            None => "host".to_string(),
        };
        let network_state = query_network()?;
        let iface_ctxts: Vec<IfaceCtxt> = network_state
            .ifaces
            .values()
            .cloned()
            .map(|np_iface| IfaceCtxt {
                iface: np_iface.clone(),
                depth: IfaceCtxt::calculate_iface_depth(&np_iface),
            })
            .collect();
        let name_ctxts = iface_ctxts
            .iter()
            .map(|iface_ctxt| (iface_ctxt.iface.name.clone(), iface_ctxt.clone()))
            .collect();

        Ok(DiagramCtxt {
            hostname: host,
            ifaces: name_ctxts,
        })
    }

    pub fn count_ifaces_for_depth(self, depth: u32) -> usize {
        self.ifaces
            .values()
            .filter(|iface| iface.depth == depth)
            .count()
    }

    pub fn max_depth(self) -> u32 {
        let mut depth: u32 = 1;
        for iface_ctx in self.ifaces.values() {
            if iface_ctx.depth > depth {
                depth = iface_ctx.depth;
            }
        }

        depth
    }
}
