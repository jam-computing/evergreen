use serde::{Deserialize, Serialize};

pub trait DbItem: Serialize + Deserialize<'static> {}
