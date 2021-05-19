mod components {
    pub mod alarm;
}

use iced::{
    Align,
    Length,
    Column,
    Row,
    Container,
    Text,
    Settings,
    Element,
    Button,
    Scrollable,
    Space,
    Sandbox,
    button,
    scrollable,
};

use components::alarm::{Alarm, Message as AlarmMessage};

/*

Alarm App                                  ( + )

+---------------------------------------------+
| 10:00am              S M T W T F S     [* ] |
| Alarm Name                                  |
+---------------------------------------------+

*/

pub fn main() -> iced::Result {
    Application::run(Settings::default())
}

#[derive(Debug, Clone)]
struct Application {
    alarm_create_button: button::State,
    body_scrollable: scrollable::State,
    alarms: Vec<Alarm>,
}

#[derive(Debug, Clone)]
enum Message {
    Create,
    Update(usize, AlarmMessage),
}

impl Sandbox for Application {

    type Message = Message;

    fn new() -> Application {
        Application {
            body_scrollable: scrollable::State::default(),
            alarm_create_button: button::State::default(),
            alarms: vec![],
        }
    }

    fn title(&self) -> String {
        String::from("Alarm App")
    }

    fn update(
        &mut self,
        message: Message,
    ) {
        match message {
            Message::Create => {
                self.alarms.push(Alarm::new(String::from("5:00pm")));
            },
            Message::Update(index, alarm_message) => {
                match alarm_message {
                    AlarmMessage::Delete => {
                        self.alarms.remove(index);
                    },
                    _ => {
                        if let Some(alarm) = self.alarms.get_mut(index) {
                            alarm.update(alarm_message);
                        }
                    },
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let alarms: Element<Message> = self.alarms
            .iter_mut()
            .enumerate()
            .fold(Column::new().spacing(20), |column, (i, alarm)| {
                column.push(
                    Row::new()
                        .push(
                            alarm.view().map(move |message| {
                                Message::Update(i, message)
                            })
                        )
                )
            })
            .into();

        let column: Column<Message> = Column::new()
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .spacing(20)
                    .max_width(640)
                    .push(
                        Row::new()
                            .align_items(Align::Start)
                            .width(Length::Fill)
                            .push(Text::new("Alarms"))
                            .push(Space::with_width(Length::Fill))
                            .push(
                                Button::new(&mut self.alarm_create_button, Text::new("+")).on_press(Message::Create)
                            )
                    )
                    .push(alarms);

        Scrollable::new(&mut self.body_scrollable)
            .padding(40)
            .push(
                Container::new(column).width(Length::Fill).center_x()
            )
            .into()
    }
}
