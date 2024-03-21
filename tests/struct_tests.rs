#[cfg(test)]
mod tests {

    #[test]
    fn test_owned_game_serialization_deserialization() {
        let owned_game = steam_api_wrapper::services::get_owned_games::OwnedGame {
            app_id: 570,
            name: Some("Dota 2".to_string()),
            recent_play_time: Some(120),
            total_play_time: Some(5000),
            img_icon_url: Some("test_icon_url".to_string()),
            img_logo_url: Some("test_logo_url".to_string()),
            has_community_visible_stats: Some(true),
        };

        let owned_game_json = serde_json::to_string(&owned_game).unwrap();

        let deserialized_owned_game: steam_api_wrapper::services::get_owned_games::OwnedGame =
            serde_json::from_str(&owned_game_json).unwrap();

        println!("{:?}", deserialized_owned_game);

        assert_eq!(deserialized_owned_game.app_id, 570);
        assert_eq!(deserialized_owned_game.name.unwrap(), "Dota 2");
        assert_eq!(deserialized_owned_game.recent_play_time.unwrap(), 120);
        assert_eq!(deserialized_owned_game.total_play_time.unwrap(), 5000);
        assert_eq!(
            deserialized_owned_game.img_icon_url.unwrap(),
            "test_icon_url"
        );
        assert_eq!(
            deserialized_owned_game.img_logo_url.unwrap(),
            "test_logo_url"
        );
        assert_eq!(
            deserialized_owned_game.has_community_visible_stats.unwrap(),
            true
        );
    }

    #[test]
    fn test_owned_games_serialization_deserialization() {
        let owned_game = steam_api_wrapper::services::get_owned_games::OwnedGame {
            app_id: 570,
            name: Some("Dota 2".to_string()),
            recent_play_time: Some(120),
            total_play_time: Some(5000),
            img_icon_url: Some("test_icon_url".to_string()),
            img_logo_url: Some("test_logo_url".to_string()),
            has_community_visible_stats: Some(true),
        };

        let owned_games = steam_api_wrapper::services::get_owned_games::OwnedGames {
            game_count: 1,
            games: vec![owned_game],
        };

        let owned_games_json = serde_json::to_string(&owned_games).unwrap();

        let deserialized_owned_games: steam_api_wrapper::services::get_owned_games::OwnedGames =
            serde_json::from_str(&owned_games_json).unwrap();

        println!("{:?}", deserialized_owned_games);

        assert_eq!(deserialized_owned_games.game_count, 1);
        assert_eq!(deserialized_owned_games.games.len(), 1);
        assert_eq!(deserialized_owned_games.games[0].app_id, 570);
    }

    #[test]
    fn test_get_owned_games_response_serialization_deserialization() {
        let owned_game = steam_api_wrapper::services::get_owned_games::OwnedGame {
            app_id: 570,
            name: Some("Dota 2".to_string()),
            recent_play_time: Some(120),
            total_play_time: Some(5000),
            img_icon_url: Some("test_icon_url".to_string()),
            img_logo_url: Some("test_logo_url".to_string()),
            has_community_visible_stats: Some(true),
        };

        let owned_games = steam_api_wrapper::services::get_owned_games::OwnedGames {
            game_count: 1,
            games: vec![owned_game],
        };

        let get_owned_games_response =
            steam_api_wrapper::services::get_owned_games::GetOwnedGamesResponse {
                response: owned_games,
            };

        let get_owned_games_response_json =
            serde_json::to_string(&get_owned_games_response).unwrap();

        let deserialized_get_owned_games_response:  steam_api_wrapper::services::get_owned_games::GetOwnedGamesResponse =
            serde_json::from_str(&get_owned_games_response_json).unwrap();

        println!("{:?}", deserialized_get_owned_games_response);

        assert_eq!(deserialized_get_owned_games_response.response.game_count, 1);
        assert_eq!(
            deserialized_get_owned_games_response.response.games.len(),
            1
        );
        assert_eq!(
            deserialized_get_owned_games_response.response.games[0].app_id,
            570
        );
    }
    #[test]
    fn test_achievement_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_schema_for_game::Achievement {
            achievement_name: "Test Achievement".to_string(),
            default_value: 100,
            display_name: "Test Achievement Display Name".to_string(),
            hidden: 0,
            description: Some("Test Achievement Description".to_string()),
            icon: "test_icon".to_string(),
            icongray: "test_icongray".to_string(),
        };

