#!/usr/bin/env python3
"""
Python image processing benchmark for comparison with StreamBit
Uses OpenCV and Pillow to process images
"""

import sys
import time
import json
from pathlib import Path
from typing import List, Tuple

try:
    from PIL import Image
    import numpy as np
except ImportError:
    print("Error: Please install required packages:")
    print("pip install pillow numpy")
    sys.exit(1)

def process_images_pillow(image_paths: List[str], target_size: Tuple[int, int] = (224, 224)) -> dict:
    """Process images using Pillow"""
    start_time = time.time()
    
    processed = []
    for path in image_paths:
        try:
            # Load image
            img = Image.open(path)
            # Convert to RGB
            img = img.convert('RGB')
            # Resize
            img = img.resize(target_size, Image.Resampling.BILINEAR)
            # Convert to numpy array
            arr = np.array(img)
            # Convert to CHW format (Channel, Height, Width)
            arr = np.transpose(arr, (2, 0, 1))
            # Normalize to [0, 1]
            arr = arr.astype(np.float32) / 255.0
            processed.append(arr)
        except Exception as e:
            print(f"Error processing {path}: {e}")
            continue
    
    end_time = time.time()
    elapsed = (end_time - start_time) * 1000  # Convert to ms
    
    return {
        "library": "Pillow",
        "images_processed": len(processed),
        "time_ms": round(elapsed, 2),
        "throughput": round(len(processed) / (elapsed / 1000), 2) if elapsed > 0 else 0,
        "shapes": [list(arr.shape) for arr in processed]
    }

def main():
    image_paths = []
    
    if len(sys.argv) > 1 and sys.argv[1] == "--json-stdin":
        # Read from stdin as JSON
        try:
            input_data = sys.stdin.read()
            image_paths = json.loads(input_data)
        except Exception as e:
            print(f"Error reading from stdin: {e}")
            sys.exit(1)
    elif len(sys.argv) > 2 and sys.argv[1] == "--json-file":
        # Read from JSON file
        try:
            # Use utf-8-sig to handle potential BOM from PowerShell redirection
            with open(sys.argv[2], 'r', encoding='utf-8-sig') as f:
                image_paths = json.load(f)
        except Exception as e:
            print(f"Error reading from json file: {e}")
            sys.exit(1)
    else:
        # Initial check for minimum arguments if not using stdin
        if len(sys.argv) < 2:
            print("Usage: python benchmark_python.py <image1> ... OR --json-stdin OR --json-file <path>")
            sys.exit(1)
        image_paths = sys.argv[1:]
    
    # Verify all files exist
    valid_paths = []
    for path in image_paths:
        if Path(path).exists():
            valid_paths.append(path)
        else:
            # Only warn, don't exit, to be robust
            pass # print(f"Warning: File not found: {path}", file=sys.stderr)
    
    image_paths = valid_paths
    
    if not image_paths:
        print(json.dumps({
            "error": "No valid image files found",
            "images_processed": 0,
            "time_ms": 0,
            "throughput": 0
        }))
        sys.exit(0)
    
    # Run benchmark
    result = process_images_pillow(image_paths)
    
    # Output as JSON
    print(json.dumps(result))

if __name__ == "__main__":
    main()
