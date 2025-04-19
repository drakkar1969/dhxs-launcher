use std::cell::RefCell;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::folder_select_row::FolderSelectRow;

//------------------------------------------------------------------------------
// MODULE: PreferencesDialog
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::PreferencesDialog)]
    #[template(resource = "/com/github/DHXS-Launcher/ui/preferences_dialog.ui")]
    pub struct PreferencesDialog {
        #[template_child]
        pub(super) iwad_row: TemplateChild<FolderSelectRow>,
        #[template_child]
        pub(super) pwad_row: TemplateChild<FolderSelectRow>,

        #[template_child]
        pub(super) reset_button: TemplateChild<adw::ButtonRow>,

        #[property(get, set)]
        iwad_folder: RefCell<String>,
        #[property(get, set)]
        pwad_folder: RefCell<String>,

        #[property(get, set)]
        iwad_default_folder: RefCell<String>,
        #[property(get, set)]
        pwad_default_folder: RefCell<String>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesDialog {
        const NAME: &'static str = "PreferencesDialog";
        type Type = super::PreferencesDialog;
        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PreferencesDialog {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_widgets();
            obj.setup_signals();
        }
    }

    impl WidgetImpl for PreferencesDialog {}
    impl AdwDialogImpl for PreferencesDialog {} 
    impl PreferencesDialogImpl for PreferencesDialog {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: PreferencesDialog
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct PreferencesDialog(ObjectSubclass<imp::PreferencesDialog>)
        @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::ShortcutManager;
}

impl PreferencesDialog {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    //---------------------------------------
    // Setup widgets
    //---------------------------------------
    fn setup_widgets(&self) {
        let imp = self.imp();

        // Bind properties to widgets
        self.bind_property("iwad-folder", &imp.iwad_row.get(), "folder")
            .sync_create()
            .bidirectional()
            .build();

        self.bind_property("pwad-folder", &imp.pwad_row.get(), "folder")
            .sync_create()
            .bidirectional()
            .build();

        self.bind_property("iwad-default-folder", &imp.iwad_row.get(), "default-folder")
            .sync_create()
            .bidirectional()
            .build();

        self.bind_property("pwad-default-folder", &imp.pwad_row.get(), "default-folder")
            .sync_create()
            .bidirectional()
            .build();
    }

    //-----------------------------------
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Preferences reset button clicked signal
        imp.reset_button.connect_activated(clone!(
            #[weak(rename_to = window)] self,
            #[weak] imp,
            move |_| {
                let reset_dialog = adw::AlertDialog::builder()
                    .heading("Reset Paths?")
                    .body("Reset all paths to their default values.")
                    .default_response("reset")
                    .build();

                reset_dialog.add_responses(&[("cancel", "_Cancel"), ("reset", "_Reset")]);
                reset_dialog.set_response_appearance("reset", adw::ResponseAppearance::Destructive);

                reset_dialog.choose(
                    &window,
                    None::<&gio::Cancellable>,
                    clone!(
                        #[weak] imp,
                        move |response| {
                            if response == "reset" {
                                imp.iwad_row.reset_to_default();
                                imp.pwad_row.reset_to_default();
                            }
                        }
                    )
                );
            }
        ));
    }
}

impl Default for PreferencesDialog {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        Self::new()
    }
}
