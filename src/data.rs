use std::{default, ops::Deref, process::id, sync::{Arc, Mutex}};

use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use quad_gif::*;

#[derive(Clone, Default)] // Adicione Clone para permitir cópia
pub struct Alien {
    pub name: String,
    pub health: u32,
    pub defense: u32,
    pub damage: u32,
    pub speed: f32,

    pub idle_sprites: Vec<Arc<Mutex<GifAnimation>>>,
    pub run_sprites: Vec<Arc<Mutex<GifAnimation>>>,
    pub walk_sprites: Vec<Arc<Mutex<GifAnimation>>>,
    pub first_attack_sprites: Vec<Arc<Mutex<GifAnimation>>>,
}

impl Alien {

    pub async fn init_ghostfreak() -> Alien {
        let alien = Alien {
            name: "Ghostfreak".to_owned(),
            health: 90,
            defense: 35,
            speed: 5.0,
            damage: 45,
            idle_sprites: vec![
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/Idle_Right.gif".to_string()).await)),  
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/Idle_Left.gif".to_string()).await)),
            ],
            run_sprites: vec![
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/Run_Right.gif".to_string()).await)),
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/Run_Left.gif".to_string()).await)),
            ],
            walk_sprites: vec![
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/Walk_Right.gif".to_string()).await)),
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/Walk_Left.gif".to_string()).await)),
            ],
            first_attack_sprites: vec![
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/First_Attack_Right.gif".to_string()).await)),
                Arc::new(Mutex::new(GifAnimation::load("assets/aliens/ghostfreak/First_Attack_Left.gif".to_string()).await)),
            ],
        };


        alien

    }

}

#[derive(Default)]
pub struct Omnitrix {

    pub battery: u8,
    pub unlocked_aliens: Vec<Alien>,
    
    pub transformed_alien: Option<Alien>,

}

impl Omnitrix {

    pub async fn new() -> Omnitrix {
        Omnitrix {
            battery: 100,
            unlocked_aliens: vec![Alien::init_ghostfreak().await],
            transformed_alien: None,
        }
    }

    pub fn transform(&mut self, alien_name: &str) -> bool {
        if self.unlocked_aliens.is_empty() {
            return false;
        }

        if let Some(alien) = self.unlocked_aliens.iter().find(|a| a.name == alien_name) {
            self.transformed_alien = Some(alien.clone());
            true
        } else {
            false
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct Player {

    #[serde(skip)]
    pub omnitrix: Omnitrix,

    pub nickname: String,
    pub experience: u8,

    pub x: f32,
    pub y: f32,

    pub active: bool,
    pub transformed: bool,
    pub is_alive: bool,

}

impl Player {

    pub async fn new() -> Player {
        Player {
            omnitrix: Omnitrix::new().await,
            nickname: "Player638".to_owned(),
            experience: 100,
            x: 20.0,
            y: 130.0,
            active: true,
            transformed: false,
            is_alive: true,
        }
    }

    pub fn movement(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.x += 5.0;
        }
        if is_key_down(KeyCode::Left) {
            self.x -= 5.0;
        }
        if is_key_down(KeyCode::Down) {
            self.y += 5.0;
        }
        if is_key_down(KeyCode::Up) {
            self.y -= 5.0;
        }
    }

    pub fn render(&mut self) {

        if self.transformed == false {
            self.transformed = true;
            self.omnitrix.transform("Ghostfreak");
        }

        if let Some(alien) = &self.omnitrix.transformed_alien {
            let mut animation = if is_key_down(KeyCode::Left) {
                &alien.run_sprites[1]
            } else if is_key_down(KeyCode::Right) {
                &alien.run_sprites[0]
            } else if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Down) {
                &alien.walk_sprites[0]
            } else {
                &alien.idle_sprites[0]
            };

            if let Ok(mut anim) = animation.lock() {
                anim.draw_at(self.x, self.y);
                anim.tick();
            }

        } else {
            draw_circle(self.x, self.y, 20.0, GREEN);
        }

    }


}