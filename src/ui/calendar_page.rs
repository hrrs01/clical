use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    app::{App, Id},
    configuration::KeyBindings,
    day_of_week::DayOfWeek,
    key,
    ui::Page,
};
use std::{cell::RefCell, rc::Rc};

pub struct CalendarPage {
    pub app: Rc<RefCell<App>>,
    pub weekdays: [DayOfWeek; 7],
}

impl CalendarPage {
    pub fn new(app: Rc<RefCell<App>>) -> CalendarPage {
        CalendarPage { app, weekdays: [
            DayOfWeek::Monday,
            DayOfWeek::Tuesday,
            DayOfWeek::Wednesday,
            DayOfWeek::Thursday,
            DayOfWeek::Friday,
            DayOfWeek::Saturday,
            DayOfWeek::Sunday,
        ] }
    }

    pub fn get_primary_color(&self) -> Color {
        self.app.borrow().settings.colors.primary_color
    }

    pub fn get_secondary_color(&self) -> Color {
        self.app.borrow().settings.colors.secondary_color
    }
}

impl Page for CalendarPage {
    fn ui(&self, f: &mut Frame, area: Rect, focused: bool) {
        let overall_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Fill(1)].as_ref())
            .split(area);

        let calendar_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                    Constraint::Ratio(1, 7),
                ]
                .as_ref(),
            )
            .split(overall_chunks[1]);
        // Draw border around area
        let border_style = match focused {
            true => Style::default().fg(self.get_primary_color()),
            false => Style::default(),
        };
        let border_type = match focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Calendar")
            .border_style(border_style)
            .border_type(border_type);
        f.render_widget(block, area);

        for (weekday, i) in [
            DayOfWeek::Monday,
            DayOfWeek::Tuesday,
            DayOfWeek::Wednesday,   
            DayOfWeek::Thursday,
            DayOfWeek::Friday,
            DayOfWeek::Saturday,
            DayOfWeek::Sunday,
        ]
        .iter()
        .zip(0..7)
        {
            let weekday_block = Block::default()
                .borders(Borders::ALL)
                .title(weekday.to_string())
                .border_style(border_style)
                .border_type(border_type);
            f.render_widget(weekday_block, calendar_chunks[i]);
        }
    }
}
