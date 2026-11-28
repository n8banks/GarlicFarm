use godot::classes::{CharacterBody2D, ICharacterBody2D, ISprite2D, Input, Sprite2D};
use::godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[export]
    speed: f32,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Player created!");
        Self { speed: 300.0, base}
    }

    fn physics_process(&mut self, delta: f64)
    {
        let input = Input::singleton();
        let direction = input.get_vector("ui_left", "ui_right", 
                                                  "ui_up", "ui_down");

        let velocity = direction * self.speed;
        let position = self.base().get_position();
    
        self.base_mut().set_position(position + velocity * delta as f32);
    }
}