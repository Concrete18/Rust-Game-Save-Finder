use crate::app_data::*;

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
        let game_name: String = "Street Fighter™ 6".to_string();
        let app_list: Vec<App> = get_app_list();

        let app_id: Result<u32, String> = get_app_id(game_name, app_list);
        let correct_app_id: u32 = 1364780;
        assert_eq!(app_id.unwrap(), correct_app_id);
    }
}
