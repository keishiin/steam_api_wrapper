/*
File contains all the consts vars
 used with out the api wrapper
*/

// BASE URL
pub const BASE_URL: &str = "https://api.steampowered.com";

// API VERSIONS
pub const VERSION_V1: &str = "v1";
pub const VERSION_V2: &str = "v2";

//INTERFACES
pub const ISTEAM_USER: &str = "ISteamUser";
pub const IPLAYER_SERVICE: &str = "IPlayerService";
pub const ISTEAM_USER_STATS: &str = "ISteamUserStats";

// METHODS
pub const GET_PLAYER_SUMMARIES: &str = "GetPlayerSummaries";
pub const GET_RECENTLY_PLAYED_GAMES: &str = "GetRecentlyPlayedGames";
pub const GET_OWNED_GAMES: &str = "GetOwnedGames";
pub const _GET_NUMBER_OF_CURRENT_PLAYERS: &str = "GetNumberOfCurrentPlayers";
pub const GET_PLAYER_ACHIEVEMENTS: &str = "GetPlayerAchievements";
pub const _GET_SINGLE_GAME_PLAYTIME: &str = "GetSingleGamePlaytime";
pub const GET_STEAM_LEVEL: &str = "GetSteamLevel";
pub const GET_SCHEMA_FOR_GAME: &str = "GetSchemaForGame";
