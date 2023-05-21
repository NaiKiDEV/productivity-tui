use std::{
    ops::AddAssign,
    time::{Duration, Instant},
};

use chrono::{DateTime, Local};
use crossterm::event::KeyCode;

use crate::structures::stateful_list::StatefulList;

pub struct Timer {
    pub title: String,
    pub is_active: bool,
    pub time_active: Duration,
    pub time_created: DateTime<Local>,
}

impl Timer {
    pub fn default() -> Self {
        Self {
            title: String::from(""),
            time_created: Local::now(),
            is_active: false,
            time_active: Duration::from_millis(0),
        }
    }

    pub fn new(title: String) -> Self {
        Self {
            title,
            time_created: Local::now(),
            is_active: false,
            time_active: Duration::from_millis(0),
        }
    }
}

pub struct TimerState {
    pub timers: StatefulList<Timer>,

    pub new_timer: Timer,

    pub new_timer_popup_enabled: bool,

    pub last_tick: Instant,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            timers: StatefulList::with_items(vec![]),
            new_timer: Timer::default(),
            new_timer_popup_enabled: false,
            last_tick: Instant::now(),
        }
    }

    fn delete_selected_timer(&mut self) {
        self.timers.delete_current();
    }

    fn open_create_popup(&mut self) {
        self.new_timer_popup_enabled = true;
    }

    fn close_create_popup(&mut self) {
        self.new_timer_popup_enabled = false;
    }

    fn create_new_timer(&mut self) {
        let new_timer = Timer::new(self.new_timer.title.to_owned());
        self.timers.items.push(new_timer);
        self.new_timer = Timer::default();
    }

    pub fn on_keycode(&mut self, key: KeyCode) -> bool {
        if self.new_timer_popup_enabled {
            self.on_popup_keycode(key);

            return true;
        }

        match key {
            KeyCode::Up => self.timers.previous(),
            KeyCode::Down => self.timers.next(),

            KeyCode::Enter => {
                let current_selection = self.timers.state.selected();
                match current_selection {
                    Some(selection) => {
                        self.timers.items[selection].is_active =
                            !self.timers.items[selection].is_active;
                    }
                    None => {}
                }
            }

            KeyCode::Char(c) => match c {
                'd' => {
                    self.delete_selected_timer();
                }
                'n' => {
                    self.open_create_popup();
                    self.new_timer = Timer::default();
                }
                _ => {}
            },
            _ => {}
        };

        false
    }

    fn on_popup_keycode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.new_timer.title.push(c);
            }
            KeyCode::Backspace => {
                self.new_timer.title.pop();
            }
            KeyCode::Esc => {
                self.close_create_popup();
            }
            KeyCode::Enter => {
                self.create_new_timer();
                self.close_create_popup();
            }
            _ => {}
        };
    }

    pub fn on_tick(&mut self) {
        if self.last_tick.elapsed() > Duration::from_millis(1000) {
            for timer in &mut self.timers.items.iter_mut() {
                if timer.is_active {
                    timer.time_active.add_assign(self.last_tick.elapsed());
                }
            }

            self.last_tick = Instant::now();
        }
    }
}
