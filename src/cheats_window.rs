use gtk::{gio, gdk, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::cheat_object::CheatObject;

//------------------------------------------------------------------------------
// MODULE: CheatsWindow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/DHXS-Launcher/ui/cheats_window.ui")]
    pub struct CheatsWindow {
        #[template_child]
        pub(super) model: TemplateChild<gio::ListStore>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for CheatsWindow {
        const NAME: &'static str = "CheatsWindow";
        type Type = super::CheatsWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            CheatObject::ensure_type();

            klass.bind_template();

            //---------------------------------------
            // Add class key bindings
            //---------------------------------------
            // Close window binding
            klass.add_binding_action(gdk::Key::Escape, gdk::ModifierType::NO_MODIFIER_MASK, "window.close");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CheatsWindow {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_widgets();
        }
    }

    impl WidgetImpl for CheatsWindow {}
    impl WindowImpl for CheatsWindow {}
    impl AdwWindowImpl for CheatsWindow {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: CheatsWindow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct CheatsWindow(ObjectSubclass<imp::CheatsWindow>)
        @extends adw::Window, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl CheatsWindow {
    //-----------------------------------
    // Setup widgets
    //-----------------------------------
    fn setup_widgets(&self) {
        let imp = self.imp();

        // Populate grid
        let cheats: Vec<CheatObject> = [
            ("IDBEHOLDA", "Automap"),
            ("IDBEHOLDI", "Temporary invisibility"),
            ("IDBEHOLDL", "Light amplification goggles"),
            ("IDBEHOLDR", "Radiation suit"),
            ("IDBEHOLDS", "Berserk pack"),
            ("IDBEHOLDV", "Temporary invulnerability"),
            ("IDCHOPPERS", "Chainsaw"),
            ("IDCLEV##", "Warp to episode #, map #"),
            ("IDCLIP", "No clipping (walk through objects)"),
            ("IDDQD", "God mode (invincibility)"),
            ("IDDT", "Display entire map and enemies (toggle)"),
            ("IDFA", "All weapons and 200% armor"),
            ("IDKFA", "All keys and weapons"),
            ("IDMYPOS", "Display location")
        ]
        .iter()
        .map(|(code, description)| {
            CheatObject::new(code, description)
        })
        .collect();

        imp.model.splice(0, 0, &cheats);
    }
}

impl Default for CheatsWindow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
