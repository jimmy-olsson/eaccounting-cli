use clap::{App, Arg, Command};

mod admintool_api;
mod featureflag;

fn init_cli() -> clap::App<'static> {
    let app = App::new("ea")
        .version("0.1.0")
        .about("Tool for managing eaccounting")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("ff")
                .about("Manage feature flags")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("list")
                        .short('l')
                        .long("list")
                        .help("List feature flags")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("enable")
                        .short('e')
                        .long("enable")
                        .help("Enable and disable feature flags")
                        .takes_value(true),
                ),
        );
    return app;
}

fn main() {
    let app = init_cli();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("ff", sub_matches)) => {
            if sub_matches.is_present("list") {
                featureflag::handle_list();
            }
            if sub_matches.is_present("enable") {
                let a = sub_matches.get_one::<String>("enable").expect("required");
                featureflag::handle_enable(a);
            }
        }
        _ => {
            unreachable!()
        }
    }

    return ();
}
