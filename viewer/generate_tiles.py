#!/usr/bin/env python3
"""
Generate map tiles from the Elden Ring map JPEG for use with Leaflet.js

This script splits a large image into 256x256 tiles at multiple zoom levels.
The output follows the standard tile naming convention: tiles/{z}/{x}/{y}.jpg

Usage:
    python generate_tiles.py [input_image] [output_dir]
    
    Defaults:
        input_image: fextralife_map.jpg
        output_dir: tiles
"""

import os
import sys
import math
from PIL import Image

# Disable decompression bomb check for large map images
Image.MAX_IMAGE_PIXELS = None

# Configuration
TILE_SIZE = 256
JPEG_QUALITY = 85


def calculate_zoom_levels(width, height):
    """Calculate the number of zoom levels needed."""
    max_dimension = max(width, height)
    # We want the highest zoom level to be at or near native resolution
    # Each zoom level doubles the number of tiles
    max_zoom = math.ceil(math.log2(max_dimension / TILE_SIZE))
    return max_zoom


def generate_tiles(input_path, output_dir):
    """Generate tiles from the input image."""
    
    print(f"Loading image: {input_path}")
    img = Image.open(input_path)
    original_width, original_height = img.size
    print(f"Original size: {original_width} x {original_height}")
    
    max_zoom = calculate_zoom_levels(original_width, original_height)
    print(f"Generating {max_zoom + 1} zoom levels (0 to {max_zoom})")
    
    # Create output directory
    os.makedirs(output_dir, exist_ok=True)
    
    # For Leaflet with CRS.Simple, we need to work with a power-of-2 sized image
    # at the highest zoom level. We'll pad the image to the nearest power of 2.
    max_tiles = 2 ** max_zoom
    padded_size = max_tiles * TILE_SIZE
    
    print(f"Padded size at max zoom: {padded_size} x {padded_size}")
    print(f"Tiles at max zoom: {max_tiles} x {max_tiles}")
    
    # Create padded image with background matching the page background (#1a1a2e)
    padded_img = Image.new('RGB', (padded_size, padded_size), (26, 26, 46))
    # Paste original image at top-left
    padded_img.paste(img, (0, 0))
    
    total_tiles = 0
    
    # Generate tiles for each zoom level
    for z in range(max_zoom + 1):
        zoom_dir = os.path.join(output_dir, str(z))
        os.makedirs(zoom_dir, exist_ok=True)
        
        # Number of tiles at this zoom level
        num_tiles = 2 ** z
        # Size of the image at this zoom level
        zoom_size = num_tiles * TILE_SIZE
        
        print(f"\nZoom level {z}: {num_tiles}x{num_tiles} tiles ({zoom_size}x{zoom_size} px)")
        
        # Resize the padded image to this zoom level's size
        if zoom_size != padded_size:
            zoom_img = padded_img.resize((zoom_size, zoom_size), Image.Resampling.LANCZOS)
        else:
            zoom_img = padded_img
        
        # Generate tiles
        for x in range(num_tiles):
            x_dir = os.path.join(zoom_dir, str(x))
            os.makedirs(x_dir, exist_ok=True)
            
            for y in range(num_tiles):
                # Extract tile
                left = x * TILE_SIZE
                top = y * TILE_SIZE
                right = left + TILE_SIZE
                bottom = top + TILE_SIZE
                
                tile = zoom_img.crop((left, top, right, bottom))
                
                # Save tile
                tile_path = os.path.join(x_dir, f"{y}.jpg")
                tile.save(tile_path, "JPEG", quality=JPEG_QUALITY)
                total_tiles += 1
        
        print(f"  Generated {num_tiles * num_tiles} tiles")
    
    print(f"\n{'='*50}")
    print(f"Total tiles generated: {total_tiles}")
    print(f"Output directory: {output_dir}")
    
    # Save metadata for the viewer
    metadata = {
        'original_width': original_width,
        'original_height': original_height,
        'padded_size': padded_size,
        'max_zoom': max_zoom,
        'tile_size': TILE_SIZE
    }
    
    metadata_path = os.path.join(output_dir, 'metadata.json')
    import json
    with open(metadata_path, 'w') as f:
        json.dump(metadata, f, indent=2)
    print(f"Metadata saved to: {metadata_path}")
    
    return metadata


if __name__ == '__main__':
    input_image = sys.argv[1] if len(sys.argv) > 1 else 'fextralife_map.jpg'
    output_dir = sys.argv[2] if len(sys.argv) > 2 else 'tiles'
    
    if not os.path.exists(input_image):
        print(f"Error: Input image not found: {input_image}")
        sys.exit(1)
    
    generate_tiles(input_image, output_dir)
    print("\nDone! You can now use the tiles with Leaflet.js")


