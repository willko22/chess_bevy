use bevy::prelude::*;


fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, print_persons)
        .run();
}


pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alice".to_string(),
            age: 30,
        }, 
        Employed {
            job: Job::Programmer
        }
    ));
    commands.spawn(Person {
        name: "Bob".to_string(),
        age: 25,
    });
}

pub fn print_persons(query: Query<&Person>) {
    println!("Name | Age");
    for person in query.iter() {
        println!("{} | {}", person.name, person.age);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
    pub age: u16,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

pub enum Job {
    Programmer,
    Artist,
    Musician,
}

