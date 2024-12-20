use super::system_service::SystemServiceApplication;
use crate::shared::rest_client::system_detail_rest_client::get_system_detail;
use crate::shared::types::system_type::System;
use crate::shared::utils::protocol_util::ProtocolUtil;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct ProtocolServiceApplication;

impl ProtocolServiceApplication {
    pub async fn get_machine_detail() -> Result<System, String> {
        let ips: (String, String) = ProtocolUtil::get_addrs();
        let (select_ip, _) = Self::select_ip(ips);
        // println!("{}", select_ip);
        match SystemServiceApplication::get_system_detail(select_ip) {
            Ok(r) => Ok(r),
            Err(s) => Err(s),
        }
    }

    pub async fn check_machine() -> Result<Vec<System>, String> {
        let ips: (String, String) = ProtocolUtil::get_addrs();
        // println!("ips  wlan : {}, lan: {}", ips.0, ips.1);
        let (select_ip, unselect_ip) = Self::select_ip(ips);
        // println!(
        //     "Select ip : {} | UnSelect ip: {}",
        //     select_ip.clone(),
        //     unselect_ip.clone()
        // );
        let ip = Self::slice_ip(select_ip.clone());
        let mut ips_active = ProtocolUtil::scan_network(&ip).await;
        ips_active.retain(|ip| ip != &unselect_ip);
        // println!("IPS Active : {:?}", ips_active);
        Ok(Self::combine_data_ip_active(ips_active, select_ip.clone()).await)
    }

    async fn combine_data_ip_active(ips_active: Vec<String>, my_ip: String) -> Vec<System> {
        let mut result: Vec<System> = Vec::new();
        let mut handles: Vec<JoinHandle<Result<Option<System>, String>>> = Vec::new();
        for ip in ips_active {
            if my_ip.eq_ignore_ascii_case(&ip) {
                match SystemServiceApplication::get_system_detail(ip) {
                    Ok(r) => {
                        result.push(r);
                    }
                    Err(_) => {
                        // log::error!("Get system detail error: {}", s);
                    }
                }
            } else {
                let s = tokio::task::spawn(get_system_detail(ip));
                handles.push(s);
            }
        }
        Self::get_async_system(&mut result, handles).await;
        result
    }

    async fn get_async_system(
        result: &mut Vec<System>,
        handles: Vec<JoinHandle<Result<Option<System>, String>>>,
    ) {
        for handle in handles {
            if let Ok(r) = handle.await {
                match r {
                    Ok(res) => result.push(res.unwrap()),
                    Err(_) => {
                        // log::error!("Get system detail error: {}", s);
                    }
                }
            }
        }
    }

    pub fn select_ip(ips: (String, String)) -> (String, String) {
        if ips.1.is_empty() {
            (ips.0, ips.1)
        } else {
            (ips.1, ips.0)
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
