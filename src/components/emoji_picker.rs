use crate::components::skin_tone::*;
use emojis::Emoji;
use gtk::prelude::{ButtonExt, FlowBoxChildExt, PopoverExt, WidgetExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    RelmWidgetExt, SimpleComponent,
};

pub fn emoji_matches(emoji: &'static Emoji, query: &str) -> bool {
    emoji.name().to_lowercase().contains(&query.to_lowercase())
        || emoji
        .shortcodes()
        .any(|shortcode| shortcode.to_lowercase().contains(&query.to_lowercase()))
}

fn search_emojis(query: &str) -> Vec<&'static Emoji> {
    if query.is_empty() {
        emojis::iter().collect()
    } else {
        emojis::iter()
            .filter(|emoji| emoji_matches(emoji, query))
            .collect()
    }
}

#[derive(Debug)]
struct EmojiPickerElement {
    emoji: &'static Emoji,
    tone_picker: Option<Controller<SkinTonePicker>>,
}

#[relm4::factory(pub)]
impl FactoryComponent for EmojiPickerElement {
    type Init = &'static Emoji;
    type Input = &'static Emoji;
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
            #[name="popover"]
            gtk::Popover {
                #[local_ref]
                tone_picker_widget -> gtk::Box {}
            }
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

    fn init_model(value: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let tone_picker = value.skin_tones().map(|_| {
            SkinTonePicker::builder()
                .launch(value)
                .forward(sender.input_sender(), |output| output)
        });
        let model = Self {
            emoji: value,
            tone_picker,
        };
        model
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        returned_widget: &gtk::FlowBoxChild,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let tone_picker_widget = match &self.tone_picker {
            None => gtk::Box::builder().build(),
            Some(picker) => picker.widget().clone(),
        };
        let widgets = view_output!();

        // Popup on right click
        let controller = gtk::GestureClick::builder().button(3).build();
        let has_picker = !self.tone_picker.is_none();
        let popover = widgets.popover.clone();
        controller.connect_pressed(move |_, _key, _, _| {
            if has_picker {
                popover.popup()
            };
        });
        widgets.button.add_controller(controller);

        widgets
    }

    fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
        sender.output(msg).ok();
    }
}

#[derive(Debug)]
pub struct EmojiPicker {
    grid: FactoryVecDeque<EmojiPickerElement>,
}

#[derive(Debug)]
pub enum EmojiPickerInput {
    EmojiQuery(String),
    PickEmoji(&'static Emoji),
}

#[relm4::component(pub)]
impl SimpleComponent for EmojiPicker {
    type Widgets = EmojiPickerWidgets;
    type Init = ();
    type Input = EmojiPickerInput;
    type Output = &'static Emoji;

    view! {
        #[root]
        gtk::Box {
            #[local_ref]
            emoji_box -> gtk::FlowBox {}
        }
    }

    fn init(
        _settings: (),
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut emojis = FactoryVecDeque::builder()
            .launch(
                gtk::FlowBox::builder()
                    .vexpand(false)
                    .orientation(gtk::Orientation::Horizontal)
                    .homogeneous(true)
                    .max_children_per_line(99)
                    .row_spacing(0)
                    .column_spacing(0)
                    .valign(gtk::Align::Start)
                    .build(),
            )
            .forward(sender.input_sender(), |output: &'static Emoji| {
                EmojiPickerInput::PickEmoji(output)
            });

        for emoji in search_emojis("").into_iter() {
            emojis.guard().push_back(emoji);
        }

        let model = EmojiPicker { grid: emojis };

        let emoji_box = model.grid.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: EmojiPickerInput, sender: ComponentSender<Self>) {
        match input {
            EmojiPickerInput::PickEmoji(pick) => {
                sender.output(pick).ok();
            }
            EmojiPickerInput::EmojiQuery(emoji_query) => {
                let mut guard = self.grid.guard();
                let search = search_emojis(&emoji_query);
                guard.clear();
                for emoji in search {
                    guard.push_back(emoji);
                }
            }
        }
    }
}
