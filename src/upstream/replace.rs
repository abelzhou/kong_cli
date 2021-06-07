use crate::command;
use crate::net;
use crate::upstream::TargetList;

pub fn replace(command_args: command::CommandArgs) {
    // println!("replace func");
    //Get upstream info
    let schema = command_args.schema;
    let host = command_args.host;
    let upstream_name = command_args.target_name;
    let origin = command_args.origin;
    let dest = command_args.dest;
    let is_all = command_args.is_all;
    match is_all {
        true => {}
        false => {
            replace_one(&schema, &host, &upstream_name, &origin, &dest);
        }
    }
}

fn replace_one(
    schema: &String,
    host: &String,
    upstream_name: &String,
    origin: &String,
    dest: &String,
) {
    println!();
    println!();
    let url_prefix = String::from(schema) +  "://" + host ;
    let url = String::from(url_prefix) + "/upstreams/" + upstream_name + "/targets";
    let get_upstream_ret = net::get(&url);
    if get_upstream_ret.is_err() {
        println!("Some error in get upstream : {}", &url);
        return;
    }

    let target_list: TargetList = serde_json::from_str(get_upstream_ret.unwrap().as_str()).unwrap();
    let mut count = 0;
    for target in &target_list.data {
        if target.target != String::from(origin) {
            count = count + 1;
            continue;
        }
        // println!("The origin while be replaced : {}", origin);
        command::assert_yes(String::from("UPSTREAM[") + upstream_name + "] The origin<" + origin +"> will be replace to<" + dest + ">");

        let remove_resp = net::post(&url, &[("target",origin),("weight","0")]);
        if remove_resp.is_ok() {
            println!("Remove target was ok. {}", origin);
        }else {
            println!("Some error in post upstream(remove target) : {}", &url);
            return;
        }
        let add_resp = net::post(&url,&[("target",dest),("weight","100")]);
        if add_resp.is_ok(){
            println!("Add target was ok. {},{}", dest,100);
            
        }else{
            println!("Some error in post upstream(add target) : {}", &url);
            return;
        }
    }
    if count == target_list.data.len() {
        println!("UPSTREAM[{}]Not found origin <{}>.", upstream_name, origin );
    }
}
