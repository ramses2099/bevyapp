use bevy::prelude::*;

use crate::tutorial::components::{ Position, Person, Name };

pub fn print_position_system(query: Query<&Position>) {
    for pos in query.iter() {
        println!("Position: {:?}", pos);
    }
}

pub fn add_people_system(mut commands: Commands) {
    commands.spawn((Person, Name("John Smith".to_string())));
    commands.spawn((Person, Name("Carla Perez".to_string())));
    commands.spawn((Person, Name("Carmen de lo santos".to_string())));
}

pub fn greet_person_system(query:Query<&Name, With<Person>>){
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