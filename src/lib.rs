use anyhow::Result;

use configs::{Args, Configs};
mod configs;
mod inner;

pub fn watch() -> Result<()> {
    if !Args::check()? {
        let configs = Configs::load_file()?;
        return Ok(inner::Engine::init(configs).start()?);
    };
    Ok(())
}
