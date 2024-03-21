use nispor::{NetState, NisporError};

pub fn query_network() -> Result<NetState, NisporError> {
    let net_state = NetState::retrieve()?;

    Ok(net_state)
}
