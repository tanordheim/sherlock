use rusqlite::Connection; 
use std::{collections::HashMap, env, fs, path::{Path, PathBuf}};
use chrono::{NaiveDateTime, TimeZone, Local};

use crate::CONFIG;

#[derive(Clone, Debug)]
pub struct TeamsEvent {
    pub title: String,
    pub meeting_url: String,
    pub time: String,
}

#[derive(Clone, Debug)]
pub struct EventLauncher {
    pub event: Option<TeamsEvent>
}



impl EventLauncher {
    pub fn get_event(date:&str, event_start:&str, event_end:&str)->Option<TeamsEvent>{
        let calendar_client = CONFIG.get()?.default_apps.calendar_client.as_ref();
        match calendar_client {
            "thunderbird" => {
                let thunderbird_manager = ThunderBirdEventManager::new(); 
                if let Some(path) = &thunderbird_manager.database_path {
                    match Connection::open(Path::new(path)) {
                        Ok(conn) => {
                            let meetings = thunderbird_manager.get_all_teams_events(&conn);
                            let ids = meetings.keys().map(|i| format!("'{}'", i)).collect::<Vec<String>>().join(", ");
                            if let Some((id, title, time)) = thunderbird_manager.get_teams_event_by_time(&conn, ids,date, event_start, event_end){
                                if let Some((meeting_url, _)) = meetings.get(&id){
                                    return Some(TeamsEvent {
                                        title,
                                        meeting_url: meeting_url.to_string(),
                                        time
                                    });
                                } 
                            }
                        }
                        Err(_) => return None
                    }
                }
            },
            _ => {}
            
        }
        return None
    }
}

struct ThunderBirdEventManager{
    database_path: Option<PathBuf>
    
}
impl ThunderBirdEventManager {
    pub fn new()->Self{
        let home = env::var("HOME").ok().map(PathBuf::from);
        if let Some(home) = home {
            let thunderbird_dir = home.join(".thunderbird");
            match fs::read_dir(&thunderbird_dir){
                Ok(entries) => {
                    for entry in entries.flatten(){
                        let path = entry.path();
                        if path.is_dir() && path.file_name()
                            .and_then(|n| n.to_str())
                                .map(|n| n.ends_with(".default-release"))
                                .unwrap_or(false)
                        {
                            let database_path = Some(path.join("calendar-data").join("cache.sqlite"));
                            return ThunderBirdEventManager {database_path};
                        }
                    }
                },
                Err(_) => return ThunderBirdEventManager { database_path: None }
            } 
        }
        ThunderBirdEventManager { database_path: None }
    }

    pub fn get_all_teams_events(&self, conn: &Connection) -> HashMap<String, (String, Option<i64>)> {
        let query = "
        SELECT item_id, recurrence_id, value
        FROM cal_properties 
        WHERE key = 'X-MICROSOFT-SKYPETEAMSMEETINGURL';
        ";

        let mut events: HashMap<String, (String, Option<i64>)> = HashMap::new();

        if let Ok(mut stmt) = conn.prepare(query) {
            let event_iter = stmt.query_map([], |row| {
                let id: String = row.get(0)?;
                let recurrence_id: Option<i64> = row.get(1).ok();
                let meeting_url: String = row.get(2)?;

                Ok((id, meeting_url, recurrence_id))
            });

            if let Ok(rows) = event_iter {
                for row in rows.flatten() {
                    events.insert(row.0, (row.1, row.2));
                }
            }
        }

        events
    }

    pub fn get_teams_event_by_time(&self, conn: &Connection, ids: String, date:&str, event_start:&str, event_end:&str)->Option<(String, String, String)>{
        let query  = if !ids.is_empty(){
            format!("
                SELECT id, title, event_start
                FROM cal_events 
                WHERE id IN ({})
                AND event_start BETWEEN strftime('%s', '{}', '{}') * 1000000 AND strftime('%s', '{}', '{}') * 1000000
                ORDER BY event_start;
                ", ids, date, event_start, date, event_end)
        } else {
            return None;
        };
        //AND event_start BETWEEN strftime('%s', '2025-04-08') * 1000000 AND strftime('%s', '2025-04-08', 'start of day', '+1 day') * 1000000

        if let Ok(mut stmt) = conn.prepare(&query) {
            let event_iter = stmt.query_map([], |row| {
                let id: String = row.get(0)?;
                let title: String = row.get(1)?;
                let start_time: i64 = row.get(2)?;
                Ok((id, title, start_time))
            });

            if let Ok(rows) = event_iter {
                if let Some(row) = rows.flatten().nth(0){
                    let timestamp = row.2 / 1_000_000;
                    let native_datetime = NaiveDateTime::from_timestamp(timestamp, 0);
                    let local_datetime = Local.from_utc_datetime(&native_datetime);
                    let formatted = local_datetime.format("%H:%M").to_string();
                    return Some((row.0, row.1, formatted))
                }
            }
        }
        return None
    }
}
