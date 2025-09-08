use google_calendar::{types::CalendarListEntry, Client};

pub struct GoogleCalendar {
    client: Client,
    loaded: bool,
}

impl GoogleCalendar {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        // Passing empty strings for token and refresh_token for now
        let client = Client::new(client_id, client_secret, redirect_uri , "", "");
        GoogleCalendar { client, loaded: false }
    }

    pub async fn load_calendars(&mut self) {
        // Placeholder implementation
        let entries = self.client.calendar_list().list_all(google_calendar::types::MinAccessRole::Reader, false, false).await;
        match entries {
            Ok(calendars) => {
                for calendar in calendars.body.iter() {
                    println!("Loaded calendar: {}", calendar.summary);
                    self.loaded = true;
                }
            }
            Err(e) => {
                eprintln!("Error loading calendars: {}", e);
            }
        }

    }

    pub fn get_user_consent_url(&self) -> String {
        let user_consent_url = self.client.user_consent_url(&["https://www.googleapis.com/auth/calendar.readonly".to_string()]);
        user_consent_url
    }

    pub fn get_calendar_list(&self) -> Vec<String> {
        // Placeholder implementation
        if(self.loaded) {
            vec!["Sample Calendar 1".to_string(), "Sample Calendar 2".to_string()]
        } else {
            vec!["No Calendars Loaded".to_string(), "Please load calendars first.".to_string(), self.get_user_consent_url()]
        }
    }
}