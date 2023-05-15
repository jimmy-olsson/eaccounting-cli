use colored::Colorize;
use crate::admintool_api;

pub fn handle_list(api: &admintool_api::AdmintoolApi) {
    let flags = api.get_feature_flags();
    println!("Found {} flags.", flags.len());

    for flag in flags {
        if flag.status == "PartlyActivated" {
            println!("{}", flag.name.yellow());
        } else if flag.status == "ActivatedForAll" {
            println!("{}", flag.name.green());
        } else if flag.status == "Inactive" {
            println!("{}", flag.name.red());
        }
    }
}

pub fn handle_enable(api: &admintool_api::AdmintoolApi, flag_name: &String) {
    if api.enable_feature_flag(flag_name) {
        println!("'{}' is now activated", flag_name)
    }
}
