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
use gtk::{Orientation::Horizontal, Paned, TreeStore, TreeView, Window, WindowType};

use relm::{Relm, Update, Widget};

fn create_treestore(db: &Database) -> gtk::TreeStore {
    let out = TreeStore::new(&[String::static_type()]);

    for (artist, albums) in &db.entries {
        println!("woo, {}", artist);
        let iter = out.insert_with_values(None, None, &[0], &[&artist.name]);

        for (album, tracks) in albums {
            let iter = out.insert_with_values(Some(&iter), None, &[0], &[&album.title]);

            for track in tracks {
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

        col.pack_start(&cell, true);
        col.add_attribute(&cell, "text", 0);
        tree_view.append_column(&col);
        tree_view.set_model(Some(&model.tree_store));

        pane.add1(&tree_view);

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

        for (k, v) in model.db.entries.iter() {
            println!("artist: {:?}", k);
            for (album, tracks) in v.iter() {
                println!("album: {:?}", album);
                tracks.iter().for_each(|t| println!("{:?}", t));
            }
        }

        Win {
            model,
            tree_view,
            window,
            pane,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
