use crate::command;
use crate::net;

pub fn change_upstream_weight(
    command_args: &command::CommandArgs,
    upstream_target: &String,
    weight: u32,
) {
    let url = String::from(&command_args.schema)
        + "://"
        + &command_args.host
        + "/upstreams/"
        + &command_args.target_name
        + "/targets";

    let change_resp = net::post(
        &url,
        &[
            ("target", upstream_target),
            ("weight", weight.to_string().as_ref()),
        ],
    );
    if change_resp.is_ok() {
        println!(
            "Change target weight was ok. tagret:{} weight:{}",
            upstream_target, weight
        );
    } else {
        println!("Some error in change upstream weight : {}", &url);
        return;
    }
}
