use std::collections::HashMap;

use crate::{
    calendar::{self, Calendar},
    configuration::{get_db_file, Settings},
    task::Task,
    utils,
};

pub type Id = usize;

pub struct App {
    pub tasks: HashMap<Id, Task>,
    pub calendar: Calendar,
    pub settings: Settings,
    pub current_id: usize,
}

impl App {
    pub fn new(settings: Settings) -> App {
        let tasks: HashMap<Id, Task> = utils::load_tasks(get_db_file());
        let mut google_calendar = None;
        match &settings.google_calendar {
            Some(config) => {
                google_calendar = Some(crate::google::GoogleCalendar::new(
                    config.client_id.clone(),
                    config.client_secret.clone(),
                    config.redirect_uri.clone(),
                ));
            }
            _ => {}
        }
        let calendar = Calendar::new(google_calendar, settings.monday_start);
        let current_id = tasks.iter().map(|(&k, _)| k).max().unwrap_or(0);
        App {
            tasks,
            calendar,
            settings,
            current_id,
        }
    }

    pub fn get_task(&self, id: Id) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn save_state(&mut self) {
        utils::save_tasks(get_db_file(), self);
    }

    pub fn add_task(&mut self, mut t: Task) -> Id {
        let new_id = match t.id {
            Some(id) => {
                self.tasks.insert(id, t);
                self.save_state();
                id
            }
            None => {
                let new_id = self.get_next_id();
                t.id = Some(new_id);
                self.tasks.insert(new_id, t);
                new_id
            }
        };
        self.save_state();
        new_id
    }

    pub fn delete_task(&mut self, id: usize) -> Option<Id> {
        self.tasks.remove(&id)?;
        self.save_state();
        Some(id)
    }

    pub fn set_complete(&mut self, id: usize, complete: bool) -> Option<Id> {
        let new_task_id = if complete {
            let possible_new_task = self.tasks.get_mut(&id)?.set_complete();
            if let Some(possible_new_task) = possible_new_task {
                self.delete_task(id);
                self.add_task(possible_new_task)
            } else {
                id
            }
        } else {
            self.tasks.get_mut(&id)?.set_incomplete();
            id
        };

        self.save_state();
        Some(new_task_id)
    }

    pub fn toggle_complete_task(&mut self, id: usize) -> Option<Id> {
        let complete = self.tasks.get(&id)?.complete;
        self.set_complete(id, !complete)
    }

    fn get_next_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }
}
