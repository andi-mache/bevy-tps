use bevy::prelude::*;



fn main() {
    App::new().add_plugins(DefaultPlugins).run();

}

// lets define a floor 
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandartMaterial>>
) {
    
}
