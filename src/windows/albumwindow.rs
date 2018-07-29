use gtk::prelude::*;
use gtk::{
    Builder, Entry, EntryBuffer, Label, ListBox, ListBoxRow, ListStore, Orientation, Window,
    WindowType,
};

use relm::{Relm, Update, Widget};

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {
    entries: Vec<TrackEntry>,
    album_buffer: EntryBuffer,
}

pub struct AlbumWindow {
    window: Window,
}

struct TrackEntry {
    container: gtk::Box,
    title: EntryBuffer,
    lyrics: EntryBuffer,
    title_entry: Entry,
    lyrics_entry: Entry,
    num: u32,
    num_label: Label,
}

impl TrackEntry {
    pub fn new_from_data(title: String, lyrics: String, entry: u32) -> TrackEntry {
        let num_label = Label::new(Some(entry.to_string().as_str()));

        //Setup buffers
        let title_buffer = EntryBuffer::new(Some(title.as_str()));
        let title_entry = Entry::new_with_buffer(&title_buffer);
        let lyrics_buffer = EntryBuffer::new(Some(lyrics.as_str()));
        let lyrics_entry = Entry::new_with_buffer(&lyrics_buffer);

        let container = gtk::Box::new(Orientation::Horizontal, 0);
        container.pack_end(&num_label, false, false, 0);
        container.pack_end(&title_entry, true, true, 0);
        container.pack_end(&lyrics_entry, true, true, 0);

        TrackEntry {
            container,
            title: title_buffer,
            lyrics: lyrics_buffer,
            lyrics_entry,
            title_entry,
            num: entry,
            num_label,
        }
    }
}

impl Update for AlbumWindow {
    type Model = Model;
    type ModelParam = (String, Vec<(String, String)>);
    type Msg = Msg;

    fn model(_: &Relm<Self>, (title, tracks): (String, Vec<(String, String)>)) -> Model {
        let mut entries = Vec::new();
        let album_buffer = EntryBuffer::new(Some(title.as_str()));

        let mut i = 0;
        for (title, lyrics) in tracks {
            entries.push(TrackEntry::new_from_data(title, lyrics, i));
            i += 1;
        }

        Model {
            entries,
            album_buffer,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => {
                self.window.destroy();
            }
        }
    }
}

impl Widget for AlbumWindow {
    type Root = Window;
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("albumwindow.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: Window = builder.get_object("window").unwrap();

        let album_entry: Entry = builder.get_object("album_entry").unwrap();
        album_entry.set_buffer(&model.album_buffer);

        let track_list_box: ListBox = builder.get_object("track_list_box").unwrap();
        for entry in &model.entries {
            let row = ListBoxRow::new();
            row.add(&entry.container);
            track_list_box.add(&row);
        }

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Msg::Quit, Inhibit(false))
        );
        AlbumWindow { window }
    }
}
