use anyhow::Result;

use log::info;

mod component;
mod embed;
mod game;
mod math;
mod render;
mod util;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = env!("GIT_HASH");

pub fn run() -> Result<()> {
    env_logger::init();
    info!("spellthief v{} [{}]", VERSION, GIT_HASH);

    game::main_loop()
}
