mod app;
mod crossterm;
mod ui;

use crate::crossterm::run;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let forced_tick_rate = Duration::from_millis(250);
    run(forced_tick_rate, true)?;

    Ok(())
}
