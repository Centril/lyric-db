use std::fmt;

#[derive(Eq, Hash, Debug)]
pub struct Artist {
    pub name: String,
}

#[derive(Eq, Hash, Debug)]
pub struct Album {
    pub title: String,
    pub track_count: u8,
}

#[derive(Debug)]
pub struct Track {
    pub title: String,
    pub lyrics: String,
    pub track: u8,
}

impl Artist {
    pub fn new() -> Artist {
        Artist {
            name: String::new(),
        }
    }
}

impl Album {
    pub fn new() -> Album {
        Album {
            title: String::new(),
            track_count: 0,
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

impl PartialEq for Artist {
    fn eq(&self, other: &Artist) -> bool {
        self.name == other.name
    }
}

impl PartialEq for Album {
    fn eq(&self, other: &Album) -> bool {
        self.title == other.title
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Track) -> bool {
        self.title == other.title
    }
}
