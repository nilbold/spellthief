use anyhow::Result;

mod component;
mod game;
mod math;
mod util;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = env!("GIT_HASH");

pub fn run() -> Result<()> {
    println!("spellthief v{} [{}]", VERSION, GIT_HASH);

    game::main_loop()
}
