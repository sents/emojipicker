use emojis::Emoji;
use gtk::prelude::{ButtonExt, FlowBoxChildExt, WidgetExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::{gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
struct TonePickerElement {
    emoji: &'static Emoji,
}

#[relm4::factory(pub)]
impl FactoryComponent for TonePickerElement {
    type Init = &'static Emoji;
    type Input = ();
    type Output = &'static Emoji;
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;
    view! {
        #[root]
        gtk::Box {
            #[name="button"]
            gtk::Button {
                set_label: self.emoji.as_str(),
                set_align: gtk::Align::Center,
                set_hexpand: false,
                set_vexpand: false,
                set_can_focus: false,
                set_tooltip_text: Some(self.emoji.name()),
                connect_clicked[sender,
                                emoji = self.emoji] => move |_| {
                    let _ = sender.output(emoji);
                },
            },
        },
        #[local_ref]
        returned_widget -> gtk::FlowBoxChild {
            set_vexpand: false,
            set_align: gtk::Align::Start,
            set_can_focus: true,
            set_focus_on_click: true,
            connect_activate[button] => move |_| {
                button.activate();
            },
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { emoji: value }
    }

    fn update(&mut self, _msg: Self::Input, _sender: FactorySender<Self>) {}
}

#[derive(Debug)]
pub struct SkinTonePicker {
    grid: FactoryVecDeque<TonePickerElement>,
}

#[relm4::component(pub)]
impl SimpleComponent for SkinTonePicker {
    type Widgets = SkinToneWidgets;
    type Init = &'static Emoji;
    type Input = &'static Emoji;
    type Output = &'static Emoji;

    view! {
        #[root]
        gtk::Box {
            #[local_ref]
            emoji_box -> gtk::FlowBox {}
        }
    }

    fn init(
        init: &'static Emoji,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut emojis = FactoryVecDeque::builder()
            .launch(
                gtk::FlowBox::builder()
                    .vexpand(false)
                    .orientation(gtk::Orientation::Horizontal)
                    .homogeneous(true)
                    .max_children_per_line(5)
                    .row_spacing(0)
                    .column_spacing(0)
                    .valign(gtk::Align::Start)
                    .build(),
            )
            .forward(sender.input_sender(), |output: &'static Emoji| output);

        for emoji in init.skin_tones().expect("Should have Skintones") {
            emojis.guard().push_back(emoji);
        }

        let model = SkinTonePicker { grid: emojis };

        let emoji_box = model.grid.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: &'static Emoji, sender: ComponentSender<Self>) {
        sender.output(input).ok();
    }
}
