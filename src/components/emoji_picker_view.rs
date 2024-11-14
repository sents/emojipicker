use crate::components::skin_tone::*;
use std::borrow::Borrow;
use emojis::Emoji;
use gtk::prelude::{ButtonExt, FlowBoxChildExt, PopoverExt, WidgetExt, GridExt, ListItemExt};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    RelmWidgetExt, SimpleComponent,
};
use gtk::prelude::*;
use relm4::{
    binding::{Binding, U8Binding},
    prelude::*,
    typed_view::grid::{RelmGridItem, TypedGridView},
    RelmObjectExt,
};

fn make_tone_picker(value: &'static Emoji,
                    sender: ComponentSender<EmojiPicker>)
                    -> Option<Controller<SkinTonePicker>> {
    value.skin_tones().map(|_| {
        SkinTonePicker::builder()
            .launch(value)
            .forward(sender.input_sender(),
                     |output| EmojiPickerInput::PickEmoji(output))
    })
}

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
struct EmojiPickerElement<C: Component> {
    emoji: &'static Emoji,
    tone_picker: Option<Controller<SkinTonePicker>>,
    sender: ComponentSender<C>
}

struct EmojiPickerElementWidgets {
    button: gtk::Button,
    popover: gtk::Popover
}

impl RelmGridItem for EmojiPickerElement<EmojiPicker> {
    type Root = gtk::Box;
    type Widgets =  EmojiPickerElementWidgets;

    fn bind(&mut self, widgets: &mut Self::Widgets, root: &mut Self::Root) {
        widgets.button.set_label(self.emoji.as_str());
        widgets.button.set_tooltip_text(Some(self.emoji.name()));
        widgets.popover.set_child(self.tone_picker
                                  .as_ref().map(|picker| picker.widget()));

    }

    fn setup(grid_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        relm4::view! {
            #[root]
            root_box = gtk::Box {
                set_align: gtk::Align::Start,
                set_can_focus: true,
                set_focus_on_click: true,
                set_vexpand: false,
                #[name="button"]
                gtk::Button {
                    set_align: gtk::Align::Center,
                    set_hexpand: false,
                    set_vexpand: false,
                    set_can_focus: false,
                    // connect_clicked[sender,
                    //                 emoji = self.emoji] => move |_| {
                    //     let _ = sender.output(emoji);
                    // },
                },
                #[name="popover"]
                gtk::Popover {
                }
            },
        }

        let widgets = Self::Widgets {
            button,
            popover
        };

        (root_box, widgets)
    }

}


#[derive(Debug)]
pub struct EmojiPicker {
    grid: TypedGridView<EmojiPickerElement<Self>, gtk::SingleSelection>,
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
        gtk::ScrolledWindow {

            set_vexpand: true,
            set_hexpand: true,
            set_propagate_natural_height: true,
            set_max_content_height: 20,
            model.grid.view.borrow() -> &gtk::GridView {
                set_halign: gtk::Align::Start,
                set_valign: gtk::Align::Start,
                set_max_columns: 99,
                set_orientation: gtk::Orientation::Vertical,
                set_vexpand_set: false,
                set_vexpand: false,
                set_hexpand: false,
                set_hexpand_set: false,
            }
        }
    }

    fn init(
        _settings: (),
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut view: TypedGridView<EmojiPickerElement<EmojiPicker>, gtk::SingleSelection>
            = TypedGridView::default();

        view.extend_from_iter(search_emojis("").into_iter().map(
            |emoji|
            {
                EmojiPickerElement {
                    emoji,
                    tone_picker: make_tone_picker(emoji, sender.clone()),
                    sender: sender.clone()
                }
            }
        ));
        // for emoji in search_emojis("").into_iter() {
        //     emojis.guard().push_back(emoji);
        // }


        let model = EmojiPicker { grid: view };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: EmojiPickerInput, sender: ComponentSender<Self>) {
        match input {
            EmojiPickerInput::PickEmoji(pick) => {
                sender.output(pick).ok();
            }
            EmojiPickerInput::EmojiQuery(emoji_query) => {
                let search = search_emojis(&emoji_query);
                self.grid.clear_filters();
                self.grid.add_filter(move |item| search.contains(&item.emoji))
            }
        }
    }
}
