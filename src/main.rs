use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

mod actions;
mod config;
mod consts;
use actions::*;
use consts::*;

fn main() -> glib::ExitCode {
    config::init();

    let app = Application::builder().application_id(APP_NAME).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_NAME)
        .default_width(350)
        .default_height(200)
        .opacity(config::opacity())
        .decorated(false)
        .fullscreened(true)
        .build();

    // Configure buttons
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(SPACING)
        .margin_start(SPACING)
        .margin_end(SPACING)
        .margin_bottom(SPACING)
        .spacing(SPACING)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    // Configure action buttons
    add_button("logout", &hbox, |_| Action::Logout.run());
    add_button("restart", &hbox, |_| Action::Restart.run());
    add_button("shutdown", &hbox, |_| Action::Shutdown.run());
    add_button("suspend", &hbox, |_| Action::Suspend.run());
    add_button("lock", &hbox, |_| Action::Lock.run());

    window.set_child(Some(&hbox));

    // Configure action controller
    let actions = gtk::EventControllerKey::new();
    actions.connect_key_pressed(|_ctrl, key, _, _| {
        Action::from(key).run();
        // let win: ApplicationWindow = ctrl.widget().dynamic_cast().unwrap();
        // win.close();
        glib::Propagation::Proceed
    });
    window.add_controller(actions);

    window.present();
}

// Add a button with the given label to the given container and call the given callback function
// when the button is triggered.
fn add_button<F: Fn(&gtk::Button) + 'static>(label: &str, container: &gtk::Box, callback: F) {
    // Build the box to place the button and the label
    let btn_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACING)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    // Build the button and add the callback
    let lock_btn = Button::with_label(label);
    lock_btn.connect_clicked(callback);
    btn_box.append(&lock_btn);

    // Build the label and add it to the box
    let btn_label = gtk::Label::builder()
        .label(label)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    btn_box.append(&btn_label);
    container.append(&btn_box);
}
