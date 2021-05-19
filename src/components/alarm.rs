use iced::{
    Align,
    Length,
    Column,
    Row,
    Text,
    Checkbox,
    Space,
    Element,
    Button,
    button,
};

/*
+---------------------------------------------+
| 10:00am              S M T W T F S     [* ] |
| Alarm Name                                  |
+---------------------------------------------+
*/

#[derive(Debug, Clone)]
pub struct Alarm {
    time: String,
    label: String,
    active: bool,

    delete_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(bool),
    Delete,
}

impl Alarm {
    pub fn new(t: String) -> Alarm {
        Alarm {
            time: t.to_string(),
            label: "Alarm Label Here...".to_string(),
            active: false,
            delete_button: button::State::default(),
        }
    }

    pub fn update(
        &mut self,
        message: Message,
    ) {
        match message {
            Message::Toggle(active) => {
                self.active = active
            },
            Message::Delete => {
                // Noop here
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .align_items(Align::Start)
            .width(Length::Fill)
            .push(
                Column::new()
                    .align_items(Align::Start)
                    .push(Text::new(&self.time))
                    .push(Text::new(&self.label))
            )
            .push(Space::with_width(Length::Fill))
            .push(
                Checkbox::new(self.active, String::from(""), Message::Toggle)
            )
            .push(
                Button::new(&mut self.delete_button, Text::new("-"))
                    .on_press(Message::Delete)

            )
            .into()
    }
}
