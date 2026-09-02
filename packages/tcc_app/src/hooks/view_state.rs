//! View state hooks

use freya::prelude::*;
use freya::query::{QueryStateData, UseQuery};

pub struct PersistedView {
    pub name: String,
}

pub fn use_view_state() -> PersistedView {
    PersistedView {
        name: "default".to_string(),
    }
}

pub fn settled_or_loading<Q: freya::query::QueryCapability>(
    query: &UseQuery<Q>,
) -> Option<Q::Ok> {
    match &*query.read().state() {
        QueryStateData::Settled { res: Ok(data), .. } => Some(data.clone()),
        QueryStateData::Loading { res: Some(Ok(data)), .. } => Some(data.clone()),
        _ => None,
    }
}
