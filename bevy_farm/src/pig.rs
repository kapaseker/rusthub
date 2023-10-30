use bevy::prelude::*;
use crate::{Money, Player};
use crate::res::ints::PIG_LIFE_SECOND;
use crate::res::size::SPRITE;
use crate::res::sprite::MONSTER;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub life_timer: Timer,
}

#[derive(Component)]
pub struct PigParent;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pig_parent).add_systems(Update, (spawn_pig, pig_lifetime)).register_type::<Pig>();
    }
}

fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PigParent, Name::new("Pig Parent")));
}

fn spawn_pig(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;

        info!("Spent $10 on a pig, remaining: {}", money.0);

        let pig_texture = asset_server.load(MONSTER);
        let mut pig_transform = *player_transform;
        pig_transform.translation.z = -1.0;

        command.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2 { x: SPRITE, y: SPRITE }),
                        ..default()
                    },
                    texture: pig_texture,
                    transform: pig_transform,
                    ..default()
                },
                Pig {
                    life_timer: Timer::from_seconds(PIG_LIFE_SECOND, TimerMode::Once)
                },
                Name::new("Pig"),
            ));
        });
        //
        // command.spawn(
        //     (
        //
        //     )
        // );
    }
}

fn pig_lifetime(
    mut command: Commands,
    timer: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    parent: Query<Entity, With<PigParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();
    for (pig_entity, mut pig) in &mut pigs {
        pig.life_timer.tick(timer.delta());

        if pig.life_timer.finished() {
            money.0 += 15.0;
            command.entity(parent).remove_children(&[pig_entity]);
            command.entity(pig_entity).despawn();
            info!("Pig Sold, {} left", money.0);
        }
    }
}