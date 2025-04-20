use crate::prelude::*;

use crate::player::player::Player;



pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    for (player, mut transform) in &mut query {
        let mut direction = 0.0;
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }
        
        transform.translation.x += direction * player.speed * time.delta_secs();
    }
}

pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut player, mut transform) in &mut query {
        if keyboard_input.just_pressed(KeyCode::Space){
            player.is_grounded = true;
            transform.translation.y += player.jump_force * time.delta_secs();
        } else if player.is_grounded {
            player.is_grounded = false
        }
    }
}