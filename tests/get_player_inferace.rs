use steam_wrapper::Steam;

#[tokio::test]
async fn get_player_summaries() {
    let steam = Steam::new("steam_key");

    let response = steam.get_player_summaries(2345436547).await.unwrap();

    println!(
        "{:?}",
        response
    );

    assert_eq!(1, 1)
}