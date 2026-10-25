use crate::support::consts::net::{REST_ROOT, SUPABASE_URL};

pub fn auth(path: &str) -> String {
    format!("{}/{}", SUPABASE_URL, path)
}

pub fn rest(table: &str) -> String {
    format!("{}/{}{}", SUPABASE_URL, REST_ROOT, table)
}

pub fn rpc(name: &str) -> String {
    format!("{}/{}rpc/{}", SUPABASE_URL, REST_ROOT, name)
}
