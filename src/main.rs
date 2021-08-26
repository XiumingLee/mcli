mod app;
mod module;


fn main() {
    let (app,module_manager) = app::build_app();
    let mut app_clone = app.clone();

    let matches = app.get_matches();

    let subcommand_option = matches.subcommand();

    if let Some((name, matches)) = subcommand_option {
        module_manager.run(name, matches);
    } else {
        app_clone.print_help().unwrap_or(());
        println!();
    }

}
