use anyhow::Result;

fn main() -> Result<()> {
    watch::watch()?;
    Ok(())
}
