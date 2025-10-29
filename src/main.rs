use anyhow::Result;
use blockchain::cli::Cli;

fn main() -> Result<()> {

    let cli = Cli::new();
    let _ = cli?.run();

    Ok(())

    // dbg!(&b);
}
