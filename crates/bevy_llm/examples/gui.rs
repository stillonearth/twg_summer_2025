use bevy::prelude::*;
use bevy_llm::*;

#[derive(Component)]
struct ResponseText(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CraneAiPlugin)
        .add_systems(Startup, (setup_ui, send_test_prompt))
        .add_systems(
            Update,
            (read_generation_response, read_async_generation_response),
        )
        .run();
}

// Setup the UI camera and initial text
fn setup_ui(mut commands: Commands) {
    // Spawn a camera for UI rendering
    commands.spawn(Camera2d);

    // Spawn initial text
    commands.spawn((
        Text::new("bevy_llm"),
        ResponseText(String::new()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Left),
    ));
}

// Send a test prompt once
fn send_test_prompt(mut ev_gen_req: EventWriter<AiGenerationRequest>) {
    let messages = vec![
        ChatMessage::system("You are a helpful assistant."),
        ChatMessage::user("What is the capital of France?"),
    ];
    let request = AiGenerationRequest::with_config(
        1,         // request ID
        messages,  // messages
        Some(50),  // max_tokens
        Some(0.7), // temperature
    );
    ev_gen_req.write(request);
}

// Read and display the AI response on screen
fn read_generation_response(
    mut ev_responses: EventReader<AiGenerationResponse>,
    mut text_query: Query<&mut Text, With<ResponseText>>,
) {
    for response in ev_responses.read() {
        for (mut text) in text_query.iter_mut() {
            *text = Text::new(format!("[Response ID {}] {}", response.id, response.result));
        }
    }
}

fn read_async_generation_response(
    mut ev_responses: EventReader<AsyncAiGenerationResponse>,
    mut text_query: Query<(&mut Text, &mut ResponseText)>,
) {
    return;
    for response in ev_responses.read() {
        for (mut text, mut response_text) in text_query.iter_mut() {
            response_text.0 = format!("{}{}", response_text.0, response.result);
            *text = Text::new(format!("[Response ID {}] {}", response.id, response_text.0));
        }
    }
}
