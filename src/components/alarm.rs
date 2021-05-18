use iced::{
    Align,
    Length,
    Column,
    Row,
    Text,
    Checkbox,
    Space,
    Element,
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
}

#[derive(Debug, Clone)]
pub enum Message {
    Toggle(bool),
}

impl Alarm {
    pub fn new(t: String) -> Alarm {
        Alarm {
            time: t.to_string(),
            label: "Alarm Label Here...".to_string(),
            active: false,
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
                Checkbox::new(self.active, String::from("Active"), Message::Toggle)
            )
            .into()
    }
}
