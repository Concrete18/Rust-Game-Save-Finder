#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppListResponse {
    pub applist: AppList,
}

#[derive(Deserialize, Debug)]
pub struct AppList {
    pub apps: Vec<App>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct App {
    appid: u32,
    name: String,
}

const APP_LIST_URL: &str = "http://api.steampowered.com/ISteamApps/GetAppList/v0002/?l=english";

pub fn get_app_list() -> Vec<App> {
    let response_result = reqwest::blocking::get(APP_LIST_URL);

    match response_result {
        Ok(response) => {
            let app_list_response_result: Result<AppListResponse, reqwest::Error> = response.json();
            match app_list_response_result {
                Ok(app_list_response) => app_list_response.applist.apps,
                Err(_) => vec![], // Return an empty vector if deserialization fails
            }
        }
        Err(_) => vec![], // Return an empty vector if the request fails
    }
}

pub fn get_app_id(game_name: String, app_list: Vec<App>) -> Result<u32, String> {
    for app in app_list {
        // TODO use better matching system
        if game_name == app.name {
            println!("Found {} for {}", app.appid, game_name);
            return Ok(app.appid);
        }
    }
    Err("No game found matching {game_name}".to_string()) // TODO fix this
}

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn test_get_app_list() {
        let app_list: Vec<App> = get_app_list();

        let half_life: App = App {
            appid: 70,
            name: "Half-Life".to_string(),
        };

        assert!(app_list.contains(&half_life));
    }

    #[test]
    fn test_get_app_id() {
        // create app_list
        let game1: App = App {
            appid: 1234,
            name: "Test Game 1".to_string(),
        };

        let game2: App = App {
            appid: 4321,
            name: "Test Game 2".to_string(),
        };

        let app_list = vec![game1, game2];

        // test
        let game_name: String = "Test Game 2".to_string();
        let app_id: Result<u32, String> = get_app_id(game_name, app_list);
        let correct_app_id: u32 = 4321;
        assert_eq!(app_id.unwrap(), correct_app_id);
    }

    #[test]
    fn test_get() {
        let game_name: String = "Half-Life".to_string();
        let app_list: Vec<App> = get_app_list();

        let app_id: Result<u32, String> = get_app_id(game_name, app_list);
        let correct_app_id: u32 = 70;
        assert_eq!(app_id.unwrap(), correct_app_id);
    }
}
