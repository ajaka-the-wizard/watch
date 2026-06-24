use anyhow::Result;

use configs::{Args, Configs};
mod configs;
mod inner;

pub fn watch() -> Result<()> {
    Args::check()?;
    let configs = Configs::load_file()?;
    Ok(inner::Engine::init(configs).start()?)
}
