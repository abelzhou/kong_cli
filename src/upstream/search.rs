use super::TargetList;
use super::Upstream;
use crate::command;
use crate::net;
use crate::upstream;

pub fn get_all_upstream(command_args: &command::CommandArgs) -> Vec<upstream::Upstream> {
    let url_prefix = String::from(&command_args.schema) + "://" + &command_args.host;
    let mut next_page = String::from("/upstreams/");
    let mut upstream_all:Vec<Upstream> = Vec::new();
    loop{
        let url = String::from(&url_prefix) + &next_page;

        let get_all_ret = net::get(&url);
        if get_all_ret.is_err() {
            println!("Can not get all upstreams.");
            return Vec::new();
        }
        
        let mut upstream_list: upstream::UpstreamList =
            serde_json::from_str(get_all_ret.unwrap().as_str()).unwrap();
        upstream_all.append(&mut upstream_list.data);

        match upstream_list.next {
            Some(next_page_uri)=>{
                next_page = next_page_uri;
            }
            None =>{
                break;
            }
        }
    }
    return upstream_all;
}

pub fn search_all_by_ip(command_args: &command::CommandArgs) {
    let all_upstream = get_all_upstream(command_args);

    for upstream in all_upstream {
        let url = String::from(&command_args.schema)
            + "://"
            + &command_args.host
            + "/upstreams/"
            + &upstream.name
            + "/targets";

        let get_upstream_ret = net::get(&url);
        if get_upstream_ret.is_err() {
            println!("Some error in get upstream : {}", &url);
            continue;
        }
        let target_list: TargetList =
            serde_json::from_str(get_upstream_ret.unwrap().as_str()).unwrap();
        for target in &target_list.data {
            if target.target.contains(&command_args.query_string) {
                println!(
                    "[upstream] -> {}    [target] -> {}   [weight] -> {}",
                    &upstream.name, &target.target, &target.weight
                )
            }
        }
    }
}
