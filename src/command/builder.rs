use clap::{App, Arg};

//the struct of args.
pub struct CommandArgs {
    pub host: String,
    pub schema: String,
    pub target: String,
    pub option: String,
    pub target_name: String,
    pub is_all: bool,
    pub origin: String,
    pub dest: String,
    pub query_string: String,
}

pub fn run() -> CommandArgs {
    let matches = App::new("KongCLI")
        .version("0.1.0")
        .author("abelzhou<abel.zhou@hotmail.com>")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .default_value("127.0.0.1:8001")
                .takes_value(true)
                .required(false)
                .help("The host of kong server."),
        )
        .arg(
            Arg::with_name("schema")
                .short("s")
                .long("schema")
                .takes_value(true)
                .required(false)
                .default_value("http")
                .help("The schema of kong server."),
        )
        .arg(
            Arg::with_name("TARGET")
                .help("server/route/upstream")
                .required(true)
                .index(1)
                .possible_values(&["server", "route", "upstream"]),
        )
        .arg(
            Arg::with_name("OPTION")
                .help("add/remove/replace/search")
                .required(true)
                .index(2)
                .possible_values(&["add", "remove", "replace", "search"]),
        )
        .arg(
            Arg::with_name("TARGET_NAME")
                .help("Target service name.")
                .required_unless("all")
                .index(3),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .required_unless("TARGET_NAME")
                .help("Search all upstream or server or router."),
        )
        .arg(
            Arg::with_name("origin")
                .short("o")
                .long("origin")
                .required_if("OPTION", "replace")
                .takes_value(true)
                .help("Its required when the target bust be replace."),
        )
        .arg(
            Arg::with_name("dest")
                .short("d")
                .long("dist")
                .takes_value(true)
                .required_if("OPTION", "replace")
                .help("Its required when the target bust be replace."),
        )
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .takes_value(true)
                .required_if("OPTION", "search")
                .help("Query string."),
        )
        // .arg(
        //     Arg::with_name("upstream_target")
        //         .short("t")
        //         .long("upstream_target")
        //         .takes_value(true)
        //         .required_if("OPTION", "remove")
        //         .help("Remove upstream target."),
        // )
        .get_matches();


    let target = matches.value_of("TARGET").unwrap();
    let option = matches.value_of("OPTION").unwrap();
    let target_name = matches.value_of("TARGET_NAME").unwrap_or(&"");

    let host = matches.value_of("host").unwrap();
    let schema = matches.value_of("schema").unwrap();

    //replace
    let is_all = matches.is_present("all");
    let origin = matches.value_of("origin").unwrap_or(&"");
    let dest = matches.value_of("dest").unwrap_or(&"");
    //search
    let query_string = matches.value_of("query").unwrap_or(&"");
    //remove
    // let upstream_target = matches.value_of("upstream_target").unwrap_or(&"");

    return CommandArgs {
        host: String::from(host),
        schema: String::from(schema),
        target: String::from(target),
        option: String::from(option),
        target_name: String::from(target_name),
        is_all: is_all,
        origin: String::from(origin),
        dest: String::from(dest),
        query_string: String::from(query_string),
    };
}
