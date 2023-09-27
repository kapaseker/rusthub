use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component, Copy)]
struct Xp(u32);

impl Clone for Xp {
    fn clone(&self) -> Self {
        return Xp(self.0);
    }
}

#[derive(Component, Copy)]
struct Damage(u32);

impl Clone for Damage {
    fn clone(&self) -> Self {
        return Damage(self.0);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Event)]
struct PlayerAttack(Damage);

#[derive(Event)]
struct EnemyKill(Damage);

/// use query and event simply
fn main() {
    let mut xp = Xp(32);
    xp.0 = 12;

    App::new().add_plugins(DefaultPlugins)
        .add_systems(Update, (bevy::window::close_on_esc, print_player_info, print_enemy_info, keyboard_input_system, check_win))
        .add_systems(Startup, setup)
        .add_event::<PlayerAttack>()
        .add_event::<EnemyKill>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Player, Xp(100), Damage(10)));
    commands.spawn((Enemy, Xp(120), Damage(2)));
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>,
                         player_query: Query<(&Xp, &Damage), With<Player>>,
                         enemy_query: Query<(&Xp, &Damage), With<Enemy>>,
                         mut attack_writer: EventWriter<PlayerAttack>,
                         mut kill_writer: EventWriter<EnemyKill>) {
    if keyboard_input.just_pressed(KeyCode::A) {
        println!("Player attack");
        let damage = player_query.get_single().unwrap();
        attack_writer.send(PlayerAttack(*damage.1))
    }

    if keyboard_input.just_pressed(KeyCode::K) {
        println!("Enemy kill");
        let damage = enemy_query.get_single().unwrap();
        kill_writer.send(EnemyKill(*damage.1))
    }
}

fn check_win(
    mut exit: EventWriter<AppExit>,
    player_query: Query<&Xp, (With<Player>, Changed<Xp>)>,
    enemy_query: Query<&Xp, (With<Enemy>, Changed<Xp>)>) {

    if !player_query.is_empty() {
        let player = player_query.get_single();
        if player.unwrap().0 <= 0 {
            print!("Enemy wins !!!");
            exit.send(AppExit);
        }
    }

    if !enemy_query.is_empty() {
        let enemy = enemy_query.get_single();
        if enemy.unwrap().0 <= 0 {
            print!("Player wins !!!");
            exit.send(AppExit);
        }
    }
}

fn print_player_info(
    mut enemy_kill: EventReader<EnemyKill>,
    mut query: Query<(&mut Xp, &Damage), With<Player>>) {
    for x in enemy_kill.iter() {
        let damage = x.0.0;
        let mut player = query.get_single_mut().unwrap();
        (*(player.0)).0 -= damage;
        println!("kill player,{}xp left.", player.0.0);
    }
}

fn print_enemy_info(
    mut player_attack: EventReader<PlayerAttack>,
    mut query: Query<(&mut Xp, &Damage), With<Enemy>>) {
    for x in player_attack.iter() {
        let damage = x.0.0;
        let mut enemy = query.get_single_mut().unwrap();
        (*(enemy.0)).0 -= damage;
        println!("attack enemy,{}xp left.", enemy.0.0);
    }
}