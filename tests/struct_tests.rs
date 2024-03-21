#[cfg(test)]
mod tests {
    use steam_api_wrapper::services::get_recently_played_games::{
        Game, Games, RecentlyPlayedSummary,
    };

    use super::*;

    #[test]
    fn test_game_struct_construction() {
        let game = Game {
            app_id: 123,
            name: "Test Game".to_string(),
            recently_played: 60,
            overall_plat_time: 600,
            img_icon_url: "test_icon_url".to_string(),
            img_logo_url: Some("test_logo_url".to_string()),
        };

        assert_eq!(game.app_id, 123);
        assert_eq!(game.name, "Test Game");
        assert_eq!(game.recently_played, 60);
        assert_eq!(game.overall_plat_time, 600);
        assert_eq!(game.img_icon_url, "test_icon_url");
        assert_eq!(game.img_logo_url.unwrap(), "test_logo_url");
    }

    #[test]
    fn test_games_struct_construction() {
        let game = Game {
            app_id: 123,
            name: "Test Game".to_string(),
            recently_played: 60,
            overall_plat_time: 600,
            img_icon_url: "test_icon_url".to_string(),
            img_logo_url: Some("test_logo_url".to_string()),
        };

        let games = Games {
            total_count: 1,
            games: vec![game],
        };

        assert_eq!(games.total_count, 1);
        assert_eq!(games.games.len(), 1);
        assert_eq!(games.games[0].name, "Test Game");
    }

    #[test]
    fn test_recently_played_summary_struct_construction() {
        let game = Game {
            app_id: 123,
            name: "Test Game".to_string(),
            recently_played: 60,
            overall_plat_time: 600,
            img_icon_url: "test_icon_url".to_string(),
            img_logo_url: Some("test_logo_url".to_string()),
        };

        let games = Games {
            total_count: 1,
            games: vec![game],
        };

        let summary = RecentlyPlayedSummary { response: games };

        assert_eq!(summary.response.total_count, 1);
        assert_eq!(summary.response.games.len(), 1);
        assert_eq!(summary.response.games[0].name, "Test Game");
    }
}
