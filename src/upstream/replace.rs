use crate::command;
use crate::net;
use crate::upstream;
use crate::upstream::TargetList;

use super::get_all_upstream;

pub fn replace(command_args: &command::CommandArgs) {
    println!();
    println!();
    let is_all = command_args.is_all;
    match is_all {
        true => {
            let all_upstream = get_all_upstream(command_args);
            for upstream in all_upstream {
                println!("Scan upstream <{}>...", &upstream.name);
                replace_one(command_args, &upstream.name)
            }
        }
        false => {
            replace_one(command_args, &command_args.target_name);
        }
    }
}

fn replace_one(command_args: &command::CommandArgs, upstream_name: &String) {
    println!("upstream name {}", upstream_name);
    let url = String::from(&command_args.schema)
        + "://"
        + &command_args.host
        + "/upstreams/"
        + upstream_name
        + "/targets";
    let get_upstream_ret = net::get(&url);
    if get_upstream_ret.is_err() {
        println!("Some error in get upstream : {}", &url);
        return;
    }

    let target_list: TargetList = serde_json::from_str(get_upstream_ret.unwrap().as_str()).unwrap();
    let mut count = 0;
    for target in &target_list.data {
        if target.target != command_args.origin {
            count = count + 1;
            continue;
        }
        // println!("The origin while be replaced : {}", origin);
        command::assert_yes(
            String::from("UPSTREAM[")
                + upstream_name
                + "] The origin<"
                + &command_args.origin
                + "> will be replace to<"
                + &command_args.dest
                + ">",
        );

        upstream::change_upstream_weight(&command_args, upstream_name, &command_args.origin, 0);
        upstream::change_upstream_weight(
            &command_args,
            upstream_name,
            &command_args.dest,
            target.weight,
        );
    }
    if count == target_list.data.len() {
        println!(
            "UPSTREAM[{}]Not found origin <{}>.",
            upstream_name, &command_args.origin
        );
    }
}
