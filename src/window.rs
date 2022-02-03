use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/vt/WSNFN/window.ui")]
    pub struct WirelessSensorNodeFrontendWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WirelessSensorNodeFrontendWindow {
        const NAME: &'static str = "WirelessSensorNodeFrontendWindow";
        type Type = super::WirelessSensorNodeFrontendWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WirelessSensorNodeFrontendWindow {}
    impl WidgetImpl for WirelessSensorNodeFrontendWindow {}
    impl WindowImpl for WirelessSensorNodeFrontendWindow {}
    impl ApplicationWindowImpl for WirelessSensorNodeFrontendWindow {}
}

glib::wrapper! {
    pub struct WirelessSensorNodeFrontendWindow(ObjectSubclass<imp::WirelessSensorNodeFrontendWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl WirelessSensorNodeFrontendWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create WirelessSensorNodeFrontendWindow")
    }
}
