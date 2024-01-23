use gtk4 as gtk;

use glib::clone;
use gtk::glib;

use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.aprilJade.rs-calculator"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Calculator"));

    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(80)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(0)
        .column_spacing(0)
        .can_focus(false)
        .build();
    window.set_child(Some(&grid));

    let btn_len = 1;
    let button_0 = gtk::Button::with_label("0");
    let button_1 = gtk::Button::with_label("1");
    let button_2 = gtk::Button::with_label("2");
    let button_3 = gtk::Button::with_label("3");
    let button_4 = gtk::Button::with_label("4");
    let button_5 = gtk::Button::with_label("5");
    let button_6 = gtk::Button::with_label("6");
    let button_7 = gtk::Button::with_label("7");
    let button_8 = gtk::Button::with_label("8");
    let button_9 = gtk::Button::with_label("9");
    let button_cancel = gtk::Button::with_label("AC");
    let button_sign = gtk::Button::with_label("+/-");
    let button_modular = gtk::Button::with_label("%");
    let button_divider = gtk::Button::with_label("÷");
    let button_multiplier = gtk::Button::with_label("x");

    // grid.attach(&button, 0, 0, 1, 1);

    window.show();
}
