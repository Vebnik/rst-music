#[derive(Debug, poise::ChoiceParameter)]
pub enum EngineChoice {
    YouTube,
    YouTubeMusic,
    SoundCloud,
    /// NOTE: Requires LavaSrc plugin.
    Deezer,
    /// NOTE: Requires LavaSrc plugin.
    DeezerISRC,
    /// NOTE: Requires LavaSrc plugin.
    YandexMusic,
}

impl ToString for EngineChoice {
    fn to_string(&self) -> String {
        match self {
            EngineChoice::YouTube => "YouTube".to_string(),
            EngineChoice::YouTubeMusic => "YouTubeMusic".to_string(),
            EngineChoice::SoundCloud => "SoundCloud".to_string(),
            EngineChoice::Deezer => "Deezer".to_string(),
            EngineChoice::DeezerISRC => "DeezerISRC".to_string(),
            EngineChoice::YandexMusic => "YandexMusic".to_string(),
        }
    }
}