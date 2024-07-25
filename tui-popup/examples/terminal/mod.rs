use std::io::{stdout, Stdout};

use color_eyre::{config::HookBuilder, Result};
use ratatui::{
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::{CrosstermBackend, Terminal},
};

pub fn init() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    install_error_hooks()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    enable_raw_mode()?;
    Ok(terminal)
}

pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn install_error_hooks() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        panic_hook(panic_info);
    }));
    let eyre_hook = eyre_hook.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |error| {
        let _ = restore();
        eyre_hook(error)
    }))?;
    Ok(())
}
