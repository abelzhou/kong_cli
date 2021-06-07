use crate::command::CommandArgs;
use crate::net;

pub fn replace(command_args: CommandArgs) {
    println!("replace func");
    //Get upstream info
    let schema = command_args.schema;
    let host = command_args.host;
    let upstream_name = command_args.target_name;
    let origin = command_args.origin;
    let dest = command_args.dest;
    
    let url = schema + "://" + &host + "/upstreams/" + &upstream_name + "/targets";
    println!("the url is:{}",url);
    if let Ok(ret) = net::get(url){
        println!("result {:#?}", ret);
    }
}
