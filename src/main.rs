mod utils;
mod scenes;
mod logic;

use std::collections::BTreeMap;
use crate::utils::mathemann::stretch_float_to;
use crate::utils::text::{draw_text_center, draw_text_centered};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};
use utils::enums::{SceneTextureKey, TextureKey};
use crate::scenes::level_selector::level_selector;
use crate::scenes::levels::levels;
use crate::scenes::levels::levels::{start_level, LevelSceneData};
use crate::scenes::main_menu::main_menu;

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

enum Scene {
    MainMenu,
    SettingsMenu,
    /// The i32 is the Page
    LevelSelector(i32),
    Level(levels::Level)
}

#[macroquad::main(window_conf)]
async fn main() {
    let skin = {
        let font = load_ttf_font("res/fonts/MinimalPixel v2.ttf").await.unwrap();

        let window_style = root_ui()
            .style_builder()
            .background(load_image("res/ui/window_background.png").await.expect("Error loading res/ui/window_background.png"))
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(load_image("res/ui/button/background.png").await.expect("Error loading res/ui/window_background.png"))
            .background_hovered(load_image("res/ui/button/background_hovered.png").await.expect("Error loading res/ui/button/background_hovered.png"))
            .background_clicked(load_image("res/ui/button/background_clicked.png").await.expect("Error loading res/ui/button/background_clicked.png"))
            .background_margin(RectOffset::new(70.0, 70.0, 70.0, 70.0))
            .margin(RectOffset::new(screen_width() / 100.0, screen_width() / 100.0, screen_height() / 100.0, screen_height() / 100.0))
            .with_font(&font)
            .unwrap()
            .text_color(WHITE)
            .text_color_hovered(LIGHTGRAY)
            .text_color_clicked(GRAY)
            .font_size(40)
            .build();

        Skin {
            window_style,
            button_style,
            ..root_ui().default_skin()
        }
    };

    let window_skin = skin.clone();

    root_ui().push_skin(&window_skin);

    // Runs to make sure the screen size is the right one
    for _ in 0..4 {
        println!("{}x{}", screen_width(), screen_height());
        next_frame().await
    }

    loading().await;

    // Holds the current scene
    let mut scene = Scene::MainMenu;
    // Holds all data of scenes (score, enemies ...)
    let mut level_scene_data = LevelSceneData {level: None, player: None, platforms: vec![], world: None, triggers: BTreeMap::new(), trigger_locks: BTreeMap::new() };
    // Holds all textures
    let mut textures = BTreeMap::<SceneTextureKey, BTreeMap<TextureKey, Vec<Texture2D>>>::new();
    loop {
        clear_background(BLACK);
        // Depending on the Scene does something else
        match scene {
            Scene::MainMenu => {
                main_menu(&mut scene).await;
            }
            Scene::SettingsMenu => {

            }
            Scene::LevelSelector(_) => {
                level_selector(&mut scene).await;
            }
            Scene::Level(_) => {
                start_level(&mut scene, &mut textures, &mut level_scene_data).await;
            }
        }

        // Gets ONLY called after the game loop is done to ensure everything is drawn the right way
        next_frame().await
    };
}

/// 100% True Loading Screen that is definitely not based on frames
async fn loading() {
    let mut last_update = 1.0;
    let speed = 0.0005;
    let mut last_opacity = 255;
    let mut opacity_up = false;
    let mut runtime = 0.0;

    loop {
        if get_time() - last_update > speed {
            last_update = get_time();
            if runtime > 100.0 {
                break
            } else {
                runtime += 0.5;
            }
            // Counts up if the alpha is larger than 0 and counts down
            let alpha = {
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
            draw_text_center("Welcome to MumboJumbo!", screen_height() / 8.0, Color::from_rgba(255, 255, 255, 255)).await;
            draw_text_centered("Loading...", screen_height() / 4.0, screen_height() / 16.0, Color::from_rgba(255, 255, 255, alpha)).await;

            { // Loading Bar
                let width = screen_width() / 4.0;
                let height = screen_height() / 1.5;


                // Stretches the runtime float (0.0 - 100.0) to 2/4 of the screen width and adds it to the start point of the loading bar
                let length = width + stretch_float_to(runtime, 100.0, width * 2.0).await;

                draw_line(width, height, length, height, screen_height() / 32.0, WHITE);
            }
        }
        next_frame().await
    }
}
