use iced::widget::{Column, button, column, text};
fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
