use bevy::prelude::*;
use crane_core::{
    autotokenizer::AutoTokenizer,
    chat::Role,
    generation::{based::ModelForCausalLM, streamer::TextStreamer, GenerationConfig},
    models::{qwen25::Model as Qwen25Model, DType, Device},
    Msg,
};
use log::*;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

pub struct CraneAiPlugin;

impl Plugin for CraneAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ai_model)
            .add_systems(
                Update,
                (handle_generation_requests, handle_generation_responses),
            )
            .add_event::<AiGenerationRequest>()
            .add_event::<AiGenerationResponse>()
            .init_resource::<AiModelResource>();
    }
}

#[derive(Resource)]
pub struct AiModelResource {
    pub model: Arc<Mutex<Qwen25Model>>,
    pub tokenizer: AutoTokenizer,
    pub generation_config: GenerationConfig,
    pub request_sender: mpsc::UnboundedSender<GenerationTask>,
    pub response_receiver: mpsc::UnboundedReceiver<GenerationResult>,
    pub is_initialized: bool,
}

impl Default for AiModelResource {
    fn default() -> Self {
        // Create dummy channels that will be replaced during initialization
        let (req_tx, _req_rx) = mpsc::unbounded_channel::<GenerationTask>();
        let (_res_tx, res_rx) = mpsc::unbounded_channel::<GenerationResult>();

        // Create a dummy model - this will be replaced during initialization
        let config = AiConfig::default();
        let dummy_model = Qwen25Model::new(&config.model_path, &config.device, &config.dtype)
            .expect("Failed to create dummy model");
        let dummy_tokenizer = AutoTokenizer::from_pretrained(&config.model_path, None)
            .expect("Failed to create dummy tokenizer");

        let dummy_gen_config = GenerationConfig {
            max_new_tokens: config.max_new_tokens,
            temperature: Some(config.temperature),
            top_p: Some(config.top_p),
            repetition_penalty: config.repetition_penalty,
            repeat_last_n: config.repeat_last_n,
            do_sample: config.do_sample,
            pad_token_id: dummy_tokenizer.get_token("<|end_of_text|>"),
            eos_token_id: dummy_tokenizer.get_token("<|im_end|>"),
            report_speed: true,
        };

        Self {
            model: Arc::new(Mutex::new(dummy_model)),
            tokenizer: dummy_tokenizer,
            generation_config: dummy_gen_config,
            request_sender: req_tx,
            response_receiver: res_rx,
            is_initialized: false,
        }
    }
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
    pub result: Result<String, String>,
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
}

pub struct GenerationResult {
    id: u32,
    result: Result<String, String>,
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

    //// Print candle build info
    //crane_core::utils::utils::print_candle_build_info();

    // Initialize tokenizer
    match AutoTokenizer::from_pretrained(&config.model_path, None) {
        Ok(tokenizer) => {
            info!("Successfully loaded tokenizer from: {}", config.model_path);

            // Initialize model
            match Qwen25Model::new(&config.model_path, &config.device, &config.dtype) {
                Ok(mut model) => {
                    info!("Successfully loaded AI model");

                    // Warmup the model
                    let _ = model.warmup();

                    // Create generation config
                    let gen_config = GenerationConfig {
                        max_new_tokens: config.max_new_tokens,
                        temperature: Some(config.temperature),
                        top_p: Some(config.top_p),
                        repetition_penalty: config.repetition_penalty,
                        repeat_last_n: config.repeat_last_n,
                        do_sample: config.do_sample,
                        pad_token_id: tokenizer.get_token("<|end_of_text|>"),
                        eos_token_id: tokenizer.get_token("<|im_end|>"),
                        report_speed: true,
                    };

                    // Create channels for async communication
                    let (req_tx, mut req_rx) = mpsc::unbounded_channel::<GenerationTask>();
                    let (res_tx, res_rx) = mpsc::unbounded_channel::<GenerationResult>();

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
                                    task.messages,
                                    task.max_tokens,
                                    task.temperature,
                                )
                                .await;

                                let _ = res_tx.send(GenerationResult {
                                    id: task.id,
                                    result,
                                });
                            }
                        });
                    });

                    // Update resources
                    ai_resource.model = model_arc;
                    ai_resource.tokenizer = tokenizer;
                    ai_resource.generation_config = gen_config;
                    ai_resource.request_sender = req_tx;
                    ai_resource.response_receiver = res_rx;
                    ai_resource.is_initialized = true;
                }
                Err(e) => {
                    error!("Failed to load AI model: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to load tokenizer: {}", e);
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

    for request in generation_requests.read() {
        let task = GenerationTask {
            id: request.id,
            messages: request.messages.clone(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };

        if let Err(e) = ai_resource.request_sender.send(task) {
            error!("Failed to send generation request: {}", e);
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

    while let Ok(result) = ai_resource.response_receiver.try_recv() {
        generation_responses.write(AiGenerationResponse {
            id: result.id,
            result: result.result,
        });
    }
}

async fn generate_response(
    model_arc: &Arc<Mutex<Qwen25Model>>,
    tokenizer: &AutoTokenizer,
    gen_config: &GenerationConfig,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
) -> Result<String, String> {
    // Convert ChatMessage to crane_core format
    let chats: Vec<_> = messages
        .into_iter()
        .map(|msg| Msg!(msg.role, msg.content))
        .collect();

    // Apply chat template
    let prompt = tokenizer
        .apply_chat_template(&chats, true)
        .map_err(|e| format!("Failed to apply chat template: {}", e))?;

    // Lock the model for generation
    let mut model = model_arc
        .lock()
        .map_err(|e| format!("Failed to lock model: {}", e))?;

    // Prepare inputs
    let input_ids = model
        .prepare_inputs(&prompt)
        .map_err(|e| format!("Failed to prepare inputs: {}", e))?;

    // Create custom generation config if needed
    let mut custom_config = gen_config.clone();
    if let Some(max_tokens) = max_tokens {
        custom_config.max_new_tokens = max_tokens as usize;
    }
    if let Some(temp) = temperature {
        custom_config.temperature = Some(temp as f64);
    }

    // Create streamer
    let mut streamer = TextStreamer {
        tokenizer: tokenizer.clone(),
        buffer: String::new(),
    };

    // Generate response
    let output_ids = model
        .generate(&input_ids, &custom_config, Some(&mut streamer))
        .map_err(|e| format!("Generation failed: {}", e))?;

    // Decode the response
    let response = tokenizer
        .decode(&output_ids, false)
        .map_err(|e| format!("Failed to decode response: {}", e))?;

    println!("response :: {}", response);

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
