use gtk::gdk::Clipboard;
use gtk::prelude::{
    BoxExt, DisplayExt, EditableExt, GtkWindowExt, OrientableExt, WidgetExt
};
use relm4::factory::FactoryVecDeque;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

use emojis::Emoji;
mod components;
use components::emojigrid::EmojiGridElement;

mod debounce;
use debounce::make_debounce;


// static debouncer = make_debounce(100, |msg: &str| {println!("{}", msg)});


fn search_emojis(query: &str) -> Vec<&'static Emoji> {
    if query.is_empty() {
        emojis::iter().collect()
    } else {
        emojis::iter()
            .filter(|emoji| {
                emoji.name().to_lowercase().contains(&query.to_lowercase())
                    || emoji
                    .shortcodes()
                    .any(|shortcode|
                         shortcode.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect()
    }
}

struct AppModel {
    emojis: FactoryVecDeque<EmojiGridElement>,
    clipboard: Clipboard,
}

#[derive(Debug)]
enum AppMsg {
    SearchEmojis(String),
    PickEmoji(String),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            // set_class_active: ("identical", model.identical),
            set_title: Some("Simple app"),
            set_default_width: 300,
            set_default_height: 80,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                gtk::Entry {
                    connect_changed[sender, debouncer] => move |entry| {
                        debouncer((
                            AppMsg::SearchEmojis(entry.text().to_string()),
                            sender.clone()
                        ))
                    }
                },
                gtk::ScrolledWindow {
                    set_vexpand: true,
                    set_propagate_natural_height: true,
                    set_max_content_height: 20,
                    #[local_ref]
                    emoji_box -> gtk::FlowBox {
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
        let emojis = FactoryVecDeque::builder()
            .launch(
                gtk::FlowBox::builder().vexpand(false)
                    .orientation(gtk::Orientation::Horizontal)
                    .homogeneous(true)
                    .max_children_per_line(99)
                    .row_spacing(0)
                    .column_spacing(0)
                    .valign(gtk::Align::Start)
                    .build()
            )
            .forward(sender.input_sender(), |output: String| {
                AppMsg::PickEmoji(output)
            });

        let display = root.display();
        let clipboard = display.clipboard();

        let mut model = AppModel { clipboard, emojis };

        for emoji in search_emojis("").iter() {
            model.emojis.guard().push_back(emoji);
        }

        let emoji_box = model.emojis.widget();

        let debouncer = make_debounce(200,
                                      move |arg: (Self::Input, ComponentSender<Self>)| {
                                          let (msg, sender) = arg;
                                          sender.input(msg)
                                      });

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::PickEmoji(emoji) => {
                self.clipboard.set_text(&emoji);
            }
            AppMsg::SearchEmojis(emoji_query) => {
                let mut emojis_guard = self.emojis.guard();
                let search = search_emojis(&emoji_query);
                emojis_guard.clear();
                for emoji in search {
                    emojis_guard.push_back(emoji);
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(());
}
