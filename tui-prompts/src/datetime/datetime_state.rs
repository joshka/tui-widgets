use chrono::NaiveDate;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::borrow::Cow;

use crate::prelude::*;
use chrono::{DateTime, Days, Local, Months, NaiveDateTime, TimeDelta};

use strum::FromRepr;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DateTimeState<'a> {
    status: Status,
    position: usize,
    cursor: (u16, u16),
    elements: Vec<NumericChildState<'a>>,
}

#[derive(FromRepr)]
enum DateTimePosition {
    Year,
    Month,
    Day,
    Hour,
    Minute,
}

impl<'a> DateTimeState<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            status: Status::Pending,
            position: 0,
            cursor: (0, 0),
            elements: vec![
                NumericChildState::new(4, 'Y', Cow::from("")),
                NumericChildState::new(2, 'M', Cow::from("")),
                NumericChildState::new(2, 'D', Cow::from("")),
                NumericChildState::new(2, 'H', Cow::from("")),
                NumericChildState::new(2, 'M', Cow::from("")),
            ],
        }
    }

    #[must_use]
    pub const fn with_status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.status.is_finished()
    }

    fn increment_element(&mut self, n: u32) {
        self.increment_element_at_position(self.position(), n);
    }

    fn decrement_element(&mut self, n: u32) {
        self.decrement_element_at_position(self.position(), n);
    }

    fn increment_element_at_position(&mut self, i: usize, n: u32) {
        match self.output_value() {
            Err(_) => (),
            Ok(t) => match DateTimePosition::from_repr(i).expect("invalid position") {
                DateTimePosition::Year => {
                    let n = Months::new(n * 12);
                    let new_time = t.checked_add_months(n).expect("year add failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Month => {
                    let n = Months::new(n);
                    let new_time = t.checked_add_months(n).expect("month add failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Day => {
                    let n = Days::new(n.into());
                    let new_time = t.checked_add_days(n).expect("day add failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Hour => {
                    let n = TimeDelta::hours(n.into());
                    let new_time = t.checked_add_signed(n).expect("hour add failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Minute => {
                    let n = TimeDelta::minutes(n.into());
                    let new_time = t.checked_add_signed(n).expect("minute add failed");
                    self.with_value(new_time);
                }
            },
        }
    }

    fn decrement_element_at_position(&mut self, i: usize, n: u32) {
        match self.output_value() {
            Err(_) => (),
            Ok(t) => match DateTimePosition::from_repr(i).expect("invalid position") {
                DateTimePosition::Year => {
                    let n = Months::new(n * 12);
                    let new_time = t.checked_sub_months(n).expect("year sub failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Month => {
                    let n = Months::new(n);
                    let new_time = t.checked_sub_months(n).expect("month sub failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Day => {
                    let n = Days::new(n.into());
                    let new_time = t.checked_sub_days(n).expect("day sub failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Hour => {
                    let n = TimeDelta::hours(n.into());
                    let new_time = t.checked_sub_signed(n).expect("hour sub failed");
                    self.with_value(new_time);
                }
                DateTimePosition::Minute => {
                    let n = TimeDelta::minutes(n.into());
                    let new_time = t.checked_sub_signed(n).expect("minute sub failed");
                    self.with_value(new_time);
                }
            },
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Up, KeyModifiers::CONTROL) => self.increment_element(5),
            (KeyCode::Up, _) => self.increment_element(1),
            (KeyCode::Down, KeyModifiers::CONTROL) => self.decrement_element(5),
            (KeyCode::Down, _) => self.decrement_element(1),
            (KeyCode::Left, KeyModifiers::CONTROL) | (KeyCode::BackTab, KeyModifiers::SHIFT) => {
                self.jump_left()
            }
            (KeyCode::Left, _) | (KeyCode::Char('b'), KeyModifiers::CONTROL) => self.move_left(),
            (KeyCode::Right, KeyModifiers::CONTROL) | (KeyCode::Tab, KeyModifiers::NONE) => {
                self.jump_right()
            }
            (KeyCode::Right, _) | (KeyCode::Char('f'), KeyModifiers::CONTROL) => self.move_right(),
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => self.push(c),
            _ => self.current_element_mut().handle_key_event(key_event),
        }
    }

    pub fn time_now(&mut self) {
        let local_datetime: DateTime<Local> = Local::now();
        self.with_value(local_datetime.naive_local());
    }
}

impl CursorControl for DateTimeState<'_> {
    fn cursor(&self) -> (u16, u16) {
        self.cursor
    }

    fn cursor_mut(&mut self) -> &mut (u16, u16) {
        &mut self.cursor
    }
}

impl StateCommon for DateTimeState<'_> {
    fn status(&self) -> Status {
        self.status
    }

    fn status_mut(&mut self) -> &mut Status {
        &mut self.status
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.current_element_mut().focus
    }

    fn focus_state(&self) -> FocusState {
        self.current_element().focus
    }

    fn position(&self) -> usize {
        self.position
    }

    fn position_mut(&mut self) -> &mut usize {
        &mut self.position
    }

    fn is_valid_value(&self) -> bool {
        self.output_value().is_ok()
    }

    fn final_position(&self) -> usize {
        self.elements.len() - 1
    }
}

impl<'a> CompoundState<NumericChildState<'a>, NaiveDateTime> for DateTimeState<'a> {
    fn with_value(&mut self, val: NaiveDateTime) {
        // formats described in https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        let date_formats = ["%Y", "%m", "%d", "%H", "%M"];

        self.elements_mut()
            .iter_mut()
            .zip(date_formats.iter())
            .for_each(|(el, dt)| {
                *el.value_mut() = val.format(dt).to_string();
                *el.position_mut() = el.final_position();
            });
    }

    fn current_element(&self) -> &NumericChildState<'a> {
        let i = self.position();
        &self.elements[i]
    }

    fn current_element_mut(&mut self) -> &mut NumericChildState<'a> {
        let i = self.position();
        &mut self.elements[i]
    }

    fn elements(&self) -> Vec<NumericChildState<'a>> {
        self.elements.clone()
    }

    fn elements_mut(&mut self) -> &mut Vec<NumericChildState<'a>> {
        &mut self.elements
    }

    fn output_value_from_elements(&self, el: Vec<NumericChildState<'a>>) -> Result<NaiveDateTime, ()> {
        if !self.elements.iter().all(|x| x.is_valid_value()) {
            return Err(());
        }
        match NaiveDate::from_ymd_opt(
            el[0].as_numeric() as i32,
            el[1].as_numeric(),
            el[2].as_numeric(),
        ) {
            None => Err(()),
            Some(x) => match x.and_hms_opt(el[3].as_numeric(), el[4].as_numeric(), 0) {
                Some(x) => Ok(x),
                None => Err(()),
            },
        }
    }
}
