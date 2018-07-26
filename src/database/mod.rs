use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

// use xml::reader::{EventReader, XmlEvent};
// use xml::ParserConfig;
use treexml::Document;

pub mod error;
pub mod metadata;
pub use self::error::DatabaseError;
use self::metadata::*;
pub struct Database {
    pub entries: Vec<Artist>,
    file_path: String,
}

// fn read_artist(attrs: Vec<OwnedAttribute>) -> Artist {
//     for a in attrs {
//         if a.value == "name" {
//             return Artist { name: a.value };
//         }
//     }
// }
impl Database {
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
}
