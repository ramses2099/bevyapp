use bevy::prelude::*;
use crate::tutorial::components::{ Person, Name };
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people_system);
        app.add_systems(Update, (greet_person_system, update_people_system).chain());
    }
}

pub fn add_people_system(mut commands: Commands) {
    commands.spawn((Person, Name("John Smith".to_string())));
    commands.spawn((Person, Name("Carla Perez".to_string())));
    commands.spawn((Person, Name("Carmen de lo santos".to_string())));
}

pub fn greet_person_system(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello, {}!", name.0);
    }
}

pub fn update_people_system(mut query: Query<&mut Name, With<Person>>) {
    for mut name in query.iter_mut() {
        if name.0 == "John Smith" {
            name.0 = "Jane Doe".to_string();
            break;
        }
    }
}
