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
    if len(sys.argv) < 2:
        print("Usage: python benchmark_python.py <image1> <image2> ...")
        sys.exit(1)
    
    image_paths = sys.argv[1:]
    
    # Verify all files exist
    for path in image_paths:
        if not Path(path).exists():
            print(f"Error: File not found: {path}")
            sys.exit(1)
    
    # Run benchmark
    result = process_images_pillow(image_paths)
    
    # Output as JSON
    print(json.dumps(result))

if __name__ == "__main__":
    main()
