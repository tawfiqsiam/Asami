use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Getters)]
#[get = "pub"]
pub struct Configuration {
    token: String,
    debug_logging: bool,
    pgsql_url: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            token: String::from("NTUyNDIyMjEwNDE4MTE0NTcx.XNaxDQ.dKF1y2WFD_7D0CQshhbq7M72hfQ"),
            debug_logging: false,
            pgsql_url: String::from("postgresql://superuser:root@localhost:5432/asami"),
        }
    }
}
