use clap::{App};

use crate::module::ModuleManager;

pub fn build_app<'a>() -> (App<'a>,ModuleManager<'a>){
    let mut app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));

    let module_manager = ModuleManager::new();
    for (_,m) in module_manager.modules.iter() {
        app = app.subcommand(m.app.clone());
    }

    (app,module_manager)
}
