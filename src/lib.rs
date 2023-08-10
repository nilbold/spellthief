use anyhow::Result;

pub mod component;
pub mod game;
pub mod math;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = env!("GIT_HASH");

pub fn run() -> Result<()> {
    println!("spellthief v{} [{}]", VERSION, GIT_HASH);

    game::main_loop()
}
