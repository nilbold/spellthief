use anyhow::Result;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &'static str = env!("GIT_HASH");

pub fn run() -> Result<()> {
    println!("spellthief v{} [{}]", VERSION, GIT_HASH);

    Ok(())
}
