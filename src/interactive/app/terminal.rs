use std::path::PathBuf;

use anyhow::Result;
use crossbeam::channel::Receiver;
use crosstermion::input::Event;
use dua::{
    traverse::{Traversal, TraversalStats},
    ByteFormat, WalkOptions, WalkResult,
};
use tui::prelude::Backend;
use tui_react::Terminal;

use crate::interactive::widgets::MainWindow;

use super::{sorted_entries, state::AppState, DisplayOptions};

/// State and methods representing the interactive disk usage analyser for the terminal
pub struct TerminalApp {
    pub traversal: Traversal,
    pub stats: TraversalStats,
    pub display: DisplayOptions,
    pub state: AppState,
    pub window: MainWindow,
}

impl TerminalApp {
    pub fn initialize<B>(
        terminal: &mut Terminal<B>,
        walk_options: WalkOptions,
        byte_format: ByteFormat,
    ) -> Result<TerminalApp>
    where
        B: Backend,
    {
        terminal.hide_cursor()?;
        terminal.clear()?;

        let display = DisplayOptions::new(byte_format);
        let window = MainWindow::default();

        let mut state = AppState::new(walk_options);
        let traversal = Traversal::new();
        let stats = TraversalStats::default();

        state.navigation_mut().view_root = traversal.root_index;
        state.entries = sorted_entries(
            &traversal.tree,
            state.navigation().view_root,
            state.sorting,
            state.glob_root(),
        );
        state.navigation_mut().selected = state.entries.first().map(|b| b.index);

        let app = TerminalApp {
            state,
            display,
            traversal,
            stats,
            window,
        };
        Ok(app)
    }

    pub fn traverse(&mut self, input: Vec<PathBuf>) -> Result<()> {
        self.state.traverse(&self.traversal, input)?;
        Ok(())
    }

    pub fn process_events<B>(
        &mut self,
        terminal: &mut Terminal<B>,
        events: Receiver<Event>,
    ) -> Result<WalkResult>
    where
        B: Backend,
    {
        self.state.process_events(
            &mut self.window,
            &mut self.traversal,
            &mut self.display,
            terminal,
            events,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::TerminalApp;

    impl TerminalApp {
        pub fn run_until_traversed<B>(
            &mut self,
            terminal: &mut Terminal<B>,
            events: Receiver<Event>,
        ) -> Result<WalkResult>
        where
            B: Backend,
        {
            while self.state.scan.is_some() {
                if let Some(res) = self.state.process_event(
                    &mut self.window,
                    &mut self.traversal,
                    &mut self.display,
                    terminal,
                    &events,
                )? {
                    return Ok(res);
                }
            }
            Ok(WalkResult {
                num_errors: self.stats.io_errors,
            })
        }
    }
}
