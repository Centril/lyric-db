use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use treexml::{Document, Element};

pub mod error;
pub mod metadata;
pub use self::error::DatabaseError;
use self::metadata::*;
pub struct Database {
    pub entries: Vec<Artist>,
    file_path: String,
}

impl Database {
    pub fn empty() -> Database {
        Database {
            entries: Vec::new(),
            file_path: String::new(),
        }
    }
    pub fn clean(&mut self) {
        self.entries.clear();
    }
    pub fn from(path_str: &str) -> Result<Database, DatabaseError> {
        let mut entries = Vec::new();

        //Open file for reading
        let path = Path::new(path_str);
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut data = String::new();
        buf_reader.read_to_string(&mut data)?;

        let doc = Document::parse(data.as_bytes()).unwrap();
        if let Some(root) = doc.root {
            if root.name != "database" {
                return Err(DatabaseError::InvalidTag(root.name));
            }
            for artist_tag in root.children {
                let mut artist = Artist::new();
                if artist_tag.name != "artist" {
                    return Err(DatabaseError::InvalidTag(artist_tag.name));
                }

                if artist_tag.attributes.is_empty() {
                    return Err(DatabaseError::MissingAttribute((
                        "name".to_string(),
                        artist_tag.name,
                    )));
                }

                for (attribute, value) in artist_tag.attributes {
                    if attribute != "name" {
                        return Err(DatabaseError::InvalidAttribute((attribute, value)));
                    } else {
                        artist.name = value;
                    }
                }

                let mut albums = Vec::new();
                for album_tag in artist_tag.children {
                    if album_tag.name != "album" {
                        return Err(DatabaseError::InvalidTag(album_tag.name));
                    }
                    let mut album = Album::new();
                    for (attr, val) in album_tag.attributes {
                        match attr.as_ref() {
                            "title" => album.title = val,
                            "tracks" => album.track_count = val.parse::<u8>().unwrap(),
                            _ => {
                                return Err(DatabaseError::InvalidAttribute((attr, album_tag.name)))
                            }
                        };
                    }

                    let mut tracks = Vec::new();
                    for track_tag in album_tag.children {
                        let mut track = Track::new();
                        for (attr, val) in track_tag.attributes {
                            match attr.as_ref() {
                                "name" => track.title = val,
                                "num" => track.track = val.parse::<u8>().unwrap(),
                                _ => {
                                    return Err(DatabaseError::InvalidAttribute((
                                        attr,
                                        track_tag.name,
                                    )))
                                }
                            };
                        }

                        if let Some(lyrics) = track_tag.text {
                            track.lyrics = lyrics;
                        }

                        tracks.push(track);
                    }

                    tracks.sort_by(|a, b| a.track.cmp(&b.track));
                    album.tracks = tracks;
                    albums.push(album);
                }
                artist.albums = albums;
                entries.push(artist);
            }
        } else {
            return Err(DatabaseError::Empty);
        }

        Ok(Database {
            entries: entries,
            file_path: path_str.to_owned(),
        })
    }

    pub fn save(&self, path: &str) -> Result<(), DatabaseError> {
        let mut root = Element::new("database");
        for artist in &self.entries {
            let mut artist_el = Element::new("artist");
            artist_el
                .attributes
                .insert("name".to_owned(), artist.name.clone());
            for album in &artist.albums {
                let mut album_el = Element::new("album");
                album_el
                    .attributes
                    .insert("title".to_owned(), album.title.clone());
                album_el
                    .attributes
                    .insert("tracks".to_owned(), album.track_count.to_string());

                for track in &album.tracks {
                    let mut track_el = Element::new("track");
                    track_el
                        .attributes
                        .insert("num".to_owned(), track.track.to_string());
                    track_el
                        .attributes
                        .insert("name".to_owned(), track.title.to_string());
                    track_el.text = Some(track.lyrics.clone());
                    album_el.children.push(track_el);
                }
                artist_el.children.push(album_el);
            }
            root.children.push(artist_el);
        }
        let doc = Document {
            root: Some(root),
            ..Document::default()
        };
        println!("{}", doc);
        Ok(())
    }
}
