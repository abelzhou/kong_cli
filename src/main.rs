use clap::{App,Arg};

fn main() {
    let matches = App::new("KongCLI")
                            .version("0.1.0")
                            .author("abelzhou<abel.zhou@hotmail.com>")
                            .arg(Arg::with_name("TARGET")
                                .help("server/route/upstream")
                                .required(true)
                                .index(1)
                                .possible_values(&["server","route","upstream"]))
                            .arg(Arg::with_name("OPTION")
                                .help("add/remove/replace")
                                .required(true)
                                .index(2)
                                .possible_values(&["add","remove","replace"])
                            )
                            .arg(Arg::with_name("TARGET_NAME")
                                .help("Target service name.")
                                .required_unless("all")
                                .index(3))

                            .arg(Arg::with_name("all")
                                .short("a")
                                .required_unless("TARGET_NAME")
                                .help("Search all upstream or server or router.")
                            )
                            .arg(Arg::with_name("origin")
                                .short("o")
                                .long("origin")
                                .required_if("OPTION","replace")
                                .takes_value(true)
                                .help("Its required when the target bust be replace.")
                            )
                            .arg(Arg::with_name("dist")
                                .short("d")
                                .long("dist")
                                .takes_value(true)
                                .required_if("OPTION","replace")
                                .help("Its required when the target bust be replace.")
                            )
                            // .subcommand(SubCommand::with_name("debug")
                            //                     .help("Print debug information verbosely")
                            //                 )
                            // .arg(Arg::with_name("all")
                            //    .short("c")
                            //    .long("config")
                            //    .value_name("FILE")
                            //    .help("Sets a custom config file")
                            //    .takes_value(true))
                            // .arg(Arg::with_name("v")
                            //    .short("v")
                            //    .multiple(true)
                            //    .help("Sets the level of verbosity"))
                            // .subcommand(SubCommand::with_name("test")
                            //           .about("controls testing features")
                            //           .version("1.3")
                            //           .author("Someone E. <someone_else@other.com>")
                            //           .arg(Arg::with_name("debug")
                            //               .short("d")
                            //               .help("print debug information verbosely")))
                          .get_matches();

    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let target =matches.value_of("TARGET").unwrap();
    let option = matches.value_of("OPTION").unwrap();
    let target_name = matches.value_of("TARGET_NAME").unwrap_or(&"");
    println!("Target: {}", target);
    println!("OPTION: {}", option);
    println!("TARGET_NAME: {}", target_name);

    let origin = matches.value_of("origin").unwrap_or(&"");
    let dist = matches.value_of("dist").unwrap_or(&"");
    println!("ORIGIN: {}", origin);
    println!("DIST: {}", dist);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    // if let Some(matches) = matches.subcommand_matches("debug") {
    //     println!("Printing debug information...");
    //     if matches.is_present("debug") {
    //         println!("Printing debug info...");
    //     } else {
    //         println!("Printing normally...");
    //     }
    // }
}
