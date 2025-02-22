use iced::{
    widget::{button, container, row, text, Column},
    Application, Command, Element, Settings, Theme, Length,
    time::every,
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
    command_tx: mpsc::Sender<()>,
    roi_controller: Arc<Mutex<ROIController>>,
    status_text: String,
    power_state: bool,
    start_state: bool,
    stop_state: bool,
}

impl Application for TestInterface {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = (mpsc::Receiver<AppCommand>, mpsc::Sender<()>, Arc<Mutex<ROIController>>);

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            TestInterface {
                command_rx: flags.0,
                command_tx: flags.1,
                roi_controller: flags.2,
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
                let _ = self.command_tx.send(());
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

        let power_button = button(power_text)
            .width(Length::Fixed(150.0))
            .style(if self.power_state {
                iced::theme::Button::Positive
            } else {
                iced::theme::Button::Secondary
            })
            .on_press(Message::ButtonPressed("power"));

        let start_button = button(start_text)
            .width(Length::Fixed(150.0))
            .style(if self.start_state {
                iced::theme::Button::Positive
            } else {
                iced::theme::Button::Secondary
            })
            .on_press(Message::ButtonPressed("start"));

        let stop_button = button(stop_text)
            .width(Length::Fixed(150.0))
            .style(if self.stop_state {
                iced::theme::Button::Positive
            } else {
                iced::theme::Button::Secondary
            })
            .on_press(Message::ButtonPressed("stop"));

        let exit_button = button("Exit")
            .width(Length::Fixed(150.0))
            .style(iced::theme::Button::Destructive)
            .on_press(Message::Exit);

        let buttons = row![power_button, start_button, stop_button]
            .spacing(20)
            .padding(20);

        let content = Column::new()
            .push(status)
            .push(buttons)
            .push(exit_button)
            .spacing(20)
            .padding(20)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        every(std::time::Duration::from_millis(100))
            .map(|_| Message::Tick)
    }
}

pub fn run_gui(
    command_rx: mpsc::Receiver<AppCommand>,
    command_tx: mpsc::Sender<()>,
    roi_controller: Arc<Mutex<ROIController>>,
) -> anyhow::Result<()> {
    let mut settings = Settings::with_flags((command_rx, command_tx, roi_controller));
    settings.window.size = (600, 300);
    settings.window.resizable = false;
    settings.window.decorations = true;
    settings.window.transparent = false;
    settings.window.level = iced::window::Level::AlwaysOnTop;  // Keep window on top
    settings.window.position = iced::window::Position::Specific(20, 20);
    settings.antialiasing = true;
    settings.exit_on_close_request = false;  // Prevent window from closing when clicking other windows

    TestInterface::run(settings)?;
    Ok(())
}
