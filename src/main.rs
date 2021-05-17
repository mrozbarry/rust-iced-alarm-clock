use iced::{Column, Text, Settings, Align, Element, Button, Sandbox, button};

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
struct Alarm {
    time: String
}

#[derive(Debug, Clone)]
struct Application {
    alarm_create_button: button::State,
    alarms: Vec<Alarm>
}

#[derive(Debug, Clone)]
enum Message {
    AlarmCreate
}

impl Sandbox for Application {

    type Message = Message;

    fn new() -> Application {
        Application {
            alarms: vec![Alarm{time:"5:00pm".to_string()}],
            alarm_create_button: button::State::default(),
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
            Message::AlarmCreate => {
                self.alarms.push(Alarm{time:"5:00pm".to_string()});
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let alarms : Element<Message> = self.alarms
                        .iter_mut()
                        .enumerate()
                        .fold(Column::new().spacing(20), |column, (i, alarm)| {
                            column.push(
                                Column::new()
                                .push(Text::new(&alarm.time))
                                .push(Text::new(format!("Alarm {}", i)))
                            )
                        })
                        .into();

        Column::new()
            .align_items(Align::Center)
            .padding(20)
            .push(Button::new(&mut self.alarm_create_button, Text::new("Add Alarm")).on_press(Message::AlarmCreate))
            .push(alarms)
            .into()
    }
}
