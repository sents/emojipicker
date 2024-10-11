use emojis::Emoji;
use gtk::prelude::{ButtonExt, WidgetExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender};
use relm4::{gtk, RelmWidgetExt};
use std::sync::LazyLock;

static ROCKET: LazyLock<&'static Emoji> = LazyLock::new(|| {
    emojis::get("🚀").unwrap()
});

#[derive(Debug, PartialEq)]
pub struct EmojiGridElement {
    emoji: &'static Emoji,
}

#[relm4::factory(pub)]
impl FactoryComponent for EmojiGridElement {
    type Init = &'static Emoji;
    type Input = ();
    type Output = String;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;
    view! {
        #[root]
        gtk::Button {
            set_label: self.emoji.as_str(),
            set_align: gtk::Align::Center,
            set_hexpand: false,
            set_vexpand: false,
            set_can_focus: false,
            connect_clicked[sender,
                                     emoji_str = self.emoji.as_str()] => move |_| {
                let _ = sender.output(emoji_str.to_string());
            },
        },
        #[local_ref]
        returned_widget -> gtk::FlowBoxChild {
            set_vexpand: false,
            set_align: gtk::Align::Start,
            set_can_focus: true,
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { emoji: value }
    }

    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}
