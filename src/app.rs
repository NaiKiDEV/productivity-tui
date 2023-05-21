use crossterm::event::KeyCode;

use crate::features::{tasks::TaskState, timers::TimerState};

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

pub enum AppTab {
    Tasks,
    Timers,
}

pub fn get_menu_item_title(menu_item: AppTab) -> &'static str {
    match menu_item {
        AppTab::Tasks => "Tasks",
        AppTab::Timers => "Timers",
    }
}

pub struct App<'a> {
    // App state
    pub title: &'a str,
    pub tabs: TabsState<'a>,

    // Feature state definitions
    pub task_state: TaskState,
    pub timer_state: TimerState,

    pub should_quit: bool,

    // Internals
    pub display_debugger: bool,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,

            should_quit: false,

            tabs: TabsState::new(vec![
                get_menu_item_title(AppTab::Tasks),
                get_menu_item_title(AppTab::Timers),
            ]),

            task_state: TaskState::new(),
            timer_state: TimerState::new(),

            enhanced_graphics,
            display_debugger: false,
        }
    }

    // TODO: Implement tab focusing or active state selection
    pub fn on_keycode(&mut self, key: KeyCode) {
        // TODO: Improve this logic?

        // Tab index == 0 is TaskList
        if self.tabs.index == 0 && self.task_state.on_keycode(key) {
            // Return when keyboard action is non-app interactive
            return;
        }
        // Tab index == 0 is Timers
        if self.tabs.index == 1 && self.timer_state.on_keycode(key) {
            // Return when keyboard action is non-app interactive
            return;
        }

        return match key {
            // Character handling
            KeyCode::Char(c) => match c {
                '1' => {
                    self.tabs.index = 0;
                }
                '2' => {
                    self.tabs.index = 1;
                }
                'q' => {
                    self.should_quit = true;
                }
                _ => {}
            },

            // Keyboard arrow actions
            KeyCode::Left => self.on_left(),
            KeyCode::Right => self.on_right(),

            KeyCode::Esc => {}
            _ => {}
        };
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_tick(&mut self) {
        self.timer_state.on_tick();
    }
}
