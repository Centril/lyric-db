use gtk::prelude::*;
use gtk::{
    Builder, Button, ButtonsType, DialogFlags, FileChooserAction, FileChooserDialog, Label, Menu,
    MenuItem, MessageDialog, MessageType, TreeStore, TreeView, TreeViewColumn, Window,
};

use relm::{init, Component, Relm, Update, Widget};

use std::path::Path;

use database::Database;

use albumwindow::AlbumWindow;

fn update_treestore(db: &mut Database, input: &TreeStore) {
    input.clear();
    for artist in &db.entries {
        let iter = input.insert_with_values(None, None, &[0], &[&artist.name]);

        for album in &artist.albums {
            let iter = input.insert_with_values(Some(&iter), None, &[0], &[&album.title]);

            for track in &album.tracks {
                input.insert_with_values(
                    Some(&iter),
                    None,
                    &[0, 1],
                    &[&track.title, &track.lyrics],
                );
            }
        }
    }
    db.clean();
}

#[derive(Msg)]
pub enum Msg {
    SelectedItem,
    MenuOpen,
    AddArtist,
    EditEntry,
    Quit,
}

pub struct Model {
    db: Database,
    tree_store: gtk::TreeStore,
}

pub struct MainWindow {
    tree_view: TreeView,
    model: Model,
    window: Window,
    text_viewer: Label,
    albumwin: Option<Component<AlbumWindow>>,
    context_menu: Menu,
}

impl Update for MainWindow {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    //Return empty model
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            db: Database::empty(),
            tree_store: TreeStore::new(&[
                String::static_type(),
                String::static_type(),
                i32::static_type(),
            ]),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::SelectedItem => {
                let selection = self.tree_view.get_selection();
                if let Some((model, iter)) = selection.get_selected() {
                    let mut path = model.get_path(&iter).expect("failed to get path");

                    if path.get_depth() != 3 {
                        return;
                    }

                    //TODO this leaks memory, fix
                    if let Some(lyrics) = model.get_value(&iter, 1).get::<String>() {
                        self.text_viewer.set_text(&lyrics);
                    } else {
                        self.text_viewer.set_text("");
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
                        // self.model.db.save("").unwrap();
                        update_treestore(&mut self.model.db, &self.model.tree_store);
                    }
                }
                dialog.destroy();
            }
            Msg::AddArtist => {
                //TODO: pop up dialog to ask for name
                self.model
                    .tree_store
                    .insert_with_values(None, None, &[0], &[&String::new()]);
            }
            Msg::EditEntry => {
                //TODO
                self.albumwin = Some(init::<AlbumWindow>(()).expect("album window"));
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for MainWindow {
    type Root = Window;
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("mainwindow.glade");
        let builder = Builder::new_from_string(glade_src);

        //Load glade items
        let window: Window = builder.get_object("window").unwrap();
        let open: MenuItem = builder.get_object("menu_open").unwrap();
        let text_viewer = builder.get_object("text_viewer").unwrap();
        let tree_view: TreeView = builder.get_object("tree_view").unwrap();
        let col_name: TreeViewColumn = builder.get_object("view_column").unwrap();
        let col_lyrics: TreeViewColumn = builder.get_object("lyric_column").unwrap();
        let button_add_artist: Button = builder.get_object("button_add_artist").unwrap();

        //Context menu
        let context_menu: Menu = builder.get_object("context_menu").unwrap();
        let context_edit: MenuItem = builder.get_object("context_menu_edit").unwrap();

        //Setup tree view
        let cell_name = gtk::CellRendererText::new();
        col_name.pack_start(&cell_name, true);
        col_name.add_attribute(&cell_name, "text", 0);

        let cell_lyrics = gtk::CellRendererText::new();
        col_lyrics.pack_start(&cell_lyrics, true);
        col_lyrics.add_attribute(&cell_lyrics, "text", 0);

        cell_name
            .set_property("editable", &true)
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
        connect!(relm, button_add_artist, connect_activate(_), Msg::AddArtist);
        connect!(relm, context_edit, connect_activate(_), Msg::EditEntry);

        //Connections that cant be done with relm
        let con_menu = context_menu.clone();
        tree_view.connect_button_press_event(move |view, event| {
            if view.get_selection().get_selected().is_some() && event.get_button() == 3 {
                con_menu.popup_easy(event.get_button(), event.get_time());
            }
            Inhibit(false)
        });
        let model1 = model.tree_store.clone();
        cell_name.connect_edited(move |_, path, string| {
            let iter = model1.get_iter(&path).unwrap();
            model1.set(&iter, &[0], &[&string.to_owned()]);
        });

        MainWindow {
            model,
            tree_view,
            window,
            text_viewer,
            context_menu,
            albumwin: None,
        }
    }
}
