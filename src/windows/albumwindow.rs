use gtk::prelude::*;
use gtk::{Builder, Label, ListStore, Window, WindowType};

use relm::{Relm, Update, Widget};

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

pub struct AlbumWindow {
    window: Window,
}

impl Update for AlbumWindow {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
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
