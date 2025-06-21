use anyhow::Result;
use clap::Parser;
use clap_derive::Parser;

use crate::{batch::Batch, question::Question};

pub mod batch;
pub mod question;
pub mod results;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    pub question: String,
}

fn main() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    let args = Args::try_parse()?;
    let question = Question::load(&args.question)?;

    let batch = Batch::load(&question)?;
    let results = batch.process(&question)?;

    log::info!("{results:?}");

    Ok(())
}
