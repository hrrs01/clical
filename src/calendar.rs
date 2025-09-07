use crate::google::GoogleCalendar;
use crate::day_of_week::DayOfWeek;


pub struct Calendar {
    // Calendar related fields and methods
    google_calendar: Option<GoogleCalendar>,
    monday_start: bool, // Whether the week starts on Monday, or Sunday
}

impl Calendar {
    pub fn new(google_calendar: Option<GoogleCalendar>, monday_start: bool) -> Self {
        Calendar { google_calendar, monday_start }
    }

    pub fn get_weekdays(&self) -> Vec<DayOfWeek> {
        if self.monday_start {
            vec![DayOfWeek::Monday, DayOfWeek::Tuesday, DayOfWeek::Wednesday, DayOfWeek::Thursday, DayOfWeek::Friday, DayOfWeek::Saturday, DayOfWeek::Sunday]
        } else {
            vec![DayOfWeek::Sunday, DayOfWeek::Monday, DayOfWeek::Tuesday, DayOfWeek::Wednesday, DayOfWeek::Thursday, DayOfWeek::Friday, DayOfWeek::Saturday]
        }
    }

    pub fn get_events(&self) -> Vec<String> {
        // Returns a 2D vector representing the weeks in the current month
        // Each inner vector represents a week, with Option<u32> for each day (None for empty days)
        vec![]
    }

    pub fn load_calendars(&self) -> Vec<String> {
        if let Some(google_calendar) = &self.google_calendar {
            google_calendar.load_calendars()
        } else {
            vec!["No Google Calendar Configured".to_string(), "Please either set up Google Calendar".to_string(), "or use a different calendar backend (e.g. ical).".to_string()]
        }
    }
}