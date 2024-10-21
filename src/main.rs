use gtk::gio::*;
use gtk::glib;
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.battery_limiter")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_default_size(400, 400);
    window.set_title(Some("Battery Limiter"));
    let action_about = ActionEntry::builder("about")
        .activate(|_, _, _| {
            show_about_dialog();
        })
        .build();

    add_menu(&window);
    let container = gtk::Box::new(gtk::Orientation::Vertical, 6);
    window.set_child(Some(&container));

    window.add_action_entries([action_about]);
    window.present();
}

fn add_menu(window: &gtk::ApplicationWindow) {
    let header_bar = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));
    let menu = Menu::new();

    // Add a MenuItem for "About"
    let about_item = MenuItem::new(Some("about"), Some("win.about"));
    menu.append_item(&about_item);

    let menu_button = gtk::MenuButton::new();
    menu_button.set_icon_name("open-menu-symbolic");
    menu_button.set_menu_model(Some(&menu));
    header_bar.pack_start(&menu_button);
}

fn show_about_dialog() {
    let dialog = gtk::AboutDialog::builder()
        .modal(true)
        .program_name("Battery Limiter")
        .version("0.1.0")
        .website("https://github.com/FrancescoLuzzi/battery_limiter")
        .license_type(gtk::License::MitX11)
        .authors(["Francesco Luzzi"])
        .build();
    dialog.present();
}
