use super::TargetList;
use crate::command;
use crate::net;
use crate::upstream;

pub fn get_all_upstream(command_args: &command::CommandArgs) -> Vec<upstream::Upstream> {
    let url = String::from(&command_args.schema) + "://" + &command_args.host + "/upstreams/";

    let get_all_ret = net::get(&url);
    if get_all_ret.is_err() {
        println!("Can not get all upstreams.");
        return Vec::new();
    }

    let upstream_list: upstream::UpstreamList =
        serde_json::from_str(get_all_ret.unwrap().as_str()).unwrap();
    return upstream_list.data;
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
