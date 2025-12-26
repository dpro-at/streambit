from datasets import load_dataset
import os
from PIL import Image
from tqdm import tqdm

import shutil

def download_image_dataset(dataset_name="beans", split="train", limit=100, output_dir="output/datasets/images"):
    print(f"[INFO] Downloading {limit} images from dataset '{dataset_name}'...")
    
    # Clean up old directory to save space
    if os.path.exists(output_dir):
        print(f"[INFO] Clearing old data in '{output_dir}'...")
        shutil.rmtree(output_dir)
    
    # Create output directory
    os.makedirs(output_dir, exist_ok=True)
    
    try:
        # Load dataset
        # "beans" is a small, clean dataset of leaf images
        dataset = load_dataset(dataset_name, split=split, streaming=True)
        
        count = 0
        for item in tqdm(dataset, total=limit, desc="Saving Images"):
            if count >= limit:
                break
                
            image = item['image']
            
            # Convert to RGB if needed
            if image.mode != "RGB":
                image = image.convert("RGB")
            
            # Save file
            filename = os.path.join(output_dir, f"image_{count:04d}.jpg")
            image.save(filename, "JPEG", quality=90)
            
            count += 1
            
        print(f"[SUCCESS] Successfully downloaded {count} images to '{output_dir}'")
        return output_dir
        
    except Exception as e:
        print(f"[ERROR] Error downloading dataset: {e}")
        return None

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Download benchmark dataset")
    parser.add_argument("--limit", type=int, default=100, help="Number of images to download")
    parser.add_argument("--dataset", type=str, default="beans", help="Dataset name")
    parser.add_argument("--output", type=str, default="output/datasets/images", help="Output directory")
    
    args = parser.parse_args()
    
    download_image_dataset(limit=args.limit, dataset_name=args.dataset, output_dir=args.output)
