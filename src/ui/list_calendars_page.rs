use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Styled},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

use crate::{
    app::{App, Id},
    calendar::{self, Calendar},
    configuration::KeyBindings,
    day_of_week::DayOfWeek,
    key,
    ui::Page,
};
use std::{cell::RefCell, rc::Rc};

pub struct ListCalendarPage {
    pub app: Rc<RefCell<App>>,
    pub available_calendars: Vec<String>,
}

impl ListCalendarPage {
    pub fn new(app: Rc<RefCell<App>>) -> ListCalendarPage {
        let available_calendars: Vec<String> = app.borrow().calendar.load_calendars();
        ListCalendarPage {
            app,
            available_calendars,
        }
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

impl Page for ListCalendarPage {
    fn ui(&self, f: &mut Frame, area: Rect, focused: bool) {

        let overall_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(2), Constraint::Fill(1)].as_ref())
            .split(area);

        let calendar_list = Layout::default()
            .direction(Direction::Vertical)
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
            .title("Available Calendars")
            .border_style(border_style)
            .border_type(border_type);
        f.render_widget(block, area);

        for (i, calendar) in self.available_calendars.iter().enumerate() {
            let paragraph = Paragraph::new(format!(" {}. {}", i + 1, calendar));
            f.render_widget(paragraph, calendar_list[i]);
        }
    }
}
