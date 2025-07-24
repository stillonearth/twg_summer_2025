use bevy::prelude::*;
use crane_core::{
    autotokenizer::AutoTokenizer,
    chat::Role,
    generation::{
        based::ModelForCausalLM,
        streamer::{AsyncTextStreamer, StreamerMessage},
        GenerationConfig,
    },
    models::{qwen25::Model as Qwen25Model, DType, Device},
    Msg,
};
use log;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

pub struct LLMPlugin;

impl Plugin for LLMPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ai_model)
            .add_systems(
                Update,
                (
                    handle_generation_requests,
                    handle_generation_responses,
                    handle_async_generation_responses,
                ),
            )
            .add_event::<AiGenerationRequest>()
            .add_event::<AiGenerationResponse>()
            .add_event::<AsyncAiGenerationResponse>()
            .init_resource::<AiModelResource>();
    }
}

#[derive(Resource, Default)]
pub struct AiModelResource {
    pub model: Option<Arc<Mutex<Qwen25Model>>>,
    pub tokenizer: Option<AutoTokenizer>,
    pub generation_config: Option<GenerationConfig>,
    pub request_sender: Option<mpsc::UnboundedSender<GenerationTask>>,
    pub generation_response_receiver: Option<mpsc::UnboundedReceiver<GenerationResult>>,
    pub async_generation_response_sender: Option<mpsc::UnboundedSender<AsyncGenerationResult>>,
    pub async_generation_response_receiver: Option<mpsc::UnboundedReceiver<AsyncGenerationResult>>,
    pub is_initialized: bool,
}

#[derive(Event)]
pub struct AiGenerationRequest {
    pub id: u32,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Event)]
pub struct AiGenerationResponse {
    pub id: u32,
    pub result: String,
}

#[derive(Event)]
pub struct AsyncAiGenerationResponse {
    pub id: u32,
    pub result: String,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }
}

pub struct GenerationTask {
    id: u32,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    async_sender: mpsc::UnboundedSender<AsyncGenerationResult>,
}

pub struct GenerationResult {
    id: u32,
    result: String,
}

pub struct AsyncGenerationResult {
    id: u32,
    result: String,
}

#[derive(Component)]
pub struct AiConfig {
    pub model_path: String,
    pub dtype: DType,
    pub device: Device,
    pub max_new_tokens: usize,
    pub temperature: f64,
    pub top_p: f64,
    pub repetition_penalty: f32,
    pub repeat_last_n: usize,
    pub do_sample: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            model_path: "checkpoints/Qwen2.5-0.5B-Instruct".to_string(),
            dtype: DType::F16,
            device: Device::Cpu,
            max_new_tokens: 235,
            temperature: 0.67,
            top_p: 1.0,
            repetition_penalty: 1.1,
            repeat_last_n: 1,
            do_sample: false,
        }
    }
}

fn setup_ai_model(mut ai_resource: ResMut<AiModelResource>) {
    // Skip initialization if already done
    if ai_resource.is_initialized {
        return;
    }

    let config = AiConfig::default();

    // Initialize tokenizer
    match AutoTokenizer::from_pretrained(&config.model_path, None) {
        Ok(tokenizer) => {
            log::info!("Successfully loaded tokenizer from: {}", config.model_path);

            // Initialize model
            match Qwen25Model::new(&config.model_path, &config.device, &config.dtype) {
                Ok(model) => {
                    log::info!("Successfully loaded AI model");

                    // Create generation config
                    let gen_config = GenerationConfig {
                        max_new_tokens: config.max_new_tokens,
                        temperature: Some(config.temperature),
                        top_p: Some(config.top_p),
                        repetition_penalty: config.repetition_penalty,
                        repeat_last_n: config.repeat_last_n,
                        do_sample: config.do_sample,
                        pad_token_id: tokenizer.get_token("<|endoftext|>"),
                        eos_token_id: tokenizer.get_token("<|im_start|>"),
                        report_speed: true,
                    };

                    // Create channels for async communication
                    let (req_tx, mut req_rx) = mpsc::unbounded_channel::<GenerationTask>();
                    let (res_tx, res_rx) = mpsc::unbounded_channel::<GenerationResult>();
                    let (async_res_tx, async_res_rx) =
                        mpsc::unbounded_channel::<AsyncGenerationResult>();

                    // Move model to Arc<Mutex<>> for thread safety
                    let model_arc = Arc::new(Mutex::new(model));
                    let tokenizer_clone = tokenizer.clone();
                    let gen_config_clone = gen_config.clone();

                    // Clone the Arc for the background thread
                    let model_arc_clone = model_arc.clone();

                    // Spawn background thread for AI generation
                    thread::spawn(move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async move {
                            while let Some(task) = req_rx.recv().await {
                                let result = generate_response(
                                    &model_arc_clone,
                                    &tokenizer_clone,
                                    &gen_config_clone,
                                    task.id,
                                    task.messages,
                                    task.max_tokens,
                                    task.temperature,
                                    task.async_sender,
                                )
                                .await;

                                if let Ok(result) = result {
                                    if let Some(llm_result) = extract_between_markers(&result) {
                                        if let Err(e) = res_tx.send(GenerationResult {
                                            id: task.id,
                                            result: llm_result,
                                        }) {
                                            log::error!("Failed to send generation result: {e}");
                                        }
                                    }
                                }
                            }
                        });
                    });

                    // Update resources
                    ai_resource.model = Some(model_arc);
                    ai_resource.tokenizer = Some(tokenizer);
                    ai_resource.generation_config = Some(gen_config);
                    ai_resource.request_sender = Some(req_tx);
                    ai_resource.generation_response_receiver = Some(res_rx);
                    ai_resource.async_generation_response_sender = Some(async_res_tx);
                    ai_resource.async_generation_response_receiver = Some(async_res_rx);
                    ai_resource.is_initialized = true;

                    log::info!("AI model initialization completed successfully");
                }
                Err(e) => {
                    log::error!("Failed to load AI model: {e}");
                }
            }
        }
        Err(e) => {
            log::error!("Failed to load tokenizer: {e}");
        }
    }
}

