#![feature(use_extern_macros)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate treexml;

mod database;
use database::Database;

use gtk::prelude::*;
use gtk::{
    Builder, ButtonsType, DialogFlags, FileChooserAction, FileChooserDialog, Label, MenuItem,
    MessageDialog, MessageType, TreeStore, TreeView, TreeViewColumn, Window,
};

use relm::{Relm, Update, Widget};

use std::path::Path;

fn update_treestore(db: &Database, input: &TreeStore) {
    input.clear();
    for artist in &db.entries {
        let iter = input.insert_with_values(None, None, &[0], &[&artist.name]);

        for album in &artist.albums {
            let iter = input.insert_with_values(Some(&iter), None, &[0], &[&album.title]);

            for track in &album.tracks {
                input.insert_with_values(Some(&iter), None, &[0], &[&track.title]);
            }
        }
    }
}

#[derive(Msg)]
pub enum Msg {
    SelectedItem,
    MenuOpen,
    Quit,
}

pub struct Model {
    db: Database,
    tree_store: gtk::TreeStore,
}

struct Win {
    tree_view: TreeView,
    model: Model,
    window: Window,
    text_viewer: Label,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    //Return empty model
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            db: Database::empty(),
            tree_store: TreeStore::new(&[String::static_type()]),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SelectedItem => {
                let selection = self.tree_view.get_selection();
                if let Some((model, iter)) = selection.get_selected() {
                    let mut path = model
                        .get_path(&iter)
                        .expect("failed to get path")
                        .to_string();
                    let places: Vec<usize> = path
                        .split(":")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect();
                    if places.len() == 3 {
                        let current =
                            &self.model.db.entries[places[0]].albums[places[1]].tracks[places[2]];
                        self.text_viewer.set_text(&current.lyrics);
                    } else if places.len() > 3 {
                        panic!();
                    }
                }
            }
            Msg::MenuOpen => {
                let dialog = FileChooserDialog::new(
                    Some("Open..."),
                    Some(&self.window),
                    FileChooserAction::Open,
                );
                dialog.add_button("Open", 0);
                dialog.add_button("Close", 1);
                let result = dialog.run();
                if result == 0 {
                    let filename = dialog.get_filename().expect("Failed to get filename");
                    let file = Path::new(&filename);
                    if !file.exists() {
                        let dialog = MessageDialog::new(
                            Some(&self.window),
                            DialogFlags::all(),
                            MessageType::Error,
                            ButtonsType::None,
                            format!("File {} does not exist!", file.to_string_lossy()).as_str(),
                        );
                        dialog.run();
                    } else {
                        self.model.db = Database::from(file.to_str().unwrap()).unwrap();
                        update_treestore(&self.model.db, &self.model.tree_store);
                    }
                }
                dialog.destroy();
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("window.glade");
        let builder = Builder::new_from_string(glade_src);

        //Load glade items
        let window: Window = builder.get_object("window").unwrap();
        let open: MenuItem = builder.get_object("menu_open").unwrap();
        let text_viewer = builder.get_object("text_viewer").unwrap();
        let tree_view: TreeView = builder.get_object("tree_view").unwrap();
        let col: TreeViewColumn = builder.get_object("view_column").unwrap();

        //Setup tree view
        let cell = gtk::CellRendererText::new();
        col.pack_start(&cell, true);
        col.add_attribute(&cell, "text", 0);
        cell.set_property("editable", &true)
            .expect("failed to set editable");
        tree_view.set_model(Some(&model.tree_store));

        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );
        connect!(
            relm,
            tree_view,
            connect_cursor_changed(_),
            Msg::SelectedItem
        );
        connect!(relm, open, connect_activate(_), Msg::MenuOpen);

        Win {
            model,
            tree_view,
            window,
            text_viewer,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
