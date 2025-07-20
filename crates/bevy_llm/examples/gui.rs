use bevy::prelude::*;
use bevy_llm::*;

#[derive(Component)]
struct ResponseText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CraneAiPlugin)
        .add_systems(Startup, (setup_ui, send_test_prompt))
        .add_systems(Update, read_response)
        .run();
}

// Setup the UI camera and initial text
fn setup_ui(mut commands: Commands) {
    // Spawn a camera for UI rendering
    commands.spawn(Camera2d);

    // Spawn initial text
    commands.spawn((
        Text::new("bevy_llm"),
        ResponseText,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        Name::new("Novel Text"),
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
fn read_response(
    mut ev_responses: EventReader<AiGenerationResponse>,
    mut text_query: Query<&mut Text, With<ResponseText>>,
) {
    for response in ev_responses.read() {
        for (mut text) in text_query.iter_mut() {
            match &response.result {
                Ok(ai_text) => {
                    *text = Text::new(format!(
                        "[Response ID {}] AI said:\n\n{}",
                        response.id, ai_text
                    ));
                }
                Err(err) => {
                    *text = Text::new(format!("[Response ID {}] Error:\n\n{}", response.id, err));
                }
            }
        }
    }
}
