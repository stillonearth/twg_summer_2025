import json
import os
import io
import time
import random
from PIL import Image
import requests
import websocket
import threading
from urllib.parse import urlparse

# Define the API endpoint and the output directory
COMFYUI_SERVER = "http://localhost:8188"
OUTPUT_DIR = "./processed_images"
os.makedirs(OUTPUT_DIR, exist_ok=True)

# Your ComfyUI workflow template
WORKFLOW_TEMPLATE = {
    "6": {
        "inputs": {
            "text": "photo of a cat holding up a sign that says \"FLUX DEV FP 8\"",
            "clip": ["30", 1]
        },
        "class_type": "CLIPTextEncode",
        "_meta": {
            "title": "CLIP Text Encode (Positive Prompt)"
        }
    },
    "8": {
        "inputs": {
            "samples": ["31", 0],
            "vae": ["30", 2]
        },
        "class_type": "VAEDecode",
        "_meta": {
            "title": "VAE Decode"
        }
    },
    "9": {
        "inputs": {
            "filename_prefix": "ComfyUI",
            "images": ["8", 0]
        },
        "class_type": "SaveImage",
        "_meta": {
            "title": "Save Image"
        }
    },
    "27": {
        "inputs": {
            "width": 1024,
            "height": 1024,
            "batch_size": 1
        },
        "class_type": "EmptySD3LatentImage",
        "_meta": {
            "title": "EmptySD3LatentImage"
        }
    },
    "30": {
        "inputs": {
            "ckpt_name": "flux1-dev-fp8.safetensors"
        },
        "class_type": "CheckpointLoaderSimple",
        "_meta": {
            "title": "Load Checkpoint"
        }
    },
    "31": {
        "inputs": {
            "seed": 484528868454858,
            "steps": 20,
            "cfg": 1,
            "sampler_name": "euler",
            "scheduler": "simple",
            "denoise": 1,
            "model": ["30", 0],
            "positive": ["35", 0],
            "negative": ["33", 0],
            "latent_image": ["27", 0]
        },
        "class_type": "KSampler",
        "_meta": {
            "title": "KSampler"
        }
    },
    "33": {
        "inputs": {
            "text": "",
            "clip": ["30", 1]
        },
        "class_type": "CLIPTextEncode",
        "_meta": {
            "title": "CLIP Text Encode (Negative Prompt)"
        }
    },
    "35": {
        "inputs": {
            "guidance": 3.5,
            "conditioning": ["6", 0]
        },
        "class_type": "FluxGuidance",
        "_meta": {
            "title": "FluxGuidance"
        }
    }
}

def generate_client_id():
    """Generate a unique client ID for this session"""
    return f"python_client_{random.randint(1000, 9999)}"

def queue_prompt(workflow, client_id):
    """Queue a prompt for generation"""
    prompt_data = {
        "prompt": workflow,
        "client_id": client_id
    }

    response = requests.post(f"{COMFYUI_SERVER}/prompt", json=prompt_data)
    if response.status_code == 200:
        return response.json()
    else:
        print(f"Error queuing prompt: {response.status_code} - {response.text}")
        return None

def get_history(prompt_id):
    """Get the history for a specific prompt ID"""
    response = requests.get(f"{COMFYUI_SERVER}/history/{prompt_id}")
    if response.status_code == 200:
        return response.json()
    else:
        return None

def wait_for_completion(prompt_id, timeout=300):
    """Wait for the prompt to complete processing"""
    start_time = time.time()
    while time.time() - start_time < timeout:
        history = get_history(prompt_id)
        if history and prompt_id in history:
            return history[prompt_id]
        time.sleep(2)
    return None

def download_image(filename, subfolder="", folder_type="output"):
    """Download the generated image"""
    url = f"{COMFYUI_SERVER}/view"
    params = {
        "filename": filename,
        "subfolder": subfolder,
        "type": folder_type
    }

    response = requests.get(url, params=params)
    if response.status_code == 200:
        return response.content
    else:
        print(f"Error downloading image: {response.status_code}")
        return None

def generate_and_download_image(prompt_text, output_filename):
    """Generate an image using ComfyUI and download it"""
    # Create a copy of the workflow template
    workflow = json.loads(json.dumps(WORKFLOW_TEMPLATE))

    # Update the prompt text
    workflow["6"]["inputs"]["text"] = prompt_text

    # Generate random seed for variation
    workflow["31"]["inputs"]["seed"] = random.randint(1, 1000000000000000)

    # Generate client ID
    client_id = generate_client_id()

    print(f"Generating image for: {prompt_text}")

    # Queue the prompt
    result = queue_prompt(workflow, client_id)
    if not result:
        print("Failed to queue prompt")
        return False

    prompt_id = result["prompt_id"]
    print(f"Prompt queued with ID: {prompt_id}")

    # Wait for completion
    print("Waiting for image generation to complete...")
    history = wait_for_completion(prompt_id)
    if not history:
        print("Timeout waiting for image generation")
        return False

    # Get the output images
    if "outputs" in history and "9" in history["outputs"]:
        images = history["outputs"]["9"]["images"]
        if images:
            # Download the first image
            image_info = images[0]
            filename = image_info["filename"]
            subfolder = image_info.get("subfolder", "")

            print(f"Downloading image: {filename}")
            image_data = download_image(filename, subfolder)

            if image_data:
                # Process and save the image
                with Image.open(io.BytesIO(image_data)) as img:
                    # Resize to your desired dimensions
                    img = img.resize((360, 360))

                    # Save the processed image
                    output_path = os.path.join(OUTPUT_DIR, f"{output_filename}.png")
                    img.save(output_path)
                    print(f"Processed and saved {output_filename}.png")
                    return True
            else:
                print("Failed to download image")
                return False
        else:
            print("No images found in output")
            return False
    else:
        print("No outputs found in history")
        return False

def main():
    """Main function to process cards.json"""
    try:
        with open("cards.json") as json_file:
            data = json.load(json_file)

            for i, item in enumerate(data):
                prompt = item.get("prompt")
                filename = f"card-picture-{i}"

                if prompt:
                    success = generate_and_download_image(prompt, filename)
                    if not success:
                        print(f"Failed to generate image for prompt: {prompt}")

                    # Add a small delay between requests to avoid overwhelming the server
                    time.sleep(1)
                else:
                    print(f"No prompt found for item {i}")

    except FileNotFoundError:
        print("cards.json file not found")
    except json.JSONDecodeError:
        print("Error parsing cards.json file")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()
