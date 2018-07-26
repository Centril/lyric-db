#![feature(use_extern_macros)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate treexml;

mod database;
use database::metadata;
use database::Database;

use gtk::prelude::*;
use gtk::{Label, Orientation::Horizontal, Paned, TreeStore, TreeView, Window, WindowType};

use relm::{Relm, Update, Widget};

fn create_treestore(db: &Database) -> gtk::TreeStore {
    let out = TreeStore::new(&[String::static_type()]);

    for artist in &db.entries {
        let iter = out.insert_with_values(None, None, &[0], &[&artist.name]);

        for album in &artist.albums {
            let iter = out.insert_with_values(Some(&iter), None, &[0], &[&album.title]);

            for track in &album.tracks {
                out.insert_with_values(Some(&iter), None, &[0], &[&track.title]);
            }
        }
    }
    out
}

#[derive(Msg)]
pub enum Msg {
    SelectedItem,
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
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        let database = Database::from("testfiles/test.xml").unwrap();
        let tree_store = create_treestore(&database);
        Model {
            db: database,
            tree_store,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SelectedItem => {
                let selection = self.tree_view.get_selection();
                if let Some((model, iter)) = selection.get_selected() {
                    println!("{}", model.get_path(&iter).expect("Failed to get path"));
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
        let window = Window::new(WindowType::Toplevel);
        let pane = Paned::new(Horizontal);

        let col = gtk::TreeViewColumn::new();
        let cell = gtk::CellRendererText::new();
        let tree_view = gtk::TreeView::new();
        let text_viewer = gtk::Label::new("");

        col.pack_start(&cell, true);
        col.add_attribute(&cell, "text", 0);
        tree_view.append_column(&col);
        tree_view.set_model(Some(&model.tree_store));

        pane.add1(&tree_view);
        pane.add2(&text_viewer);

        window.add(&pane);
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

        Win {
            model,
            tree_view,
            window,
            pane,
            text_viewer,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
