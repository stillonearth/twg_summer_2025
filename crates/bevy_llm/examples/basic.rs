use bevy::prelude::*;
use bevy_llm::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CraneAiPlugin)
        .add_systems(Startup, send_test_prompt)
        .add_systems(Update, read_response)
        .run();
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

// Read and print the AI response
fn read_response(mut ev_responses: EventReader<AiGenerationResponse>) {
    for response in ev_responses.read() {
        match &response.result {
            Ok(text) => {
                println!("[Response ID {}] AI said:\n{}", response.id, text);
            }
            Err(err) => {
                eprintln!("[Response ID {}] Error: {}", response.id, err);
            }
        }
    }
}
