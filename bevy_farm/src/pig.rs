use bevy::prelude::*;

#[derive(Component)]
pub struct Pig {
    pub life_timer: Timer,
}

fn spawn_pig(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;

        info!("Spent $10 on a pig, remaining: {}", money.0);

        let pig_texture = asset_server.load("images/monster/nose_red.png");
        let mut pig_transform = *player_transform;
        pig_transform.translation.z = -1.0;
        command.spawn(
            (
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: SPRITE_SIZE, y: SPRITE_SIZE }),
                        ..default()
                    },
                    texture: pig_texture,
                    transform: pig_transform,
                    ..default()
                },
                Pig {
                    life_timer: Timer::from_seconds(2.0, TimerMode::Once)
                }
            )
        );
    }
}

fn pig_lifetime(
    mut command: Commands,
    timer: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.life_timer.tick(timer.delta());

        if pig.life_timer.finished() {
            money.0 += 15.0;
            command.entity(pig_entity).despawn();
            info!("Pig Sold, {} left", money.0);
        }
    }
}