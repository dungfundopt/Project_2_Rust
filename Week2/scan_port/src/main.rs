//(phiên bản đơn luồng)
mod common_ports;
mod error;
use crate::error::Error;
mod model;
use crate::model::Subdomain;
mod ports;
mod subdomains;
use reqwest::{blocking::Client, redirect};
use std::env;
use std::time::Duration;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect(); //nhận đối số

    if args.len() != 2 {
        return Err(Error::CliUsage.into()); //nếu không đúng đối số thì báo lỗi
    }

    let target = args[1].as_str(); //lấy đối số thứ 2

    let http_timeout = Duration::from_secs(5);  //thời gian chờ
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))//giới hạn số lần chuyển hướng
        .timeout(http_timeout)
        .build()?;  

    let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)?//lấy danh sách subdomain
        .into_iter() 
        .map(ports::scan_ports)
        .collect();

    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        println!("  Open Ports:");
        for port in &subdomain.open_ports {
            println!("    {}", port.port);
        }

        println!();
    }

    Ok(())
}