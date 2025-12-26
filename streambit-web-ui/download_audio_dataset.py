from datasets import load_dataset
import os
import shutil

def download_audio_dataset(dataset_name="PolyAI/minds14", split="train", subset="en-US", limit=10, output_dir="output/datasets/audio"):
    """
    Download audio dataset from Hugging Face
    
    Args:
        dataset_name: Name of the dataset (default: PolyAI/minds14 - small audio dataset)
        split: Dataset split (train/test/validation)
        subset: Language subset for minds14
        limit: Number of audio files to download
        output_dir: Output directory
    """
    print(f"[INFO] Downloading {limit} audio files from dataset '{dataset_name}'...")
    
    # Clean up old directory
    if os.path.exists(output_dir):
        print(f"[INFO] Clearing old data in '{output_dir}'...")
        shutil.rmtree(output_dir)
    
    # Create output directory
    os.makedirs(output_dir, exist_ok=True)
    
    try:
        # Load dataset
        print(f"[INFO] Loading dataset: {dataset_name} ({subset})...")
        dataset = load_dataset(dataset_name, subset, split=split, streaming=True)
        
        count = 0
        for item in dataset:
            if count >= limit:
                break
            
            # Get audio data
            audio = item['audio']
            audio_array = audio['array']
            sample_rate = audio['sampling_rate']
            
            # Save as WAV file
            import soundfile as sf
            filename = os.path.join(output_dir, f"audio_{count:04d}.wav")
            sf.write(filename, audio_array, sample_rate)
            
            print(f"  ✓ Saved: {filename} ({sample_rate}Hz)")
            count += 1
        
        print(f"[SUCCESS] Successfully downloaded {count} audio files to '{output_dir}'")
        return output_dir
        
    except Exception as e:
        print(f"[ERROR] Error downloading dataset: {e}")
        print(f"[INFO] Make sure you have soundfile installed: pip install soundfile")
        return None

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Download audio dataset from Hugging Face")
    parser.add_argument("--limit", type=int, default=10, help="Number of audio files to download")
    parser.add_argument("--dataset", type=str, default="PolyAI/minds14", help="Dataset name")
    parser.add_argument("--subset", type=str, default="en-US", help="Dataset subset")
    parser.add_argument("--output", type=str, default="output/datasets/audio", help="Output directory")
    
    args = parser.parse_args()
    
    download_audio_dataset(
        dataset_name=args.dataset,
        subset=args.subset,
        limit=args.limit,
        output_dir=args.output
    )
