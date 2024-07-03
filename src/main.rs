use gtk::{gdk, glib, Application, ApplicationWindow};
use gtk::{prelude::*, Button};

mod config;
mod consts;
mod theme;
use consts::*;

fn main() -> glib::ExitCode {
    if cfg!(debug_assertions) {
        println!("Debug mode enabled");
    }
    config::init();
    if cfg!(debug_assertions) {
        println!("Active: {:?}", config::active_path());
        println!("System: {:?}", config::sys_path());
        println!("User: {:?}", config::user_path());
    }

    // Build the app
    let app = Application::builder().application_id(APP_ID).build();

    // Load the style provider
    app.connect_startup(|_| {
        let css = gtk::CssProvider::new();
        css.load_from_string(include_str!("style.css"));
        gtk::style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &css, 800);
    });

    // Build the UI
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
    for action in config::actions() {
        add_button(&hbox, action);
    }

    window.set_child(Some(&hbox));

    // Configure action controller
    window.add_controller(setup_key_press_controller(config::actions()));

    window.present();
}

// Configure the key press controller to handle the configured actions
fn setup_key_press_controller(actions: Vec<config::Action>) -> gtk::EventControllerKey {
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_ctrl, key, _, _| {
        if key == gdk::Key::Escape {
            std::process::exit(0);
        }

        // Execute any actions by key, checking upper and lower case for thoroughness
        for action in &actions {
            if let Some(k) = gdk::Key::from_name(action.key()) {
                if key == k {
                    action.run();
                    break;
                }
            }
            if let Some(k) = gdk::Key::from_name(action.key().to_lowercase()) {
                if key == k {
                    action.run();
                    break;
                }
            }
        }

        // let win: ApplicationWindow = ctrl.widget().dynamic_cast().unwrap();
        // win.close();

        glib::Propagation::Stop
    });
    key_controller
}

// Add a button with the given label to the given container and call the given callback function
// when the button is triggered.
fn add_button(container: &gtk::Box, action: config::Action) {
    let label = action.name();
    let hotkey = action.key();

    // Build the box to place the button and the label
    let btn_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        //.spacing(SPACING)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    //let file = gio::File::for_path("./examples/clipboard/asset.png");
    //let asset_paintable = gdk::Texture::from_file(&file).unwrap();
    let bytes = glib::Bytes::from_static(LOGOUT_SVG);
    let logo = gdk::Texture::from_bytes(&bytes).expect("to load icon");
    let image = gtk::Image::builder()
        .pixel_size(config::icon_size())
        .paintable(&logo)
        .build();

    // Build the button and add the callback
    let btn = Button::builder().has_frame(false).child(&image).build();
    btn.connect_clicked(move |_| action.run());
    btn_box.append(&btn);

    // Build the label and add it to the box
    let btn_label = gtk::Label::default();
    btn_label.set_markup(
        format!(
            "<span color=\"{}\" font=\"{}\">{} ({})</span>",
            config::font_color(),
            config::font_size(),
            label,
            hotkey
        )
        .as_str(),
    );

    btn_box.append(&btn_label);
    container.append(&btn_box);
}
