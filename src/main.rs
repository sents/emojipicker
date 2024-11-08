use gtk::gdk::Clipboard;
use gtk::glib;
use gtk::prelude::{BoxExt, DisplayExt, EditableExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::actions::{AccelsPlus, RelmAction, RelmActionGroup};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
    RelmWidgetExt, SimpleComponent,
};

use emojis::Emoji;
mod components;
use components::emoji_picker::{EmojiPicker, EmojiPickerInput};

mod debounce;
use debounce::make_debounce;

relm4::new_action_group!(WindowActionGroup, "win");
relm4::new_stateless_action!(FocusSearchAction, WindowActionGroup, "focus_search");

struct AppModel {
    emojis: Controller<EmojiPicker>,
    clipboard: Clipboard,
}

#[derive(Debug)]
enum AppMsg {
    SearchEmojis(String),
    PickEmoji(&'static Emoji),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            // set_class_active: ("identical", model.identical),
            set_title: Some("Emoji Picker"),
            set_default_width: 300,
            set_default_height: 600,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                #[name="search_entry"]
                gtk::SearchEntry {
                    connect_changed[sender, debounced_search] => move |entry| {
                        debounced_search((
                            sender.clone(),
                            AppMsg::SearchEmojis(entry.text().to_string())
                        ))
                    },
                },
                gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_propagate_natural_height: true,
                    set_max_content_height: 20,
                    #[local_ref]
                    emoji_box -> gtk::Box {
                    }
                }
            }
        }
    }

    fn init(
        _params: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let emojis = EmojiPicker::builder()
            .launch(())
            .forward(sender.input_sender(), |output: &'static Emoji| {
                AppMsg::PickEmoji(output)
            });

        let display = root.display();
        let clipboard = display.clipboard();

        let model = AppModel { clipboard, emojis };

        let emoji_box = model.emojis.widget();

        let debounced_search =
            make_debounce(160, move |arg: (ComponentSender<Self>, Self::Input)| {
                let (sender, msg) = arg;
                sender.input(msg)
            });

        let widgets = view_output!();

        let app = relm4::main_application();

        let action: RelmAction<FocusSearchAction> = RelmAction::new_stateless(glib::clone!(
            #[strong(rename_to = search_entry)]
            widgets.search_entry,
            move |_| {
                search_entry.grab_focus();
            }
        ));

        let mut group = RelmActionGroup::<WindowActionGroup>::new();
        group.add_action(action);
        group.register_for_widget(&widgets.main_window);
        app.set_accelerators_for_action::<FocusSearchAction>(&["<ctrl>f"]);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::PickEmoji(emoji) => {
                self.clipboard.set_text(emoji.as_str());
            }
            AppMsg::SearchEmojis(emoji_query) => {
                self.emojis
                    .sender()
                    .send(EmojiPickerInput::EmojiQuery(emoji_query))
                    .ok();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(());
}
