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
    pub entries: HashMap<Artist, HashMap<Album, Vec<Track>>>,
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
        let mut entries: HashMap<Artist, HashMap<Album, Vec<Track>>> = HashMap::new();

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

                let mut albums = HashMap::<Album, Vec<Track>>::new();
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
                    albums.insert(album, tracks);
                }
                entries.insert(artist, albums);
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

// impl Database {
//     pub fn from(path_str: &str) -> Result<Database, DatabaseError> {
//         let mut entries: HashMap<Artist, HashMap<Album, Vec<Track>>> = HashMap::new();

//         //Open file for reading
//         let path = Path::new(path_str);
//         let file = File::open(path)?;
//         let file = BufReader::new(file);

//         let mut parser =
//             EventReader::new_with_config(file, ParserConfig::new().trim_whitespace(true));

//         // Parse file
//         'root: loop {
//             match parser.next() {
//                 Ok(XmlEvent::StartElement {
//                     name,
//                     attributes,
//                     namespace: _,
//                 }) => {
//                     let mut artist = Artist {
//                         name: String::new(),
//                     };
//                     for a in attributes {
//                         if a.name.local_name != "name" {
//                             panic!("Invalid attribute {}", a.name.local_name);
//                         }
//                         artist.name = a.value
//                     }

//                     let mut albums: HashMap<Album, Vec<Track>> = HashMap::new();
//                     'albumloop: loop {
//                         match parser.next() {
//                             Ok(XmlEvent::StartElement {
//                                 name,
//                                 attributes,
//                                 namespace: _,
//                             }) => {
//                                 if name.local_name != "album" {
//                                     panic!("Invalid tag: {}", name);
//                                 }

//                                 let mut album = Album {
//                                     title: String::new(),
//                                     track_count: 0,
//                                 };
//                                 for a in attributes {
//                                     match a.name.local_name.as_ref() {
//                                         "title" => album.title = a.value,
//                                         "tracks" => {
//                                             album.track_count = a.value.parse::<u8>().unwrap()
//                                         }
//                                         _ => panic!("Invalid attribute: {}", a.name.local_name),
//                                     }
//                                 }

//                                 let mut tracks = Vec::new();
//                                 'trackloop: loop {
//                                     match parser.next() {
//                                         Ok(XmlEvent::StartElement {
//                                             name,
//                                             attributes,
//                                             namespace: _,
//                                         }) => {
//                                             let mut track = Track {
//                                                 title: String::new(),
//                                                 lyrics: String::new(),
//                                                 track: 0,
//                                             };

//                                             if name.local_name != "track" {
//                                                 panic!("Not track");
//                                             }

//                                             for a in attributes {
//                                                 match a.name.local_name.as_ref() {
//                                                     "num" => {
//                                                         track.track = a.value.parse::<u8>().unwrap()
//                                                     }
//                                                     "name" => track.title = a.value,
//                                                     _ => panic!(
//                                                         "Invalid attribute {}",
//                                                         a.name.local_name
//                                                     ),
//                                                 }
//                                             }

//                                             loop {
//                                                 match parser.next() {
//                                                     Ok(XmlEvent::EndElement { name }) => {
//                                                         if name.local_name != "track" {
//                                                             panic!("track error");
//                                                         }
//                                                         break;
//                                                     }
//                                                     Ok(XmlEvent::Characters(s)) => track.lyrics = s,
//                                                     Ok(s) => {
//                                                         panic!("Everything went wrong: {:?}", s)
//                                                     }
//                                                     Err(e) => panic!(
//                                                         "Everything went wrong with error {}",
//                                                         e
//                                                     ),
//                                                 }
//                                             }
//                                             tracks.push(track);
//                                         }
//                                         Ok(XmlEvent::EndElement { name }) => {
//                                             if name.local_name != "album" {
//                                                 panic!(
//                                                     "Expected end of album, got {}",
//                                                     name.local_name
//                                                 );
//                                             }

//                                             break 'trackloop;
//                                         }
//                                         Err(_) => panic!("trackloop error"),
//                                         _ => panic!("trackloop is different"),
//                                     }
//                                 }
//                                 albums.insert(album, tracks);
//                             }
//                             Ok(XmlEvent::EndElement { name }) => {
//                                 if name.local_name != "artist" {
//                                     panic!("Artist should've ended");
//                                 }
//                                 entries.insert(artist, albums);
//                                 break 'albumloop;
//                             }
//                             Err(e) => panic!("{:?}", e),
//                             Ok(s) => panic!("{:?} not ok", s),
//                         }
//                     }
//                 }
//                 Ok(XmlEvent::EndDocument) => {
//                     break 'root;
//                 }
//                 Ok(XmlEvent::StartDocument {
//                     version: _,
//                     encoding: _,
//                     standalone: _,
//                 }) => (),
//                 _ => (),
//                 Err(_) => panic!("Root error!"),
//             }
//         }

//         let out = Database {
//             entries: entries,
//             file_path: path_str.to_owned(),
//         };

//         Ok(out)
//     }
// }
