use google_calendar::Client;

pub struct GoogleCalendar {
    client: Client,
}

impl GoogleCalendar {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        // Passing empty strings for token and refresh_token for now
        let client = Client::new(client_id, client_secret, redirect_uri , "", "");
        GoogleCalendar { client }
    }

    pub fn load_calendars(&self) -> Vec<String> {
        // Placeholder implementation
        vec!["Calendar 1".to_string(), "Calendar 2".to_string()]
    }
}