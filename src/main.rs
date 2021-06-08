mod command;
mod net;
mod upstream;

fn main() {
   let command_args = command::run();
   println!("Schema value: {}", command_args.schema);
   println!("Host value: {}", command_args.host);
   println!("Target value: {}", command_args.target);
   println!("Option value: {}", command_args.option);
   println!("Target_Name value: {}", command_args.target_name);
   println!("All value: {}", command_args.is_all);
   println!("Origin value: {}", command_args.origin);
   println!("Dest value: {}", command_args.dest);
   println!("Query value: {}", command_args.query_string);
   println!();
   println!();

   match command_args.target.as_ref() {
      "upstream" => {
         upstream_command(command_args);
      }
      "server" => {}
      "route" => {}
      _ => {}
   }
}

//upsrteam command required.
fn upstream_command(command_args: command::CommandArgs) {
   match command_args.option.as_ref() {
      "replace" => {
         upstream::replace(&command_args);
      }
      "add" => {}
      "remove" => {}
      "search" => {
         upstream::search_all_by_ip(&command_args);
      }
      _ => {}
   }
}
