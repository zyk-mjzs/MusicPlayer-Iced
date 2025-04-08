use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, alignment};
use rfd::FileDialog;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Default)]
pub struct MusicPlayer {
    current_file: Option<PathBuf>,
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
    is_playing: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Pause,
    Stop,
    SelectFile,
}

impl MusicPlayer {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Play => {
                if let (Some(sink), Some(path)) = (&self.sink, &self.current_file) {
                    if !self.is_playing {
                        let file = BufReader::new(File::open(path).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                        sink.play();
                        self.is_playing = true;
                    }
                }
            }
            Message::Pause => {
                if let Some(sink) = &self.sink {
                    if self.is_playing {
                        sink.pause();
                        self.is_playing = false;
                    }
                }
            }
            Message::Stop => {
                if let Some(sink) = &self.sink {
                    sink.stop();
                    self.is_playing = false;
                }
            }
            Message::SelectFile => {
                let file = FileDialog::new()
                    .add_filter("mp3", &["mp3"])
                    .add_filter("wav", &["wav"])
                    .pick_file();

                if let Some(path_buf) = file {
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&stream_handle).unwrap();

                    self.current_file = Some(path_buf);
                    self.sink = Some(sink);
                    self._stream = Some(_stream);
                    self.is_playing = false;
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let status = if self.is_playing {
            "Playing"
        } else {
            "Stopped"
        };

        let title = text("Music Player")
            .size(28)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center);

        let status_text = text(format!("Status: {}", status))
            .size(20)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center);

        let file_text = text(format!(
            "Current File: {}",
            self
                .current_file
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("None")
        ))
        .size(16)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);

        let controls = row![
            button("Play").on_press(Message::Play),
            button("Pause").on_press(Message::Pause),
            button("Stop").on_press(Message::Stop),
            button("Select File").on_press(Message::SelectFile)
        ]
        .spacing(10)
        .width(Length::Fill)
        .align_y(alignment::Alignment::Center);

        container(
            column![title, status_text, file_text, controls]
                .padding(20)
                .spacing(20)
                .width(Length::Fill)
                .align_x(alignment::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        // .center_x()
        // .center_y()
        .into()
    }
}
