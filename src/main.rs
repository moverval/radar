use std::{rc::Rc, sync::{mpsc::{Sender, Receiver}}, thread};

use action::RadioAction;
use slint::{VecModel, ModelRc, Model};

slint::include_modules!();

mod config;
mod action;
mod mp3;

fn apply_search_filter(radios: Vec<config::IndexedRadio>, search: &str) -> Vec<config::IndexedRadio> {
    let search = config::IndexedRadio::retain(search.to_string());
    radios.into_iter().filter(|radio| radio.is_part(&search)).collect()
}

fn create_radio_display(radios: Vec<config::Radio>) -> ModelRc<ModelRc<Radio>> {
    let radio_display: Vec<Radio> = radios.into_iter().map(|radio| Radio {
        name: radio.name.into(),
        description: radio.description.into(),
        url: radio.url.into()
    }).collect();

    let radio_display: Vec<ModelRc<Radio>> = radio_display
        .chunks(4)
        .map(|chunk| Rc::new(
            slint::VecModel::from(chunk.to_vec())
        ).into())
        .collect();

    let radio_model: Rc<VecModel<ModelRc<Radio>>> = Rc::new(VecModel::from(radio_display));

    radio_model.into()
}

fn main() {
    let yaml = match std::fs::read_to_string("./radios.yaml") {
        Ok(content) => content,
        Err(_) => {
            println!("Could not read yaml. File does not exist");
            return;
        }
    };

    let radios = match config::Radio::parse(&yaml) {
        Ok(radios) => radios,
        Err(err) => {
            println!("Yaml file incorrectly formatted: {}", err.to_string());
            return;
        }
    };

    let (tx, rx): (Sender<action::RadioAction>, Receiver<RadioAction>) = std::sync::mpsc::channel();

    thread::spawn(move || {
        let (_stream, stream_handler) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handler).unwrap();
        sink.set_volume(0.5);
        let client = reqwest::blocking::Client::new();

        loop {
            let action = match rx.recv() {
                Ok(action) => action,
                Err(_) => return,
            };

            match action {
                action::RadioAction::Play(url) => {
                    let response = client.get(url).send().unwrap();
                    let source = mp3::Mp3StreamDecoder::new(response).unwrap();
                    sink.append(source);
                    sink.play();
                },
                action::RadioAction::Stop => {
                    sink.clear();
                    sink.pause();
                }
            }
        }
    });

    let radio_window = RadioWindow::new().unwrap();
    let rw_weak = radio_window.as_weak();
    let rw_weak2 = radio_window.as_weak();
    let rw_weak3 = radio_window.as_weak();
    let tx2 = tx.clone();

    radio_window.set_radios(create_radio_display(radios.clone()));
    let indexed_radios: Vec<config::IndexedRadio> = radios.into_iter().map(|radio| config::IndexedRadio::from(radio)).collect();

    radio_window.on_searchbar_used(move || {
        let radio_window = rw_weak3.unwrap();

        let radios = apply_search_filter(indexed_radios.clone(), radio_window.get_search().as_str());

        radio_window.set_radios(create_radio_display(radios.into_iter().map(|radio| config::Radio::from(radio)).collect()));
    });

    radio_window.on_selected_changed(move || {
        let radio_window = rw_weak2.unwrap();
        if radio_window.get_playing() {
            radio_window.set_playing(true);
            tx2.send(RadioAction::Stop).unwrap();
            tx2.send(RadioAction::Play(radio_window.get_selected().url.to_string())).unwrap();
        }
    });

    radio_window.on_pause_continue(move || {
        let radio_window = rw_weak.unwrap();
        if !radio_window.get_playing() {
            tx.send(RadioAction::Stop).unwrap();
        } else {
            tx.send(RadioAction::Stop).unwrap();
            tx.send(RadioAction::Play(radio_window.get_selected().url.to_string())).unwrap();
        }
    });

    radio_window.run().unwrap();
}
