use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Subdomain {  //subdomain
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug, Clone)] //port
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

#[derive(Debug, Deserialize, Clone)]   //deserialize dữ liệu từ json
pub struct CrtShEntry {
    pub name_value: String,
}