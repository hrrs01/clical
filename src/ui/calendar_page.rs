use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{App, Id},
    calendar::Calendar,
    configuration::KeyBindings,
    day_of_week::DayOfWeek,
    key,
    ui::Page,
};
use std::{cell::RefCell, rc::Rc};

pub struct CalendarPage {
    pub app: Rc<RefCell<App>>,
    pub auth_url: Option<String>,
}

impl CalendarPage {
    pub fn new(app: Rc<RefCell<App>>) -> CalendarPage {
        let auth_url = if app.borrow().settings.google_calendar.enabled {
            // If Google Calendar is enabled but not set up, set it up
            app.borrow().calendar.get_google_auth_url()
        } else {
            None
        };
        CalendarPage { app, auth_url }
    }


    pub fn get_primary_color(&self) -> Color {
        self.app.borrow().settings.colors.primary_color
    }

    pub fn get_secondary_color(&self) -> Color {
        self.app.borrow().settings.colors.secondary_color
    }

    pub fn get_accent_color(&self) -> Color {
        self.app.borrow().settings.colors.accent_color
    }
}

impl Page for CalendarPage {
    fn ui(&self, f: &mut Frame, area: Rect, focused: bool) {
        let overall_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Fill(1), Constraint::Length(3)].as_ref())
            .split(area);

        if let Some(auth_url) = &self.auth_url {
            let paragraph = Paragraph::new(format!("Google Calendar Auth URL:\n{}", auth_url)).wrap(Wrap { trim: false })
                .style(Style::default().fg(self.get_accent_color()));
            f.render_widget(paragraph, overall_chunks[2]);
        }

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
        f.render_widget(block, overall_chunks[0]);

        for (weekday, i) in self.app.borrow().calendar.get_weekdays().iter().zip(0..7) {
            let weekday_block = Block::default()
                .borders(Borders::ALL)
                .title(weekday.to_string().set_style(Style::reset()))
                .border_style(self.get_accent_color())
                .border_type(border_type);
            f.render_widget(weekday_block, calendar_chunks[i]);
        }
    }
}
