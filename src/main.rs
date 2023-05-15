mod app;
mod crossterm;
mod ui;

use crate::crossterm::run;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    run(Duration::from_millis(250), true)?;
    Ok(())
}
