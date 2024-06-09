use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppListResponse {
    pub applist: AppList,
}

#[derive(Deserialize)]
pub struct AppList {
    pub apps: Vec<App>,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct App {
    pub appid: u32,
    pub name: String,
}

const APP_LIST_URL: &str = "http://api.steampowered.com/ISteamApps/GetAppList/v0002/?l=english";

pub fn get_app_list() -> Vec<App> {
    // TODO make this no longer blocking
    let response_result = reqwest::blocking::get(APP_LIST_URL);
    match response_result {
        Ok(response) => {
            let app_list_response_result: Result<AppListResponse, reqwest::Error> = response.json();
            match app_list_response_result {
                Ok(app_list_response) => app_list_response.applist.apps,
                Err(_) => vec![],
            }
        }
        Err(_) => vec![],
    }
}

pub fn get_app_id(game_name: String, app_list: Vec<App>) -> Result<u32, String> {
    for app in app_list {
        if game_name == app.name {
            return Ok(app.appid);
        }
    }
    Err(format!("No game found matching {game_name}"))
}
