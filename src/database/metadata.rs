use std::fmt;

#[derive(Debug)]
pub struct Artist {
    pub name: String,
    pub albums: Vec<Album>,
}

#[derive(Debug)]
pub struct Album {
    pub title: String,
    pub tracks: Vec<Track>,
    pub track_count: u8,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub lyrics: String,
    pub track: u8,
}

impl Artist {
    pub fn new() -> Artist {
        Artist {
            name: String::new(),
            albums: Vec::new(),
        }
    }
}

impl Album {
    pub fn new() -> Album {
        Album {
            title: String::new(),
            track_count: 0,
            tracks: Vec::new(),
        }
    }
}

impl Track {
    pub fn new() -> Track {
        Track {
            track: 0,
            lyrics: String::new(),
            title: String::new(),
        }
    }
}

impl fmt::Display for Artist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
