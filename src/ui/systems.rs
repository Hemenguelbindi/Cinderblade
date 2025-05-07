use crate::prelude::*;
use crate::states::GameState;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = spawn_menu_root(&mut commands);
    let button = spawn_main_button(&mut commands, asset_server);
    commands.entity(root).add_child(button);
}



fn spawn_menu_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ))
        .id()
}


fn spawn_main_button(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
    let font = asset_server.load("fonts/Roboto-VariableFont_wdth,wght.ttf");
    
    let container_node = Node {
        width: Val::Px(250.),
        height: Val::Px(250.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button_node = Node{
        width: Val::Px(250.0),
        height: Val::Px(65.),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_node = Text::new("Нажми меня");
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
    let button_text_font = TextFont{
        font,
        font_size: 20.,
        ..default()
    };

    let container = commands.spawn(container_node).id();
    let button = commands.spawn((button_node, Button, BorderColor(Color::BLACK), BackgroundColor(Color::srgb(0.3, 0.3, 0.3)), TextLayout::new_with_justify(JustifyText::Center))).id();

    let button_text = commands
    .spawn((button_text_node, button_text_color, button_text_font))
    .id();
    commands.entity(button).add_children(&[button_text]);
    commands.entity(container).add_children(&[button]);

    container
}


pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
                println!("Кнопка нажата!");
                
                next_state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}