use iced::{
    Application, Element, Command, Settings, Container, Row, Column, Text, Button, Length,
    button, text, time,
};
use std::sync::{mpsc, Arc, Mutex};
use crate::roi_controller::ROIController;

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(&'static str),
    Exit,
    Tick,
}

#[derive(Debug, Clone)]
pub enum AppCommand {
    Click(&'static str),
}

pub struct TestInterface {
    command_rx: mpsc::Receiver<AppCommand>,
    roi_controller: Arc<Mutex<ROIController>>,
    status_text: String,
    power_state: bool,
    start_state: bool,
    stop_state: bool,
}

impl Application for TestInterface {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = (mpsc::Receiver<AppCommand>, Arc<Mutex<ROIController>>);

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            TestInterface {
                command_rx: flags.0,
                roi_controller: flags.1,
                status_text: String::from("Waiting for voice commands..."),
                power_state: false,
                start_state: false,
                stop_state: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Voice Control Test")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPressed(cmd) => {
                self.status_text = format!("Last command: {}", cmd);
                match cmd {
                    "power" => self.power_state = !self.power_state,
                    "start" => self.start_state = !self.start_state,
                    "stop" => self.stop_state = !self.stop_state,
                    _ => {}
                }
                if let Ok(mut controller) = self.roi_controller.lock() {
                    controller.click_region(cmd);
                }
                Command::none()
            }
            Message::Exit => {
                Command::none()
            }
            Message::Tick => {
                match self.command_rx.try_recv() {
                    Ok(AppCommand::Click(cmd)) => {
                        self.status_text = format!("Last command: {}", cmd);
                        match cmd {
                            "power" => self.power_state = !self.power_state,
                            "start" => self.start_state = !self.start_state,
                            "stop" => self.stop_state = !self.stop_state,
                            _ => {}
                        }
                        if let Ok(mut controller) = self.roi_controller.lock() {
                            controller.click_region(cmd);
                        }
                    }
                    Err(_) => {}
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let status = text(&self.status_text);
        
        let power_text = if self.power_state { "Power ON" } else { "Power OFF" };
        let start_text = if self.start_state { "Go ON" } else { "Go OFF" };
        let stop_text = if self.stop_state { "Stop ON" } else { "Stop OFF" };

        let power_button = Button::new(text(power_text))
            .on_press(Message::ButtonPressed("power"))
            .padding(10);

        let start_button = Button::new(text(start_text))
            .on_press(Message::ButtonPressed("start"))
            .padding(10);

        let stop_button = Button::new(text(stop_text))
            .on_press(Message::ButtonPressed("stop"))
            .padding(10);

        let exit_button = Button::new(text("Exit"))
            .on_press(Message::Exit)
            .padding(10);

        let button_row = Row::new()
            .spacing(20)
            .push(power_button)
            .push(start_button)
            .push(stop_button)
            .push(exit_button);

        Column::new()
            .spacing(20)
            .padding(20)
            .push(status)
            .push(button_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        time::every(std::time::Duration::from_millis(100))
            .map(|_| Message::Tick)
    }
}

pub fn run(
    command_rx: mpsc::Receiver<AppCommand>,
    roi_controller: Arc<Mutex<ROIController>>,
) -> anyhow::Result<()> {
    let mut settings = Settings::default();
    settings.window.size = (400, 200);
    TestInterface::run(settings, (command_rx, roi_controller))?;
    Ok(())
}
