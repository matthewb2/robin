use gio::SimpleAction;
use glib::{clone, Priority};
use gtk::gio::SimpleActionGroup;
use gtk::prelude::*;
use gtk::gdk::Display;
use gtk::{gio, glib, Application, ApplicationWindow,
			FileChooserDialog, FileChooserAction, ResponseType,
			STYLE_PROVIDER_PRIORITY_APPLICATION, CssProvider

};
use sourceview5::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;



// ANCHOR: main
const APP_ID: &str = "org.gtk_rs.Actions2";

fn main() -> glib::ExitCode {
    // Create a new application
    let application = Application::builder().application_id(APP_ID).build();
	   application.connect_activate(|_app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        build_ui(_app);
    });

    // Set keyboard accelerator to trigger "win.close".
    application.set_accels_for_action("win.close", &["<Ctrl>C"]);
	application.set_accels_for_action("win.open", &["<Ctrl>O"]);
	application.set_accels_for_action("win.save", &["<Ctrl>S"]);
    // Run the application
    application.run()
}
// ANCHOR_END: main

/*
fn actions_added(some_vec: Vec<SimpleAction>, actions: SimpleActionGroup){
	for i in &some_vec {
		actions.add_action(i);
	}
	
}
*/
// ANCHOR: build_ui
fn build_ui(app: &Application) {
	
	let buffer = sourceview5::Buffer::new(None);
	//buffer.set_text("test");
    buffer.set_highlight_syntax(true);
    if let Some(ref language) = sourceview5::LanguageManager::new().language("rust") {
        buffer.set_language(Some(language));
    }
    if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("solarized-light") {
        buffer.set_style_scheme(Some(scheme));
    }
	
	    let file = gtk::gio::File::for_path("buffer.rs");
    let file = sourceview5::File::builder().location(&file).build();
    let loader = sourceview5::FileLoader::new(&buffer, &file);
	loader.load_async_with_callback(
        Priority::default(),
        gtk::gio::Cancellable::NONE,
        move |current_num_bytes, total_num_bytes| {
            println!(
                "loading: {:?}",
                (current_num_bytes as f32 / total_num_bytes as f32) * 100f32
            );
        },
        |res| {
            println!("loaded: {:?}", res);
        },
    );

    
	let view = sourceview5::View::with_buffer(&buffer);
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
		.build();
	
	window.set_default_size(650, 600);
	window.set_title(Some("Robin Editor"));
    // Add action "close" to `window` taking no parameter
    let action_close = SimpleAction::new("close", None);
    action_close.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    
	
	let action_open = SimpleAction::new("open", None);
	let action_save = SimpleAction::new("save", None);
	/*
	let window_save = window.as_ref();
	
	action_save.connect_activate( move |_, _| {
		 let file_chooser = FileChooserDialog::new(
            Some("Save File"),
            Some(*window_save),
            FileChooserAction::Open,
            &[("저장", ResponseType::Ok), ("취소", ResponseType::Cancel)],
        );
		
	});
	*/
	
	//save action
	action_save.connect_activate(clone!(@weak window, @weak buffer => move |_, _| {
		let file_chooser = FileChooserDialog::new(
            Some("Save File"),
            Some(&window),
            FileChooserAction::Open,
            &[("저장", ResponseType::Ok), ("취소", ResponseType::Cancel)],
        );
		let buffer = buffer.clone();
		file_chooser.connect_response(clone!(@weak window, @weak buffer => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {
                let file = file_chooser.file().expect("Couldn't get filename");
				    let file = sourceview5::File::builder().location(&file).build();
					let saver = sourceview5::FileSaver::new(&buffer, &file);
						saver.save_async_with_callback(
						Priority::default(),
						gtk::gio::Cancellable::NONE,
						move |current_num_bytes, total_num_bytes| {
							println!(
								"saving: {:?}",
								(current_num_bytes as f32 / total_num_bytes as f32) * 100f32
							);
						},
						|res| {
							println!("saved: {:?}", res);
						},
					);
				
            
            }
            file_chooser.destroy();
        }));
		
		file_chooser.show();
		println!("saved");
	}));
	
	//open action
	
	action_open.connect_activate(clone!(@weak window, @weak buffer => move |_, _| {
        let file_chooser = FileChooserDialog::new(
            Some("파일 열기"),
            Some(&window),
            FileChooserAction::Open,
            &[("열기", ResponseType::Ok), ("취소", ResponseType::Cancel)],
        );
		
		file_chooser.connect_response(clone!(@strong window, @weak buffer => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {
                let file = file_chooser.file().expect("Couldn't get filename");
				/*
				let filename: &PathBuf = match file.path().as_ref() {
					Some(s) => s, // deref coercion
					None  =>
				};
				
				println!("{:?}", filename);
				*/
				//let filename = file.path();
				
				let filename = file.path().expect("REASON").display().to_string();
				println!("{:?}", filename);
				let file = File::open(&filename).expect("Couldn't open file");
				
				let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);
				
				&buffer.set_text(&contents);
            
            }
            file_chooser.close();
        }));
		
        file_chooser.show();
    }));
	
	
	window.add_action(&action_open);
	window.add_action(&action_save);
	window.add_action(&action_close);

    // ANCHOR: action_group
    // Create a new action group and add actions to it
    let actions = SimpleActionGroup::new();
    window.insert_action_group("win", Some(&actions));
    let mut action_vec = vec![action_open, action_save, action_close];
	//actions_added(action_vec.clone(), actions.clone());
	action_vec.iter_mut().for_each(|e| actions.add_action(e));
    // ANCHOR_END: action_group

	let header_bar = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));
	
	let button1 = gtk::MenuButton::new();
	let menumodel = gio::Menu::new();
		menumodel.append(Some("열기"), Some("win.open"));
		menumodel.append(Some("저장"), Some("win.save"));
		menumodel.append(Some("닫기"), Some("win.close"));
	
	button1.set_icon_name("open-menu-symbolic");
	button1.set_menu_model(Some(&menumodel));
	header_bar.pack_end(&button1);
	
	let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
	let scrolled_window = gtk::ScrolledWindow::new();
	
    
	view.add_css_class("sourceview5");
    view.set_monospace(true);
    //view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
    view.set_show_line_numbers(true);
    view.set_highlight_current_line(true);
    view.set_tab_width(4);
    view.set_hexpand(true);
    container.append(&view);
    let map = sourceview5::Map::new();
    map.set_view(&view);
    container.append(&map);
	scrolled_window.set_child(Some(&container));
	window.set_child(Some(&scrolled_window));
    // Present window
    window.present();
}
// ANCHOR: build_ui
