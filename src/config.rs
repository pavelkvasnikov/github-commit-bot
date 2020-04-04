
pub mod app_config {
    use std::collections::HashMap;
    use crate::constants::*;

    pub fn check_config(config: HashMap<String, String>) {
        let constants_list = vec! [
            PATH,
            TIMEOUT ,
            PATH,
            REPO ,
            SSH_PUB_KEY ,
            SSH_PRIVATE_KEY,
            USERNAME  ,
            EMAIL  ,
            FILE
        ];
        for constant in constants_list.iter() {
            if !config.contains_key(&constant.to_string()) { panic!("Missing param {}", constant); }
            if config[&constant.to_string()].is_empty() { panic!("Param {} is missing", constant);  }
        }

    }
}
