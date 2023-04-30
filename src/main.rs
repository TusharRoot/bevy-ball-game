use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const ENEMIES_SPEED:f32 = 200.0;
pub const PLAYER_SPEED:f32= 500.0;
pub const NUMBER_OF_ENEMIES:usize = 4;
pub const PLAYER_SIZE:f32 = 64.0;
pub const ENEMY_SIZE:f32 = 64.0;
pub const NUMBER_OF_STARS:usize = 10; 
pub const STAR_SIZE:f32 = 30.0;
pub const STAR_SPAWN_TIME:f32 = 1.0;
pub const ENEMY_SPAWN_TIME:f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_startup_system(spawn_camara)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_star)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(enemies_movement)
        .add_system(update_enemy_direction)
        .add_system(enemy_hit_player)
        .add_system(player_hit_star)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(spawn_star_over_time)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_over_time)
        .add_system(exit_game)
        .run();
}

#[derive(Component)]
pub struct Star {}
#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Enemy {
    pub direction:Vec2,
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score  {
    fn default() -> Self {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer  {
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating) }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer{
    pub timer: Timer,
}

impl Default for EnemySpawnTimer  {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (SpriteBundle{
            transform: Transform::from_xyz(window.width()/2.0,window.height()/2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camara(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle{
            transform: Transform::from_xyz(window.width()/2.0,window.height()/2.0, 0.0),
            ..default()
        }
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

     for _ in 0..NUMBER_OF_ENEMIES{
          let random_x = random::<f32>() * window.width();
          let random_y = random::<f32>() * window.height();

          commands.spawn(
                (SpriteBundle{
                transform: Transform::from_xyz(random_x,random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
          ));
    }
}

pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

    for _  in 0..NUMBER_OF_STARS{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle{
                    transform: Transform::from_xyz(random_x,random_y,0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},   
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform,With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform)=player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A){
            direction += Vec3::new(-1.0,0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D){
            direction += Vec3::new(1.0,0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0,1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0,-1.0, 0.0);
        }
 
        if direction.length() > 0.0{
            direction = direction.normalize()
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform,With<Player>>,
    window_query: Query<&Window,With<PrimaryWindow>>,
){
    if let Ok(mut player_transform) = player_query.get_single_mut(){
        let window = window_query.get_single().unwrap();
        
        let half_player_size = PLAYER_SIZE/2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0+half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min{
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemies_movement(
    mut enemy_query: Query<(&mut Transform,&Enemy)>,
    time: Res<Time>,
){
    for (mut transform, enmy) in enemy_query.iter_mut(){
        let direction = Vec3::new(enmy.direction.x,enmy.direction.y,0.0);
        transform.translation += direction * ENEMIES_SPEED * time.delta_seconds();
   }
}

pub fn update_enemy_direction(
    mut enmey_query: Query<(&Transform,&mut Enemy)>,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    audio : Res<Audio>,
){
    let window = window_query.get_single().unwrap();
        
        let half_player_size = ENEMY_SIZE/2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0+half_player_size;
        let y_max = window.height() - half_player_size;   

        for (transform,mut enmy) in enmey_query.iter_mut(){
            let mut direction_changed = false;
            let translation = transform.translation;
            if translation.x<x_min || translation.x>x_max{
                enmy.direction.x *=-1.0;
                direction_changed = true; 
            }
            if translation.y<y_min || translation.y>y_max{
                enmy.direction.y *=-1.0; 
                direction_changed = true;
            }

            //Play SFX
            if direction_changed{
                //Play Sound Effect
                let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
                let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
                
                let sound_effect = if random::<f32>() > 0.5{
                    sound_effect_1
                }else{
                    sound_effect_2
                };
                audio.play(sound_effect);
            }
        }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform,With<Enemy>>,
    window_query: Query<&Window,With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE/2.0;
        let x_min = 0.0 + half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = 0.0+half_enemy_size;
        let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut(){

        let mut translation = transform.translation;

        if translation.x < x_min{
            translation.x = x_min;
        }else if translation.x > x_max{
            translation.x = x_max;
        }

        if translation.y < y_min{
            translation.y = y_min;
        }else if translation.y > y_max{
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity,&Transform),With<Player>>,
    enemy_query: Query<&Transform,With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio : Res<Audio>,
    // mut app_exit_event_writer: EventWriter<AppExit>,
){
    if let Ok((player_entity,player_transform)) = player_query.get_single_mut(){
        for enmy_transform in enemy_query.iter(){
            let distance = player_transform
                                .translation
                                .distance(enmy_transform.translation);
            let player_radius = PLAYER_SIZE/2.0;
            let enmy_radius = ENEMY_SIZE/2.0;
            if distance < player_radius + enmy_radius{
                println!("Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
                // app_exit_event_writer.send(AppExit);
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform,With<Player>>,
    star_query : Query<(Entity,&Transform),With<Star>>,
    audio : Res<Audio>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
){
    if let Ok(player_transform) = player_query.get_single()  {
        for (star_entity,star_transform) in star_query.iter(){
            let distance = player_transform.translation.distance(star_transform.translation);

            if distance < PLAYER_SIZE/2.0 + STAR_SIZE/2.0{
                println!("Player hit star.!");
                score.value +=1;
                let sound_effect = asset_server.load("audio/impactPunch_heavy_004.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
){
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_star_over_time(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    start_spawn_timer: Res<StarSpawnTimer>,
){
    if start_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (SpriteBundle{
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
){
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window,With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>,
){
    if enemy_spawn_timer.timer.finished(){
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (SpriteBundle{
            transform: Transform::from_xyz(random_x,random_y, 0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
      ));

    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
){
    if keyboard_input.just_pressed(KeyCode::Escape){
        app_exit_event_writer.send(AppExit);
    }
}