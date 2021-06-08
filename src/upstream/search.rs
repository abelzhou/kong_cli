use crate::command;
use crate::net;
use crate::upstream;

pub fn get_all_upstream(
    command_args: &command::CommandArgs,
) -> Vec<upstream::Upstream> {
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
