mod command;
mod upstream;
mod net;

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

   upstream::replace(command_args);
   // net::get(String::from("http://baidu.com"));
   // command::assert_yes(String::from("hellow!"));
}
