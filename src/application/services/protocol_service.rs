use super::system_service::SystemServiceApplication;
use crate::shared::rest_client::system_detail_rest_client::get_system_detail;
use crate::shared::types::response_type::ResponseStruct;
use crate::shared::{
    types::system_type::System,
    utils::protocol_util::{get_addrs, scan_network},
};
use std::thread;
pub struct ProtocolServiceApplication;

impl ProtocolServiceApplication {
    pub async fn check_machine() -> Result<Vec<System>, String> {
        let ips: (String, String) = get_addrs();
        log::debug!("ips  wlan : {}, lan: {}", ips.0, ips.1);
        let (select_ip, unselect_ip) = Self::select_ip(ips);
        log::debug!("Select ip : {} | UnSelect ip: {}", select_ip.clone(), unselect_ip.clone());
        let ip = Self::slice_ip(select_ip.clone());
        let mut ips_active = scan_network(&ip).await;
        ips_active.retain(|ip| ip != &unselect_ip);
        log::debug!("IPS Active : {:?}", ips_active);
        Ok(Self::combine_data_ip_active(ips_active, select_ip.clone()))
    }

    fn combine_data_ip_active(ips_active: Vec<String>, my_ip: String) -> Vec<System> {
        let mut result: Vec<System> = Vec::new();
        let mut handles: Vec<thread::JoinHandle<Option<ResponseStruct<System>>>> = Vec::new();
        for ip in ips_active {
            if my_ip.eq_ignore_ascii_case(&ip) {
                match SystemServiceApplication::get_system_detail(ip) {
                    Ok(r) => {
                        result.push(r);
                    }
                    Err(s) => {
                        log::error!("Get system detail error: {}", s);
                    }
                }
            } else {
                let s = thread::spawn(move || {
                    let resp = get_system_detail(ip);
                    return resp;
                });
                handles.push(s);
            }
        }
        Self::get_async_system(&mut result, handles);
        result
    }

    fn get_async_system(
        result: &mut Vec<System>,
        handles: Vec<thread::JoinHandle<Option<ResponseStruct<System>>>>,
    ) {
        for handle in handles {
            if let Ok(r) = handle.join() {
                if let Some(s) = r {
                    result.push(s.data.unwrap());
                }
            }
        }
    }

    fn select_ip(ips: (String, String)) -> (String, String) {
        if !ips.0.is_empty() {
            (ips.1, ips.0)
        } else {
            (ips.0, ips.1)
        }
    }

    fn slice_ip(ip: String) -> String {
        let mut split_ip: std::str::Split<&str> = ip.split(".");
        let first_part = split_ip.next();
        let second_part = split_ip.next();
        let third_part = split_ip.next();
        format!(
            "{}.{}.{}",
            first_part.unwrap(),
            second_part.unwrap(),
            third_part.unwrap()
        )
    }
}
