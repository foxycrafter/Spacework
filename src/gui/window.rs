use gio::prelude::*;
  use gtk::prelude::*;

  pub fn start_gui() {
      let application = gtk::Application::new(
          Some("com.example.tabs"),
          gio::ApplicationFlags::FLAGS_NONE,
      )
      .expect("Failed to create GTK application");

      application.connect_activate(|app| {
          let window = gtk::ApplicationWindow::new(app);
          window.set_default_size(640, 480);

          let notebook = gtk::Notebook::new();
          let label1 = gtk::Label::new(Some("Pestaña 1"));
          let child1 = gtk::Box::new(gtk::Orientation::Vertical, 0);
          child1.add(&gtk::Label::new(Some("Contenido de la pestaña 1")));
          notebook.append_page(&child1, Some(&label1));

          let label2 = gtk::Label::new(Some("Pestaña 2"));
          let child2 = gtk::Box::new(gtk::Orientation::Vertical, 0);
          child2.add(&gtk::Label::new(Some("Contenido de la pestaña 2")));
          notebook.append_page(&child2, Some(&label2));

          let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
          vbox.pack_start(&notebook, true, true, 0);

          window.add(&vbox);

          window.show_all();
      });

      application.run(&[]);
  }