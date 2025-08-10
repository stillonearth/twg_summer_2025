import json
import os
import io
from PIL import Image
import requests

API_ENDPOINT = "https://kakuseinosekainokokujoninarudaikinonisemono.space/api/image"
OUTPUT_DIR = "./schizo-images"
os.makedirs(OUTPUT_DIR, exist_ok=True)


def download_and_process_image(description, filename):
    params = {"prompt": description}
    response = requests.get(API_ENDPOINT, params=params)

    if response.status_code == 200:
        image_data = response.content

        # Open the image
        with Image.open(io.BytesIO(image_data)) as img:
            # Resize and crop to 200x200 pixels
            img = img.resize((360, 200))

            # Save the processed image
            output_path = os.path.join(OUTPUT_DIR, filename)
            img.save(output_path + ".png")
            print(f"Processed and saved {filename}")
    else:
        print(f"Failed to download image for description: {description}")


def main():
    with open("schizophrenic-cards.json") as json_file:
        data = json.load(json_file)

        for i, item in enumerate(data[0:64]):

            if i not in (33,):
                continue

            description = item.get("image_prompt")
            filename = "card-picture-" + str(i)

            if description and filename:
                download_and_process_image(
                    "an image of " + description + ", NO TEXT", filename
                )


if __name__ == "__main__":
    main()
