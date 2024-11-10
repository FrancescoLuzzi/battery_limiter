use battery_limiter::battery_level::BatteryLevel;
use battery_limiter::service::BatteryLimiterService;
use gtk::gio::*;
use gtk::prelude::*;
use gtk::{gdk, glib};
use libadwaita as adw;
use libadwaita::prelude::AdwApplicationWindowExt;
use std::io;
use std::sync::Arc;

const MENU_ICON_NAME: &str = "open-menu-symbolic";

pub struct AppContext {
    pub main_window: adw::ApplicationWindow,
    pub toast_overlay: adw::ToastOverlay,
}
unsafe impl Send for AppContext {}
unsafe impl Sync for AppContext {}

fn main() -> glib::ExitCode {
    let application = adw::Application::builder()
        .application_id("com.github.battery_limiter")
        .build();
    application.connect_startup(|_| load_css());
    application.connect_activate(build_ui);
    application.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("../styles/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn add_list_view<T: BoxExt>(ctx: Arc<AppContext>, window: &T) {
    let list_box = gtk::ListBox::new();
    let curr_level = BatteryLevel::from_system()
        .expect("something went wrong getting current battery threshold");
    let (sender, receiver) = async_channel::bounded::<i32>(1);
    for (index, lvl) in [BatteryLevel::Low, BatteryLevel::Medium, BatteryLevel::Full]
        .into_iter()
        .enumerate()
    {
        let sender_clone = sender.clone();
        list_box.append(&create_battery_setter(ctx.clone(), lvl, move || {
            sender_clone
                .send_blocking(index as i32)
                .expect("The channel needs to be open.");
        }));
        if curr_level.get_percentage() == lvl.get_percentage() {
            sender
                .send_blocking(index as i32)
                .expect("The channel needs to be open.");
        }
    }
    glib::spawn_future_local(glib::clone!(
        #[weak]
        list_box,
        async move {
            while let Ok(index) = receiver.recv().await {
                list_box.select_row(list_box.row_at_index(index).as_ref());
            }
        }
    ));
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .hexpand(true)
        .child(&list_box)
        .build();
    window.append(&scrolled_window);
}

fn create_battery_setter<F>(ctx: Arc<AppContext>, bat_lvl: BatteryLevel, callback: F) -> gtk::Box
where
    F: Fn() + Clone + Send + Sync + 'static,
{
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(20)
        .margin_start(20)
        .margin_end(20)
        .margin_top(20)
        .margin_bottom(20)
        .build();
    let label = gtk::Label::new(Some(&format!(
        "Set max recharge level to {}%",
        bat_lvl.get_percentage()
    )));
    let apply_button = gtk::Button::new();
    apply_button.set_icon_name(bat_lvl.get_gtk_icon_name());
    apply_button.set_tooltip_text(Some("apply change"));
    apply_button.connect_clicked(move |button| {
        let ctx = ctx.clone();
        let callback = callback.clone();
        let button = button.to_owned();
        glib::spawn_future_local(async move {
            button.set_sensitive(false);
            let res = match bat_lvl.apply().await {
                Ok(percentage) => {
                    let service = BatteryLimiterService::new(percentage);
                    service.persist().await
                }
                Err(e) => Err(e),
            };
            let msg = match res {
                Ok(_) => {
                    callback();
                    "modification applied"
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::PermissionDenied => "permission denied",
                    io::ErrorKind::NotFound => "file not found",
                    _ => "modification failed",
                },
            };
            button.set_sensitive(true);
            ctx.toast_overlay
                .add_toast(adw::Toast::builder().timeout(2).title(msg).build());
        });
    });
    container.append(&label);
    container.append(&apply_button);
    container
}

fn build_ui(application: &adw::Application) {
    let main_window = adw::ApplicationWindow::new(application);
    let toast_overlay = adw::ToastOverlay::new();

    main_window.set_default_size(400, 400);
    main_window.set_title(Some("Battery Limiter"));
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .spacing(0)
        .build();
    toast_overlay.set_child(Some(&container));
    main_window.set_content(Some(&toast_overlay));

    let ctx = Arc::new(AppContext {
        main_window: main_window.clone(),
        toast_overlay,
    });

    container.append(&create_and_setup_menu(ctx.clone()));
    add_list_view(ctx.clone(), &container);

    main_window.present();
}

fn create_and_setup_menu(ctx: Arc<AppContext>) -> adw::HeaderBar {
    let header_bar = adw::HeaderBar::new();
    let menu = Menu::new();
    let action_about = ActionEntry::builder("about")
        .activate(|_, _, _| {
            show_about_dialog();
        })
        .build();
    let about_item = MenuItem::new(Some("about"), Some("win.about"));
    menu.append_item(&about_item);

    let menu_button = gtk::MenuButton::new();
    menu_button.set_icon_name(MENU_ICON_NAME);
    menu_button.set_menu_model(Some(&menu));
    header_bar.pack_start(&menu_button);
    ctx.main_window.add_action_entries([action_about]);
    header_bar
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
