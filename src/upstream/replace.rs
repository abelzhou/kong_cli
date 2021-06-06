use crate::command::CommandArgs;

pub async fn replace(command_args: CommandArgs) {
    //Get upstream info
    let schema = command_args.schema;
    let host = command_args.host;
    let upstream_name = command_args.target_name;
    let origin = command_args.origin;
    let dest = command_args.dest;

    let url = schema + "://" + &host + "/upstreams/" + &upstream_name + "/targets";
    if let Ok(ret) = super::request::get(url).await{
        println!("{:#?}", &ret);
    };
   
}
