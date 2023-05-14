use std::time::{Duration, Instant};

use crossterm::event::KeyCode;

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

pub enum TopMenuItem {
    Tasks,
    Timers,
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,

    // TODO: refactor into separate state
    pub create_new_task_popup_enabled: bool,
    pub new_task_description: String,

    // Internals
    pub action_delay: Instant,
    pub display_debugger: bool,
    pub enhanced_graphics: bool,
    pub should_quit: bool,
}

pub fn get_menu_item_title(menu_item: TopMenuItem) -> &'static str {
    match menu_item {
        TopMenuItem::Tasks => "Tasks",
        TopMenuItem::Timers => "Timers",
    }
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec![
                get_menu_item_title(TopMenuItem::Tasks),
                get_menu_item_title(TopMenuItem::Timers),
            ]),
            enhanced_graphics,
            action_delay: Instant::now(),
            display_debugger: false,
            create_new_task_popup_enabled: false,
            new_task_description: String::new(),
        }
    }

    // FIXME: Hacky Soft limit, keyboard events are being double read
    fn is_actionable_item_delay_finished(&mut self) -> bool {
        self.action_delay.elapsed() > Duration::from_millis(KEYBOARD_ACTION_DELAY)
    }

    fn reset_actionable_item_delay(&mut self) {
        self.action_delay = Instant::now();
    }

    // TODO: KeyCode handling should be generalized so tabs and windows are actually handling
    // based on which of them is focused and in which state the app exists at that moment
    pub fn on_keycode(&mut self, key: KeyCode) {
        if self.create_new_task_popup_enabled {
            match key {
                KeyCode::Char(c) => {
                    // Double pushing..
                    self.new_task_description.push(c);
                }
                KeyCode::Backspace => {
                    self.new_task_description.pop();
                }
                KeyCode::Esc => {
                    self.create_new_task_popup_enabled = false;
                }
                KeyCode::Enter => {
                    // TODO: Implement Add to list
                    // * Rewrite structure on how this is being handled in state (Popup State)
                    // * Create KeyCode handler for the popup
                    if self.is_actionable_item_delay_finished() {
                        self.create_new_task_popup_enabled = false;
                        self.reset_actionable_item_delay();
                    }
                }
                _ => {}
            }
            // Suspend other action execution
            return;
        }
        match key {
            // Character handling
            KeyCode::Char(c) => self.on_key(c),

            // Keyboard arrow actions
            KeyCode::Left => self.on_left(),
            KeyCode::Up => self.on_up(),
            KeyCode::Right => self.on_right(),
            KeyCode::Down => self.on_down(),

            KeyCode::Esc => {}
            _ => {}
        }
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
                if self.is_actionable_item_delay_finished() {
                    self.display_debugger = !self.display_debugger;
                    self.reset_actionable_item_delay();
                }
            }
            'n' => {
                if self.is_actionable_item_delay_finished() {
                    self.create_new_task_popup_enabled = !self.create_new_task_popup_enabled;
                    self.reset_actionable_item_delay();
                }
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Do some tick based logic
    }
}
