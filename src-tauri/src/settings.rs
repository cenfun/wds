use local_ip_address::list_afinet_netifas;
use serde::{Deserialize, Serialize};

use crate::common::get_data_file_object;
use crate::common::save_data_file_object;

use crate::cache::get_settings_cache;
use crate::cache::remove_settings_cache;
use crate::cache::update_settings_cache;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,

    //read/write
    #[serde(default)]
    pub permission: String,
}

impl Default for DirItem {
    fn default() -> Self {
        Self {
            id: rand::random::<u64>().to_string(),
            name: "".into(),
            path: "".into(),
            permission: "read".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub dir_list: Vec<DirItem>,
}

impl Default for ProfileItem {
    fn default() -> Self {
        Self {
            id: rand::random::<u64>().to_string(),
            username: "".into(),
            password: "".into(),
            dir_list: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub port: u16,
    pub profile_list: Vec<ProfileItem>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            address: "".into(),
            port: 8090,
            profile_list: vec![],
        }
    }
}

//======================================================================================

pub fn save_dir(action: &str, id: String, data: DirItem) -> bool {
    let mut settings = get_settings();
    let res = settings.profile_list.iter_mut().find(|it| it.id == id);
    if let Some(profile) = res {
        let dir_list = &mut profile.dir_list;

        return match action {
            "create" => {
                let mut item = DirItem::default();
                item.name = data.name;
                item.path = data.path;
                item.permission = data.permission;
                dir_list.push(item);
                save_settings(settings)
            }
            "delete" => {
                for (i, item) in dir_list.iter().enumerate() {
                    if item.id == data.id {
                        dir_list.remove(i);
                        return save_settings(settings);
                    }
                }
                false
            }
            "update" => {
                for item in dir_list.iter_mut() {
                    if item.id == data.id {
                        item.name = data.name;
                        item.path = data.path;
                        item.permission = data.permission;
                        return save_settings(settings);
                    }
                }
                false
            }
            _ => false,
        };
    }
    false
}

//======================================================================================

pub fn save_profile(action: &str, data: ProfileItem) -> bool {
    let mut settings = get_settings();
    match action {
        "create" => {
            let mut item = ProfileItem::default();
            item.username = data.username;
            item.password = data.password;
            settings.profile_list.push(item);
            save_settings(settings)
        }
        "delete" => {
            for (i, item) in settings.profile_list.iter().enumerate() {
                if item.id == data.id {
                    settings.profile_list.remove(i);
                    return save_settings(settings);
                }
            }
            false
        }
        "update" => {
            for item in settings.profile_list.iter_mut() {
                if item.id == data.id {
                    item.username = data.username;
                    item.password = data.password;
                    return save_settings(settings);
                }
            }
            false
        }
        _ => false,
    }
}

pub fn get_profile_by_login(username: String, password: String) -> Option<ProfileItem> {
    let settings = get_settings();
    let res = settings.profile_list.iter().find(|item| {
        if item.username == username {
            if username.is_empty() {
                return true;
            }
            if item.password == password {
                return true;
            }
        }
        false
    });
    match res {
        Some(p) => Some(p.clone()),
        None => None,
    }
}

pub fn get_profile_by_id(id: String) -> Option<ProfileItem> {
    let settings = get_settings();
    let res = settings.profile_list.iter().find(|item| item.id == id);
    match res {
        Some(p) => Some(p.clone()),
        None => None,
    }
}

//======================================================================================

fn get_address(port: u16) -> String {
    let mut my_ip = String::from("127.0.0.1");

    let network_interfaces = list_afinet_netifas();
    if let Ok(network_interfaces) = network_interfaces {
        for (_name, ip) in network_interfaces.iter() {
            // println!("{}:\t{:?}", name, ip);
            let ip_str = ip.to_string();
            if ip_str.starts_with("192.") || ip_str.starts_with("10.") {
                my_ip = ip_str;
            }
        }
    } else {
        println!("Error getting network interfaces: {:?}", network_interfaces);
    }

    let address = format!("http://{}:{}", my_ip, port);

    address
}

pub fn get_settings() -> Settings {
    if let Some(settings) = get_settings_cache() {
        return settings;
    }

    let mut settings = match get_data_file_object("settings.json") {
        Ok(s) => s,
        Err(_) => Settings::default(),
    };

    settings.address = get_address(settings.port);

    update_settings_cache(settings.clone());

    settings
}

pub fn save_settings(settings: Settings) -> bool {
    let res = save_data_file_object("settings.json", settings);
    remove_settings_cache();
    res
}

pub fn save_port(port: u16) -> bool {
    let mut settings = get_settings();
    settings.port = port;
    return save_settings(settings);
}

//======================================================================================