fn handle_generation_requests(
    mut generation_requests: EventReader<AiGenerationRequest>,
    ai_resource: Res<AiModelResource>,
) {
    if !ai_resource.is_initialized {
        return;
    }

    if let Some(request_sender) = &ai_resource.request_sender {
        if let Some(async_sender) = &ai_resource.async_generation_response_sender {
            for request in generation_requests.read() {
                let task = GenerationTask {
                    id: request.id,
                    messages: request.messages.clone(),
                    max_tokens: request.max_tokens,
                    temperature: request.temperature,
                    async_sender: async_sender.clone(),
                };

                if let Err(e) = request_sender.send(task) {
                    log::error!("Failed to send generation request: {e}");
                }
            }
        }
    }
}

fn handle_generation_responses(
    mut ai_resource: ResMut<AiModelResource>,
    mut generation_responses: EventWriter<AiGenerationResponse>,
) {
    if !ai_resource.is_initialized {
        return;
    }

    if let Some(receiver) = &mut ai_resource.generation_response_receiver {
        while let Ok(result) = receiver.try_recv() {
            generation_responses.write(AiGenerationResponse {
                id: result.id,
                result: result.result,
            });
        }
    }
}

fn handle_async_generation_responses(
    mut ai_resource: ResMut<AiModelResource>,
    mut generation_responses: EventWriter<AsyncAiGenerationResponse>,
) {
    if !ai_resource.is_initialized {
        return;
    }

    if let Some(receiver) = &mut ai_resource.async_generation_response_receiver {
        while let Ok(result) = receiver.try_recv() {
            generation_responses.write(AsyncAiGenerationResponse {
                id: result.id,
                result: result.result,
            });
        }
    }
}

async fn generate_response(
    model_arc: &Arc<Mutex<Qwen25Model>>,
    tokenizer: &AutoTokenizer,
    gen_config: &GenerationConfig,
    request_id: u32,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    async_sender: mpsc::UnboundedSender<AsyncGenerationResult>,
) -> Result<String, String> {
    // Create a custom streamer that sends async responses with the correct ID
    let (mut custom_streamer, receiver) = AsyncTextStreamer::new(tokenizer.clone());

    // Start a thread to handle streaming tokens for this specific request
    let async_sender_clone = async_sender.clone();
    std::thread::spawn(move || {
        for message in receiver {
            match message {
                StreamerMessage::Token(result) => {
                    if let Err(e) = async_sender_clone.send(AsyncGenerationResult {
                        id: request_id,
                        result,
                    }) {
                        log::error!("Failed to send async generation result: {e}");
                        break;
                    }
                }
                StreamerMessage::End => {
                    log::info!("Streaming completed for request {request_id}");
                    break;
                }
            }
        }
    });

    // Convert ChatMessage to crane_core format
    let chats: Vec<_> = messages
        .into_iter()
        .map(|msg| Msg!(msg.role, msg.content))
        .collect();

    // Apply chat template
    let prompt = tokenizer
        .apply_chat_template(&chats, true)
        .map_err(|e| format!("Failed to apply chat template: {e}"))?;

    // Lock the model for generation
    let mut model = model_arc
        .lock()
        .map_err(|e| format!("Failed to lock model: {e}"))?;

    // Prepare inputs
    let input_ids = model
        .prepare_inputs(&prompt)
        .map_err(|e| format!("Failed to prepare inputs: {e}"))?;

    // Create custom generation config if needed
    let mut custom_config = gen_config.clone();
    if let Some(max_tokens) = max_tokens {
        custom_config.max_new_tokens = max_tokens as usize;
    }
    if let Some(temp) = temperature {
        custom_config.temperature = Some(temp as f64);
    }

    // Generate response with the custom streamer
    let output_ids = model
        .generate(&input_ids, &custom_config, Some(&mut custom_streamer))
        .map_err(|e| format!("Generation failed: {e}"))?;

    // Decode the response
    let response = tokenizer
        .decode(&output_ids, false)
        .map_err(|e| format!("Failed to decode response: {e}"))?;

    Ok(response)
}

// Helper functions for easy usage
impl AiGenerationRequest {
    pub fn new(id: u32, messages: Vec<ChatMessage>) -> Self {
        Self {
            id,
            messages,
            max_tokens: None,
            temperature: None,
        }
    }

    pub fn with_config(
        id: u32,
        messages: Vec<ChatMessage>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Self {
        Self {
            id,
            messages,
            max_tokens,
            temperature,
        }
    }
}

fn extract_between_markers(text: &str) -> Option<String> {
    let re = Regex::new(r"<\|im_start\|>assistant\s*(.*?)\s*<\|im_end\|>").unwrap();
    re.captures(text)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}
