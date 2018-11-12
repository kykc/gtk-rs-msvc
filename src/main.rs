#![windows_subsystem = "windows"]

extern crate gtk;

use gtk::{Button, Window, WindowType};
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    gtk::Settings::get_default().unwrap().set_property_gtk_application_prefer_dark_theme(true);

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    gtk::main();
}
