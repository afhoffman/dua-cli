use std::collections::HashSet;
use std::path::PathBuf;

use dua::traverse::{BackgroundTraversal, TraversalStats};
use dua::WalkOptions;

use crate::interactive::widgets::Column;

use super::{navigation::Navigation, EntryDataBundle, SortMode};

#[derive(Default, Copy, Clone, PartialEq)]
pub enum FocussedPane {
    #[default]
    Main,
    Help,
    Mark,
    Glob,
}

#[derive(Default)]
pub struct Cursor {
    pub show: bool,
    pub x: u16,
    pub y: u16,
}

pub(crate) struct FilesystemScan {
    pub active_traversal: BackgroundTraversal,
    /// The selected item prior to starting the traversal, if available, based on its name or index into [`AppState::entries`].
    pub previous_selection: Option<(PathBuf, usize)>,
}

pub struct AppState {
    pub navigation: Navigation,
    pub glob_navigation: Option<Navigation>,
    pub entries: Vec<EntryDataBundle>,
    pub sorting: SortMode,
    pub show_columns: HashSet<Column>,
    pub message: Option<String>,
    pub focussed: FocussedPane,
    pub received_events: bool,
    pub scan: Option<FilesystemScan>,
    pub stats: TraversalStats,
    pub walk_options: WalkOptions,
}

impl AppState {
    pub fn new(walk_options: WalkOptions) -> Self {
        AppState {
            navigation: Default::default(),
            glob_navigation: None,
            entries: vec![],
            sorting: Default::default(),
            show_columns: Default::default(),
            message: None,
            focussed: Default::default(),
            received_events: false,
            scan: None,
            stats: TraversalStats::default(),
            walk_options,
        }
    }
}
