use std::time::{Duration, Instant};

use tui::widgets::ListState;

const KEYBOARD_ACTION_DELAY: u64 = 150;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub enhanced_graphics: bool,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub action_delay: Instant,
    pub display_debugger: bool,
}

impl<'a> App<'a> {
    // Initialize app state
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tasks", "Timers"]),
            enhanced_graphics,
            action_delay: Instant::now(),
            display_debugger: false,
        }
    }

    // Soft limit, as keyboard actions are handled too fast and keyboard actions register multiple times
    fn is_actionable_item_delay_finished(&mut self) -> bool {
        self.action_delay.elapsed() > Duration::from_millis(KEYBOARD_ACTION_DELAY)
    }

    fn reset_actionable_item_delay(&mut self) {
        self.action_delay = Instant::now();
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

    pub fn on_right(&mut self) {
        if self.is_actionable_item_delay_finished() {
            self.tabs.next();
            self.reset_actionable_item_delay();
        }
    }

    pub fn on_left(&mut self) {
        if self.is_actionable_item_delay_finished() {
            self.tabs.previous();
            self.reset_actionable_item_delay();
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'd' => {
                self.display_debugger = true;
            }
            's' => {
                self.display_debugger = false;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Do some tick based logic
    }
}
