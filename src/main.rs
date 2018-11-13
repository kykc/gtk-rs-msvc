#![windows_subsystem = "windows"]

extern crate gtk;
extern crate gio;

use gtk::{Button, ApplicationWindow, Builder, Entry, SettingsExt};
use gtk::prelude::*;
use gio::prelude::*;
use gio::ApplicationExt;
use std::env::args;
use std::cell::RefCell;

struct State {
    pub counter: u32,
}

impl Default for State {
    fn default() -> Self {
        State {counter: 0u32}
    }
}

thread_local!(
    static GLOBAL_STATE: RefCell<State> = RefCell::new(State::default());
);

macro_rules! gtk_clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(gtk_clone!(@param $p),)+| $body
        }
    );
}

pub fn build_ui(application: &gtk::Application) {
    gtk::Settings::get_default().unwrap().set_property_gtk_application_prefer_dark_theme(true);

    let builder = Builder::new_from_string(include_str!("main.glade"));

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    let btn_increment: Button = builder.get_object("btnIncrement").expect("btnIncrement not found");
    let entry: Entry = builder.get_object("txtCounter").expect("entry not found");

    window.set_application(application);

    window.connect_delete_event(gtk_clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let increment_pair = (btn_increment.clone(), entry.clone());

    btn_increment.connect_clicked(gtk_clone!(increment_pair => move |_| {
        GLOBAL_STATE.with(|state| {
            state.borrow_mut().counter += 1u32;
            increment_pair.1.set_text(&state.borrow().counter.to_string());
        });
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.automatl.gtk-rs-msvc", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});
    application.run(&args().collect::<Vec<_>>());
}
