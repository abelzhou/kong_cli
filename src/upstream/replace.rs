use crate::command;
use crate::net;
use crate::upstream;

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

    let target_list = upstream::search_target_from_upstream(
        command_args,
        upstream_name,
        &command_args.origin,
        false,
    );

    match target_list {
        None => {
            println!(
                "UPSTREAM[{}]Not found origin <{}>.",
                upstream_name, &command_args.origin
            );
        }
        Some(targets) => {
            for target in targets {
                let assert_ret = command::assert_yes(format!(
                    "UPSTREAM[{}] The origin <{}> will be replace to <{}>.",
                    upstream_name, &command_args.origin, &command_args.dest
                ));
                if assert_ret == false {
                    continue;
                }

                upstream::change_upstream_weight(
                    &command_args,
                    upstream_name,
                    &command_args.origin,
                    0,
                );
                upstream::change_upstream_weight(
                    &command_args,
                    upstream_name,
                    &command_args.dest,
                    target.weight,
                );
            }
        }
    }
}
