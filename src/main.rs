mod utils;
mod scenes;
mod logic;
mod ui;

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use dirs::config_dir;
use crate::utils::mathemann::stretch_float_to;
use crate::utils::text::{draw_text_center, draw_text_centered};
use macroquad::prelude::*;
use utils::enums::{Scene, SceneTextureKey, TextureKey};
use crate::scenes::level_selector::level_selector;
use crate::scenes::levels::levels::start_level;
use crate::scenes::main_menu::main_menu;
use logic::level::LevelSceneData;
use utils::structs::{Settings, TempSettings};
use crate::scenes::settings_menu::settings_menu;
use logic::level::PersistentLevelData;
use crate::scenes::credits::credits;

fn window_conf() -> Conf {
    Conf {
        window_title: "JumboMumbo".to_owned(),
        fullscreen: true,
        platform: miniquad::conf::Platform {
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let loading_handler = loading();

    let mut settings = {
        let config_path = format!("{}/JumboMumbo", config_dir().expect("Couldn't get config path").to_str().unwrap().to_owned());

        if !fs::exists(&config_path).unwrap() {
            fs::create_dir(&config_path).expect("Couldn't create settings directory");
            println!("Creating settings directory at: {}", config_path);
        }
        println!("Config path: {}", config_path);

        let settings = {
            let file_path = format!("{}/settings.json", config_path);
            println!("Settings file path: {}", file_path);
            match fs::exists(&file_path).unwrap() {
                true => {
                    let new_settings = {
                        let level_data_path = format!("{}/level_data.json", config_path);
                        Settings::new(file_path.to_owned(), level_data_path).await
                    };

                    let file = fs::File::open(&file_path).expect("Couldn't open settings file");

                    let settings: Settings = serde_json::from_reader(file).unwrap_or_else(|e| {
                        println!("Couldn't deserialize persistent level data with error \"{}\"", e);
                        for i in 0.. {
                            let path = file_path.replace(".json", format!(".{i}.json").as_str());
                            if !fs::exists(&path).unwrap() {
                                match fs::rename(&file_path, &path) {
                                    Ok(_) => {
                                        println!("Moved old settings file to {path}");
                                        break;
                                    },
                                    Err(_) => {
                                        println!("Couldn't move old level data file to \"{}\"! Ignoring...", path);
                                        break;
                                    }
                                }
                            }
                        }
                        new_settings
                    });

                    settings
                },
                false => {
                    let level_data_path = format!("{}/level_data.json", config_path);
                    let settings = Settings::new(file_path.to_owned(), level_data_path).await;
                    let mut file = fs::File::create(&file_path).unwrap();

                    let s_settings = serde_json::to_string_pretty(&settings).expect("Couldn't serialize settings");
                    file.write_all(s_settings.as_bytes()).expect("Couldn't write settings file");

                    settings
                }
            }
        };

        settings
    };
    let mut temp_settings = TempSettings { settings: settings.clone() };
    println!("{:?}", settings);

    let mut persistent_level_data = {
        match fs::exists(&settings.level_data_path).unwrap() {
            true => {
                let file = fs::File::open(&settings.level_data_path).expect("Couldn't open level data file");
                let persistent_level_data: PersistentLevelData = serde_json::from_reader(file).unwrap_or_else(|e| {
                    println!("Couldn't deserialize persistent level data with error \"{}\"", e);
                    for i in 0.. {
                        let path = settings.level_data_path.replace(".json", format!(".{i}.json").as_str());
                        if !fs::exists(&path).unwrap() {
                            match fs::rename(&settings.level_data_path, &path) {
                                Ok(_) => {
                                    println!("Moved old level data file to {path}");
                                    break;
                                },
                                Err(_) => {
                                    println!("Couldn't move old level data file to \"{}\"! Ignoring...", path);
                                    break;
                                }
                            }
                        }
                    }
                    PersistentLevelData::new()
                });

                persistent_level_data
            }
            false => {
                let persistent_level_data = PersistentLevelData::new();

                persistent_level_data.save(&settings).await;

                persistent_level_data
            }
        }
    };

    // Runs to make sure the screen size is the right one
    for _ in 0..4 {
        next_frame().await
    }
    println!("Resolution: {}x{}", screen_width(), screen_height());

    // Holds the current scene
    let mut scene = Scene::MainMenu;
    // Holds all data of scenes (score, enemies ...)
    let mut level_scene_data = LevelSceneData::empty().await;
    // Holds all textures
    let mut textures = BTreeMap::<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>::new();

    loading_handler.await;

    loop {
        clear_background(BLACK);
        // Depending on the Scene does something else
        match scene {
            Scene::MainMenu => {
                main_menu(&mut scene, &mut textures, &settings).await;
            }
            Scene::SettingsMenu => {
                settings_menu(&mut scene, &mut textures, &mut settings, &mut temp_settings).await;
            }
            Scene::LevelSelector(_) => {
                level_selector(&mut scene, &mut textures, &settings, &persistent_level_data).await;
            }
            Scene::Level(_) => {
                start_level(&mut scene, &mut textures, &mut level_scene_data, &mut persistent_level_data, &settings).await;
            }
            Scene::Credits(_) => {
                credits(&mut scene, &settings).await;
            }
        }

        next_frame().await
    };
}

/// 100% True Loading Screen
async fn loading() {
    let mut last_update = 1.0;
    let speed = 0.000005;
    let mut last_opacity = 255;
    let mut opacity_up = false;
    let mut runtime = 0.0;

    let mut alpha = 0;

    loop {
        if get_time() - last_update > speed {
            last_update = get_time();
            if runtime > 100.0 {
                break
            } else {
                runtime += 80.0 * get_frame_time();
            }
            // Counts up if the alpha is larger than 0 and counts down
            alpha = {
                if opacity_up {
                    if last_opacity < 255 {
                        last_opacity + 4
                    } else {
                        opacity_up = false;
                        last_opacity
                    }
                } else {
                    if last_opacity > 4 {
                        last_opacity - 4
                    } else {
                        opacity_up = true;
                        last_opacity
                    }
                }
            };
            last_opacity = alpha;
        }

        draw_text_center("Welcome to JumboMumbo!", screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;
        draw_text_centered("Loading...", screen_height() / 4.0, screen_height() / 16.0, Color::from_rgba(255, 255, 255, alpha)).await;

        { // Loading Bar
            let width = screen_width() / 4.0;
            let height = screen_height() / 1.5;


            // Stretches the runtime float (0.0 - 100.0) to 2/4 of the screen width and adds it to the start point of the loading bar
            let length = width + stretch_float_to(runtime, 100.0, width * 2.0).await;

            draw_line(width, height, length, height, screen_height() / 32.0, WHITE);
        }
        next_frame().await
    }
}
