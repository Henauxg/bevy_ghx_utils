use bevy::{
    ecs::{component::Component, query::With, system::Query},
    render::view::Visibility,
};

/// Toggles the [`Visibility`] of all `Entity` with a component of type `T`
///
/// ### Example usage
///
/// Toggles On/Off a UI view by pressing F1
/// ```rust,ignore
///  app.add_systems(
///    Update,
///    toggle_visibility::<UiRoot>.run_if(input_just_pressed(KeyCode::F1)),
///  );
/// ```
pub fn toggle_visibility<T: Component>(mut visibilities: Query<&mut Visibility, With<T>>) {
    for mut vis in &mut visibilities {
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}
