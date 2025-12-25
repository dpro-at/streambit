use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use streambit_core::StreamBitError;

/// Apply watermark to image
/// 
/// # Arguments
/// * `img` - Input image
/// * `watermark` - Watermark image
/// * `x` - X position for watermark
/// * `y` - Y position for watermark
/// * `opacity` - Opacity of watermark (0.0-1.0)
pub fn apply_watermark(
    img: &DynamicImage,
    watermark: &DynamicImage,
    x: u32,
    y: u32,
    opacity: f32,
) -> Result<DynamicImage, StreamBitError> {
    let mut base = img.to_rgba8();
    let wm = watermark.to_rgba8();
    
    let (base_width, base_height) = base.dimensions();
    let (wm_width, wm_height) = wm.dimensions();
    
    if x + wm_width > base_width || y + wm_height > base_height {
        return Err(StreamBitError::Image(
            "Watermark exceeds image boundaries".to_string()
        ));
    }
    
    let opacity = opacity.max(0.0).min(1.0);
    
    for wy in 0..wm_height {
        for wx in 0..wm_width {
            let wm_pixel = wm.get_pixel(wx, wy);
            let base_pixel = base.get_pixel(x + wx, y + wy);
            
            let alpha = (wm_pixel[3] as f32 / 255.0) * opacity;
            
            let r = ((wm_pixel[0] as f32 * alpha) + (base_pixel[0] as f32 * (1.0 - alpha))) as u8;
            let g = ((wm_pixel[1] as f32 * alpha) + (base_pixel[1] as f32 * (1.0 - alpha))) as u8;
            let b = ((wm_pixel[2] as f32 * alpha) + (base_pixel[2] as f32 * (1.0 - alpha))) as u8;
            
            base.put_pixel(x + wx, y + wy, image::Rgba([r, g, b, base_pixel[3]]));
        }
    }
    
    Ok(DynamicImage::ImageRgba8(base))
}

/// Normalize color values to 0-255 range
pub fn normalize_colors(img: &DynamicImage) -> DynamicImage {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    // Find min and max values for each channel
    let mut min_r = 255u8;
    let mut max_r = 0u8;
    let mut min_g = 255u8;
    let mut max_g = 0u8;
    let mut min_b = 255u8;
    let mut max_b = 0u8;
    
    for pixel in rgb_img.pixels() {
        min_r = min_r.min(pixel[0]);
        max_r = max_r.max(pixel[0]);
        min_g = min_g.min(pixel[1]);
        max_g = max_g.max(pixel[1]);
        min_b = min_b.min(pixel[2]);
        max_b = max_b.max(pixel[2]);
    }
    
    let mut output = RgbImage::new(width, height);
    
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let r = if max_r > min_r {
            ((pixel[0] - min_r) as f32 / (max_r - min_r) as f32 * 255.0) as u8
        } else {
            pixel[0]
        };
        
        let g = if max_g > min_g {
            ((pixel[1] - min_g) as f32 / (max_g - min_g) as f32 * 255.0) as u8
        } else {
            pixel[1]
        };
        
        let b = if max_b > min_b {
            ((pixel[2] - min_b) as f32 / (max_b - min_b) as f32 * 255.0) as u8
        } else {
            pixel[2]
        };
        
        output.put_pixel(x, y, Rgb([r, g, b]));
    }
    
    DynamicImage::ImageRgb8(output)
}

/// Auto-enhance image (brightness, contrast, and saturation)
pub fn auto_enhance(img: &DynamicImage) -> DynamicImage {
    use crate::enhancements::{adjust_brightness, adjust_contrast};
    
    // First normalize
    let normalized = normalize_colors(img);
    
    // Then apply moderate contrast boost
    let contrasted = adjust_contrast(&normalized, 1.2);
    
    // Finally slight brightness adjustment
    adjust_brightness(&contrasted, 1.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    fn create_test_image() -> DynamicImage {
        let img = RgbImage::from_fn(100, 100, |x, y| {
            Rgb([(x % 256) as u8, (y % 256) as u8, 128])
        });
        DynamicImage::ImageRgb8(img)
    }

    fn create_watermark() -> DynamicImage {
        let img = RgbImage::from_fn(20, 20, |_, _| {
            Rgb([255, 0, 0])
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_watermark() {
        let img = create_test_image();
        let wm = create_watermark();
        let result = apply_watermark(&img, &wm, 10, 10, 0.5).unwrap();
        assert_eq!(result.dimensions(), img.dimensions());
    }

    #[test]
    fn test_normalize() {
        let img = create_test_image();
        let normalized = normalize_colors(&img);
        assert_eq!(normalized.dimensions(), img.dimensions());
    }

    #[test]
    fn test_auto_enhance() {
        let img = create_test_image();
        let enhanced = auto_enhance(&img);
        assert_eq!(enhanced.dimensions(), img.dimensions());
    }
}
