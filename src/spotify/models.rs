use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub artists: Vec<Artist>,
    pub uri: String,  // uhm spotify:track:xxxxx
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Artist {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub tracks: PlaylistTracksWrapper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracksWrapper {
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTrack {
    pub track: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimplifiedPlaylist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
}