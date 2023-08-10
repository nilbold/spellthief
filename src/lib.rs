use anyhow::Result;

mod component;
mod game;
mod math;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &'static str = env!("GIT_HASH");

pub fn run() -> Result<()> {
    println!("spellthief v{} [{}]", VERSION, GIT_HASH);

    game::main_loop()
}
