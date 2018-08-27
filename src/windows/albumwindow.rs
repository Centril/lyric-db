use gtk::prelude::*;
use gtk::{
    Builder, Entry, EntryBuffer, Label, ListBox, ListBoxRow, Orientation, TextBuffer, TextView,
    Window,
};

use relm::{Relm, Update, Widget};

#[derive(Msg)]
pub enum Msg {
    SelectedTrack,
    Quit,
}

pub struct Model {
    entries: Vec<TrackEntry>,
    album_buffer: EntryBuffer,
}

pub struct AlbumWindow {
    window: Window,
    model: Model,
    lyrics_view: TextView,
    track_list_box: ListBox,
}

struct TrackEntry {
    container: gtk::Box,
    title: EntryBuffer,
    lyrics_buffer: TextBuffer,
    title_entry: Entry,
    num: u32,
    num_label: Label,
}

impl TrackEntry {
    pub fn new_from_data(title: &str, lyrics: &str, entry: u32) -> TrackEntry {
        let num_label = Label::new(Some(entry.to_string().as_str()));

        //Setup buffers
        let title_buffer = EntryBuffer::new(Some(title));
        let title_entry = Entry::new_with_buffer(&title_buffer);

        let container = gtk::Box::new(Orientation::Horizontal, 0);
        container.pack_start(&num_label, false, false, 0);
        container.pack_start(&title_entry, true, true, 0);

        let lyrics_buffer = gtk::TextBuffer::new(None);
        lyrics_buffer.insert_at_cursor(lyrics);

        TrackEntry {
            container,
            title: title_buffer,
            title_entry,
            num: entry,
            num_label,
            lyrics_buffer,
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

        for (i, (title, lyrics)) in tracks.iter().enumerate() {
            entries.push(TrackEntry::new_from_data(title, lyrics, i as u32));
        }

        Model {
            entries,
            album_buffer,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SelectedTrack => {
                let row = self.track_list_box.get_selected_row().unwrap();
                self.lyrics_view.set_buffer(Some(
                    &self.model.entries[row.get_index() as usize].lyrics_buffer,
                ));
            }
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

        get_object!(window, Window, builder);

        get_object!(album_entry, Entry, builder);
        album_entry.set_buffer(&model.album_buffer);

        get_object!(track_list_box, ListBox, builder);

        for entry in &model.entries {
            let row = ListBoxRow::new();
            row.add(&entry.container);
            track_list_box.add(&row);
        }

        get_object!(lyrics_view, TextView, builder);

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Msg::Quit, Inhibit(false))
        );

        connect!(
            relm,
            track_list_box,
            connect_row_selected(_, _),
            Msg::SelectedTrack
        );
        AlbumWindow {
            window,
            model,
            lyrics_view,
            track_list_box,
        }
    }
}