        let achievement_json = serde_json::to_string(&achievement).unwrap();

        let deserialized_achievement: steam_api_wrapper::services::get_schema_for_game::Achievement =
            serde_json::from_str(&achievement_json).unwrap();

        println!("{:?}", deserialized_achievement);

        assert_eq!(
            deserialized_achievement.achievement_name,
            "Test Achievement"
        );
        assert_eq!(deserialized_achievement.default_value, 100);
        assert_eq!(
            deserialized_achievement.display_name,
            "Test Achievement Display Name"
        );
        assert_eq!(deserialized_achievement.hidden, 0);
        assert_eq!(
            deserialized_achievement.description.unwrap(),
            "Test Achievement Description"
        );
        assert_eq!(deserialized_achievement.icon, "test_icon");
        assert_eq!(deserialized_achievement.icongray, "test_icongray");
    }

    #[test]
    fn test_available_game_stats_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_schema_for_game::Achievement {
            achievement_name: "Test Achievement".to_string(),
            default_value: 100,
            display_name: "Test Achievement Display Name".to_string(),
            hidden: 0,
            description: Some("Test Achievement Description".to_string()),
            icon: "test_icon".to_string(),
            icongray: "test_icongray".to_string(),
        };

        let available_game_stats =
            steam_api_wrapper::services::get_schema_for_game::AvailableGameStats {
                achievements: vec![achievement],
            };

        let available_game_stats_json = serde_json::to_string(&available_game_stats).unwrap();

        let deserialized_available_game_stats: steam_api_wrapper::services::get_schema_for_game::AvailableGameStats =
            serde_json::from_str(&available_game_stats_json).unwrap();

        println!("{:?}", deserialized_available_game_stats);

        assert_eq!(deserialized_available_game_stats.achievements.len(), 1);
        assert_eq!(
            deserialized_available_game_stats.achievements[0].achievement_name,
            "Test Achievement"
        );
    }

    #[test]
    fn test_game_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_schema_for_game::Achievement {
            achievement_name: "Test Achievement".to_string(),
            default_value: 100,
            display_name: "Test Achievement Display Name".to_string(),
            hidden: 0,
            description: Some("Test Achievement Description".to_string()),
            icon: "test_icon".to_string(),
            icongray: "test_icongray".to_string(),
        };

        let available_game_stats =
            steam_api_wrapper::services::get_schema_for_game::AvailableGameStats {
                achievements: vec![achievement],
            };

        let game = steam_api_wrapper::services::get_schema_for_game::Game {
            game_name: "Test Game".to_string(),
            game_version: "1.0".to_string(),
            available_game_stats: available_game_stats,
        };

        let game_json = serde_json::to_string(&game).unwrap();

        let deserialized_game: steam_api_wrapper::services::get_schema_for_game::Game =
            serde_json::from_str(&game_json).unwrap();

        println!("{:?}", deserialized_game);

        assert_eq!(deserialized_game.game_name, "Test Game");
        assert_eq!(deserialized_game.game_version, "1.0");
        assert_eq!(deserialized_game.available_game_stats.achievements.len(), 1);
        assert_eq!(
            deserialized_game.available_game_stats.achievements[0].achievement_name,
            "Test Achievement"
        );
    }

    #[test]
    fn test_root_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_schema_for_game::Achievement {
            achievement_name: "Test Achievement".to_string(),
            default_value: 100,
            display_name: "Test Achievement Display Name".to_string(),
            hidden: 0,
            description: Some("Test Achievement Description".to_string()),
            icon: "test_icon".to_string(),
            icongray: "test_icongray".to_string(),
        };

        let available_game_stats =
            steam_api_wrapper::services::get_schema_for_game::AvailableGameStats {
                achievements: vec![achievement],
            };

        let game = steam_api_wrapper::services::get_schema_for_game::Game {
            game_name: "Test Game".to_string(),
            game_version: "1.0".to_string(),
            available_game_stats: available_game_stats,
        };

        let root = steam_api_wrapper::services::get_schema_for_game::Root { game: game };

        let root_json = serde_json::to_string(&root).unwrap();

        let deserialized_root: steam_api_wrapper::services::get_schema_for_game::Root =
            serde_json::from_str(&root_json).unwrap();

        println!("{:?}", deserialized_root);

        assert_eq!(deserialized_root.game.game_name, "Test Game");
        assert_eq!(deserialized_root.game.game_version, "1.0");
        assert_eq!(
            deserialized_root
                .game
                .available_game_stats
                .achievements
                .len(),
            1
        );
        assert_eq!(
            deserialized_root.game.available_game_stats.achievements[0].achievement_name,
            "Test Achievement"
        );
    }

    #[test]
    fn test_achievements_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_player_achievements::Achievements {
            api_name: "test_achievement".to_string(),
            achieved: 1,
            name: Some("Test Achievement".to_string()),
            description: Some("Test Achievement Description".to_string()),
            unlock_time: Some(1647926865),
        };

        let achievement_json = serde_json::to_string(&achievement).unwrap();

        let deserialized_achievement: steam_api_wrapper::services::get_player_achievements::Achievements =
            serde_json::from_str(&achievement_json).unwrap();

        println!("{:?}", deserialized_achievement);

        assert_eq!(deserialized_achievement.api_name, "test_achievement");
        assert_eq!(deserialized_achievement.achieved, 1);
        assert_eq!(deserialized_achievement.name.unwrap(), "Test Achievement");
        assert_eq!(
            deserialized_achievement.description.unwrap(),
            "Test Achievement Description"
        );
        assert_eq!(deserialized_achievement.unlock_time.unwrap(), 1647926865);
    }

    #[test]
    fn test_player_achievement_info_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_player_achievements::Achievements {
            api_name: "test_achievement".to_string(),
            achieved: 1,
            name: Some("Test Achievement".to_string()),
            description: Some("Test Achievement Description".to_string()),
            unlock_time: Some(1647926865),
        };

        let player_achievement_info =
            steam_api_wrapper::services::get_player_achievements::PlayerAchievementInfo {
                game_name: Some("Test Game".to_string()),
                steam_id: Some("test_steam_id".to_string()),
                achievements: Some(vec![achievement]),
                success: true,
                error: None,
            };

        let player_achievement_info_json = serde_json::to_string(&player_achievement_info).unwrap();

        let deserialized_player_achievement_info: steam_api_wrapper::services::get_player_achievements::PlayerAchievementInfo =
            serde_json::from_str(&player_achievement_info_json).unwrap();

        println!("{:?}", deserialized_player_achievement_info);

        assert_eq!(
            deserialized_player_achievement_info.game_name.unwrap(),
            "Test Game"
        );
        assert_eq!(
            deserialized_player_achievement_info.steam_id.unwrap(),
            "test_steam_id"
        );
        assert_eq!(
            deserialized_player_achievement_info
                .achievements
                .unwrap()
                .len(),
            1
        );
        assert_eq!(deserialized_player_achievement_info.success, true);
    }

    #[test]
    fn test_player_stats_serialization_deserialization() {
        let achievement = steam_api_wrapper::services::get_player_achievements::Achievements {
            api_name: "test_achievement".to_string(),
            achieved: 1,
            name: Some("Test Achievement".to_string()),
            description: Some("Test Achievement Description".to_string()),
            unlock_time: Some(1647926865),
        };

        let player_achievement_info =
            steam_api_wrapper::services::get_player_achievements::PlayerAchievementInfo {
                game_name: Some("Test Game".to_string()),
                steam_id: Some("test_steam_id".to_string()),
                achievements: Some(vec![achievement]),
                success: true,
                error: None,
            };

        let player_stats = steam_api_wrapper::services::get_player_achievements::PlayerStats {
            player_stats: player_achievement_info,
        };

        let player_stats_json = serde_json::to_string(&player_stats).unwrap();

        let deserialized_player_stats: steam_api_wrapper::services::get_player_achievements::PlayerStats =
            serde_json::from_str(&player_stats_json).unwrap();

        println!("{:?}", deserialized_player_stats);

        assert_eq!(
            deserialized_player_stats.player_stats.game_name.unwrap(),
            "Test Game"
        );
        assert_eq!(
            deserialized_player_stats.player_stats.steam_id.unwrap(),
            "test_steam_id"
        );
        assert_eq!(
            deserialized_player_stats
                .player_stats
                .achievements
                .unwrap()
                .len(),
            1
        );
        assert_eq!(deserialized_player_stats.player_stats.success, true);
    }

    #[test]
    fn test_player_serialization_deserialization() {
        let player = steam_api_wrapper::services::get_player_info::Player {
            steam_id: "12345678901234567".to_string(),
            community_visibility_state: 3,
            profile_state: 1,
            persona_name: "TestPlayer".to_string(),
            last_logoff: Some(1647926865),
            comment_permission: Some(2),
            profile_url: "https://steamcommunity.com/id/testplayer/".to_string(),
            avatar: "test_avatar_url".to_string(),
            avatar_hash: "test_avatar_hash".to_string(),
            avatar_medium: "test_avatar_medium_url".to_string(),
            avatar_full: "test_avatar_full_url".to_string(),
            persona_state: Some(1),
            real_name: Some("John Doe".to_string()),
            primary_clan_id: "12345678901234567".to_string(),
            time_created: Some(1234567890),
            persona_state_flags: 1,
            game_extra_info: Some("Test Game".to_string()),
            game_id: Some("570".to_string()),
            loc_country_code: Some("US".to_string()),
            loc_state_code: Some("WA".to_string()),
            loc_city_id: Some(123456),
        };

        let player_json = serde_json::to_string(&player).unwrap();

        let deserialized_player: steam_api_wrapper::services::get_player_info::Player =
            serde_json::from_str(&player_json).unwrap();

        println!("{:?}", deserialized_player);

        assert_eq!(deserialized_player.steam_id, "12345678901234567");
        assert_eq!(deserialized_player.community_visibility_state, 3);
        assert_eq!(deserialized_player.profile_state, 1);
        assert_eq!(deserialized_player.persona_name, "TestPlayer");
        assert_eq!(deserialized_player.last_logoff.unwrap(), 1647926865);
        assert_eq!(deserialized_player.comment_permission.unwrap(), 2);
        assert_eq!(
            deserialized_player.profile_url,
            "https://steamcommunity.com/id/testplayer/"
        );
        assert_eq!(deserialized_player.avatar, "test_avatar_url");
        assert_eq!(deserialized_player.avatar_hash, "test_avatar_hash");
        assert_eq!(deserialized_player.avatar_medium, "test_avatar_medium_url");
        assert_eq!(deserialized_player.avatar_full, "test_avatar_full_url");
        assert_eq!(deserialized_player.persona_state.unwrap(), 1);
        assert_eq!(deserialized_player.real_name.unwrap(), "John Doe");
        assert_eq!(deserialized_player.primary_clan_id, "12345678901234567");
        assert_eq!(deserialized_player.time_created.unwrap(), 1234567890);
        assert_eq!(deserialized_player.persona_state_flags, 1);
        assert_eq!(deserialized_player.game_extra_info.unwrap(), "Test Game");
        assert_eq!(deserialized_player.game_id.unwrap(), "570");
        assert_eq!(deserialized_player.loc_country_code.unwrap(), "US");
        assert_eq!(deserialized_player.loc_state_code.unwrap(), "WA");
        assert_eq!(deserialized_player.loc_city_id.unwrap(), 123456);
    }

    #[test]
    fn test_players_serialization_deserialization() {
        let player = steam_api_wrapper::services::get_player_info::Player {
            steam_id: "12345678901234567".to_string(),
            community_visibility_state: 3,
            profile_state: 1,
            persona_name: "TestPlayer".to_string(),
            last_logoff: Some(1647926865),
            comment_permission: Some(2),
            profile_url: "https://steamcommunity.com/id/testplayer/".to_string(),
            avatar: "test_avatar_url".to_string(),
            avatar_hash: "test_avatar_hash".to_string(),
            avatar_medium: "test_avatar_medium_url".to_string(),
            avatar_full: "test_avatar_full_url".to_string(),
            persona_state: Some(1),
            real_name: Some("John Doe".to_string()),
            primary_clan_id: "12345678901234567".to_string(),
            time_created: Some(1234567890),
            persona_state_flags: 1,
            game_extra_info: Some("Test Game".to_string()),
            game_id: Some("570".to_string()),
            loc_country_code: Some("US".to_string()),
            loc_state_code: Some("WA".to_string()),
            loc_city_id: Some(123456),
        };

        let players = steam_api_wrapper::services::get_player_info::Players {
            players: vec![player],
        };

        let players_json = serde_json::to_string(&players).unwrap();

        let deserialized_players: steam_api_wrapper::services::get_player_info::Players =
            serde_json::from_str(&players_json).unwrap();

        println!("{:?}", deserialized_players);

        assert_eq!(deserialized_players.players.len(), 1);
        assert_eq!(
            deserialized_players.players[0].steam_id,
            "12345678901234567"
        );
    }

    #[test]
    fn test_player_response_serialization_deserialization() {
        let player = steam_api_wrapper::services::get_player_info::Player {
            steam_id: "12345678901234567".to_string(),
            community_visibility_state: 3,
            profile_state: 1,
            persona_name: "TestPlayer".to_string(),
            last_logoff: Some(1647926865),
            comment_permission: Some(2),
            profile_url: "https://steamcommunity.com/id/testplayer/".to_string(),
            avatar: "test_avatar_url".to_string(),
            avatar_hash: "test_avatar_hash".to_string(),
            avatar_medium: "test_avatar_medium_url".to_string(),
            avatar_full: "test_avatar_full_url".to_string(),
            persona_state: Some(1),
            real_name: Some("John Doe".to_string()),
            primary_clan_id: "12345678901234567".to_string(),
            time_created: Some(1234567890),
            persona_state_flags: 1,
            game_extra_info: Some("Test Game".to_string()),
            game_id: Some("570".to_string()),
            loc_country_code: Some("US".to_string()),
            loc_state_code: Some("WA".to_string()),
            loc_city_id: Some(123456),
        };

        let players = steam_api_wrapper::services::get_player_info::Players {
            players: vec![player],
        };

        let player_response =
            steam_api_wrapper::services::get_player_info::PlayerResponse { response: players };

        let player_response_json = serde_json::to_string(&player_response).unwrap();

        let deserialized_player_response: steam_api_wrapper::services::get_player_info::PlayerResponse =
            serde_json::from_str(&player_response_json).unwrap();

        println!("{:?}", deserialized_player_response);

        assert_eq!(deserialized_player_response.response.players.len(), 1);
        assert_eq!(
            deserialized_player_response.response.players[0].steam_id,
            "12345678901234567"
        );
    }

    #[test]
    fn test_recently_played_summary_game_serialization_deserialization() {
        let game = steam_api_wrapper::services::get_recently_played_games::Game {
            app_id: 570,
            name: "Dota 2".to_string(),
            recently_played: 120,
            overall_plat_time: 5000,
            img_icon_url: "test_icon_url".to_string(),
            img_logo_url: Some("test_logo_url".to_string()),
        };

        let game_json = serde_json::to_string(&game).unwrap();

        let deserialized_game: steam_api_wrapper::services::get_recently_played_games::Game =
            serde_json::from_str(&game_json).unwrap();

        println!("{:?}", deserialized_game);

        assert_eq!(deserialized_game.app_id, 570);
        assert_eq!(deserialized_game.name, "Dota 2");
        assert_eq!(deserialized_game.recently_played, 120);
        assert_eq!(deserialized_game.overall_plat_time, 5000);
        assert_eq!(deserialized_game.img_icon_url, "test_icon_url");
        assert_eq!(deserialized_game.img_logo_url.unwrap(), "test_logo_url");
    }

    #[test]
    fn test_games_serialization_deserialization() {
        let game = steam_api_wrapper::services::get_recently_played_games::Game {
            app_id: 570,
            name: "Dota 2".to_string(),
            recently_played: 120,
            overall_plat_time: 5000,
            img_icon_url: "test_icon_url".to_string(),
            img_logo_url: Some("test_logo_url".to_string()),
        };

        let games = steam_api_wrapper::services::get_recently_played_games::Games {
            total_count: 1,
            games: vec![game],
        };

        let games_json = serde_json::to_string(&games).unwrap();

        let deserialized_games: steam_api_wrapper::services::get_recently_played_games::Games =
            serde_json::from_str(&games_json).unwrap();

        println!("{:?}", deserialized_games);

        assert_eq!(deserialized_games.total_count, 1);
        assert_eq!(deserialized_games.games.len(), 1);
        assert_eq!(deserialized_games.games[0].app_id, 570);
    }

    #[test]
    fn test_recently_played_summary_serialization_deserialization() {
        let game = steam_api_wrapper::services::get_recently_played_games::Game {
            app_id: 570,
            name: "Dota 2".to_string(),
            recently_played: 120,
            overall_plat_time: 5000,
            img_icon_url: "test_icon_url".to_string(),
            img_logo_url: Some("test_logo_url".to_string()),
        };

        let games = steam_api_wrapper::services::get_recently_played_games::Games {
            total_count: 1,
            games: vec![game],
        };

        let recently_played_summary =
            steam_api_wrapper::services::get_recently_played_games::RecentlyPlayedSummary {
                response: games,
            };

        let recently_played_summary_json = serde_json::to_string(&recently_played_summary).unwrap();

        let deserialized_recently_played_summary: steam_api_wrapper::services::get_recently_played_games::RecentlyPlayedSummary = serde_json::from_str(&recently_played_summary_json).unwrap();

        println!("{:?}", deserialized_recently_played_summary);

        assert_eq!(deserialized_recently_played_summary.response.total_count, 1);
        assert_eq!(deserialized_recently_played_summary.response.games.len(), 1);
        assert_eq!(
            deserialized_recently_played_summary.response.games[0].app_id,
            570
        );
    }

    #[test]
    fn test_steam_player_level_serialization_deserialization() {
        let player_level =
            steam_api_wrapper::services::get_steam_level::SteamPlayerLevel { player_level: 10 };

        let player_level_json = serde_json::to_string(&player_level).unwrap();

        let deserialized_player_level: steam_api_wrapper::services::get_steam_level::SteamPlayerLevel =
            serde_json::from_str(&player_level_json).unwrap();

        println!("{:?}", deserialized_player_level);

        assert_eq!(deserialized_player_level.player_level, 10);
    }

    #[test]
    fn test_steam_player_level_response_serialization_deserialization() {
        let player_level =
            steam_api_wrapper::services::get_steam_level::SteamPlayerLevel { player_level: 10 };

        let player_level_response =
            steam_api_wrapper::services::get_steam_level::SteamPlayerLevelRespone {
                response: player_level,
            };

        let player_level_response_json = serde_json::to_string(&player_level_response).unwrap();

        let deserialized_player_level_response: steam_api_wrapper::services::get_steam_level::SteamPlayerLevelRespone =
            serde_json::from_str(&player_level_response_json).unwrap();

        println!("{:?}", deserialized_player_level_response);

        assert_eq!(deserialized_player_level_response.response.player_level, 10);
    }
}
