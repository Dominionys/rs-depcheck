use clap::Parser;
use depcheck_rs_cli::Args;
use depckeck_rs_core::checker::Checker;
use depckeck_rs_core::config::Config;
use depckeck_rs_core::package;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read package")]
    Package(#[from] package::Error),

    #[error("failed to serialize result")]
    Serialize(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    human_panic::setup_panic!();

    let Args {
        directory,
        ignore_bin_package,
        skip_missing,
        ignore_path,
        ignore_patterns,
        ignore_matches,
        verbose,
    } = Args::parse();

    env_logger::Builder::new()
        .filter_level(verbose.log_level_filter())
        .init();

    let mut config = Config::new(directory)
        .with_ignore_bin_package(ignore_bin_package)
        .with_skip_missing(skip_missing)
        .with_ignore_path(ignore_path);

    if let Some(ignore_patterns) = ignore_patterns {
        config = config.with_ignore_patterns(ignore_patterns);
    }

    if let Some(ignore_matches) = ignore_matches {
        config = config.with_ignore_matches(ignore_matches);
    }

    let result = Checker::new(config).check_package()?;

    println!("{:#?}", result);

    Ok(())
}
