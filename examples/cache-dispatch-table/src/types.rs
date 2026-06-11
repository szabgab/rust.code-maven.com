use std::collections::HashMap;

pub(crate) type Handler = fn() -> &'static str;
pub(crate) type DispatchTable = HashMap<&'static str, Handler>;
