use super::command::{assert_yes, CommandArgs};
use reqwest::header::HeaderMap;
use serde_json::value::Value;
use std::collections::HashMap;

pub fn replace(command_args: CommandArgs) {
    //Get upstream info
    let schema = CommandArgs.schema;
    let host = CommandArgs.host;
    let upstream_name = CommandArgs.target_name;
    let origin = CommandArgs.origin;
    let dest = CommandArgs.dest;

    let url = schema + "://" + host + "/upstreams/" + upstream_name + "/targets";
    let res = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{}",res)
}
