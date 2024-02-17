use dotenv::dotenv;
use steam_wrapper::Steam;

#[tokio::test]
async fn get_player_summaries() {
    dotenv().ok();
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
    let steam = Steam::new(steam_api_key.as_str());
    let response = steam.get_player_summaries(76561198163350464).await.unwrap();

    println!("{:?}", response);

    assert_eq!("76561198163350464", response[0].steam_id)
}

#[tokio::test]
async fn get_recently_played_games() {
    dotenv().ok();
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
    let steam = Steam::new(steam_api_key.as_str());
    let response = steam
        .get_recently_played_games(76561198163350464)
        .await
        .unwrap();

    println!("{:?}", response);

    assert_ne!(123434, response.total_count)
}

#[tokio::test]
async fn get_owned_games() {
    dotenv().ok();
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
    let steam = Steam::new(steam_api_key.as_str());
    let response = steam
        .get_owned_games(76561198163350464, true, false)
        .await
        .unwrap();

    println!("{:?}", response);

    assert_eq!(232, response.game_count)
}

#[tokio::test]
async fn get_player_achievements() {
    dotenv().ok();
    let steam_api_key = std::env::var("STEAM_API_KEY").expect("expected a api key for steam");
    let steam = Steam::new(steam_api_key.as_str());
    let response = steam
        .get_player_achievements(76561198163350464, 1245620)
        .await
        .unwrap();

    println!("{:?}", response);

    assert_eq!("ELDEN RING", response.game_name)
}
