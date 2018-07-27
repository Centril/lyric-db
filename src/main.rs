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
    ButtonsType, DialogFlags, FileChooserAction, FileChooserDialog, Label, Menu, MenuBar, MenuItem,
    MessageDialog, MessageType, Orientation, Paned, TreeStore, TreeView, Window, WindowType,
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
    pane: Paned,
    text_viewer: Label,
    menu_bar: MenuBar,
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
        //Window and layouts
        let window = Window::new(WindowType::Toplevel);
        let pane = Paned::new(Orientation::Horizontal);
        let g_box = gtk::Box::new(Orientation::Vertical, 1);

        let text_viewer = gtk::Label::new("");

        //Setup menu bar
        let menu = Menu::new();
        let menu_bar = MenuBar::new();
        let file = MenuItem::new_with_label("File");
        let open = MenuItem::new_with_label("Open...");

        menu.append(&open);
        file.set_submenu(Some(&menu));
        menu_bar.append(&file);
        menu_bar.show_all();

        //Setup tree view
        let col = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        let tree_view = gtk::TreeView::new();
        col.pack_start(&cell, true);
        col.add_attribute(&cell, "text", 0);
        tree_view.append_column(&col);
        tree_view.set_model(Some(&model.tree_store));

        g_box.add(&menu_bar);
        g_box.add(&pane);

        pane.add1(&tree_view);
        pane.add2(&text_viewer);

        window.add(&g_box);
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
            pane,
            text_viewer,
            menu_bar,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
