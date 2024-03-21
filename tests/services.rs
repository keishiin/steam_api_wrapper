#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::convert::TryInto;
    use steam_api_wrapper::Steam;

    const STEAM_ID: u64 = 76561198163350464;
    const TEST_APP_ID: u64 = 1245620;

    #[tokio::test]
    async fn test_get_player_summaries() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_player_summaries(STEAM_ID).await.unwrap();

        println!("{:?}", response);

        assert_eq!("76561198163350464", response[0].steam_id)
    }

    #[tokio::test]
    async fn test_get_player_summaries_err() {
        dotenv().ok();
        let steam = Steam::new("steam_api_key");
        let response = steam.get_player_summaries(12345).await;

        println!("{:?}", response);

        assert!(response.is_err())
    }

    #[tokio::test]
    async fn test_get_recently_played_games() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_recently_played_games(STEAM_ID).await.unwrap();

        println!("{:?}", response);

        assert_ne!(123434, response.total_count)
    }

    #[tokio::test]
    async fn test_get_recently_played_games_err() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_recently_played_games(23432432).await;

        println!("{:?}", response);

        assert!(response.is_err())
    }

    #[tokio::test]
    async fn test_get_owned_games() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_owned_games(STEAM_ID, true, false).await.unwrap();

        println!("{:?}", response);

        assert_eq!(233, response.game_count)
    }

    #[tokio::test]
    async fn test_get_owned_games_err() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_owned_games(1235435, true, false).await;

        println!("{:?}", response);

        assert!(response.is_err())
    }

    #[tokio::test]
    async fn test_get_player_achievements() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam
            .get_player_achievements(STEAM_ID, TEST_APP_ID)
            .await
            .unwrap();

        println!("{:?}", response);

        assert_eq!(Some("Profile is not public".to_string()), response.error)
    }

    #[tokio::test]
    async fn test_get_player_achievements_err() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_player_achievements(123453, 234324324).await;

        println!("{:?}", response);

        assert!(response.is_err())
    }

    #[tokio::test]
    async fn test_get_player_steam_level() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_steam_level(STEAM_ID).await.unwrap();

        println!("{:?}", response);

        assert_eq!(20, response.player_level)
    }

    #[tokio::test]
    async fn test_get_player_steam_level_err() {
        dotenv().ok();
        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_steam_level(123545435).await;

        println!("{:?}", response);

        assert!(response.is_err())
    }

    #[tokio::test]
    async fn test_get_schema_for_game() {
        dotenv().ok();

        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam
            .get_schema_for_game(TEST_APP_ID.try_into().unwrap())
            .await
            .unwrap();

        println!("{:?}", response);

        assert_eq!("ELDEN RING", response.game.game_name)
    }

    #[tokio::test]
    async fn test_get_schema_for_game_for_err() {
        dotenv().ok();

        let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
        let steam = Steam::new(steam_api_key.as_str());
        let response = steam.get_schema_for_game(123543543).await;

        println!("{:?}", response);

        assert!(response.is_err())
    }
}
