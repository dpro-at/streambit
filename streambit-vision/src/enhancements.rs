use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb, RgbImage};
use streambit_core::StreamBitError;

/// Adjust brightness of an image
/// 
/// # Arguments
/// * `img` - Input image
/// * `factor` - Brightness factor (1.0 = no change, >1.0 = brighter, <1.0 = darker)
pub fn adjust_brightness(img: &DynamicImage, factor: f32) -> DynamicImage {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    let mut output = RgbImage::new(width, height);
    
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let r = (pixel[0] as f32 * factor).min(255.0).max(0.0) as u8;
        let g = (pixel[1] as f32 * factor).min(255.0).max(0.0) as u8;
        let b = (pixel[2] as f32 * factor).min(255.0).max(0.0) as u8;
        
        output.put_pixel(x, y, Rgb([r, g, b]));
    }
    
    DynamicImage::ImageRgb8(output)
}

/// Adjust contrast of an image
/// 
/// # Arguments
/// * `img` - Input image
/// * `factor` - Contrast factor (1.0 = no change, >1.0 = more contrast, <1.0 = less contrast)
pub fn adjust_contrast(img: &DynamicImage, factor: f32) -> DynamicImage {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    let mut output = RgbImage::new(width, height);
    
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let r = ((pixel[0] as f32 - 128.0) * factor + 128.0).min(255.0).max(0.0) as u8;
        let g = ((pixel[1] as f32 - 128.0) * factor + 128.0).min(255.0).max(0.0) as u8;
        let b = ((pixel[2] as f32 - 128.0) * factor + 128.0).min(255.0).max(0.0) as u8;
        
        output.put_pixel(x, y, Rgb([r, g, b]));
    }
    
    DynamicImage::ImageRgb8(output)
}

/// Rotation angle
#[derive(Debug, Clone, Copy)]
pub enum RotationAngle {
    Rotate90,
    Rotate180,
    Rotate270,
}

/// Rotate image by specified angle
pub fn rotate(img: &DynamicImage, angle: RotationAngle) -> DynamicImage {
    match angle {
        RotationAngle::Rotate90 => img.rotate90(),
        RotationAngle::Rotate180 => img.rotate180(),
        RotationAngle::Rotate270 => img.rotate270(),
    }
}

/// Flip image horizontally
pub fn flip_horizontal(img: &DynamicImage) -> DynamicImage {
    img.fliph()
}

/// Flip image vertically
pub fn flip_vertical(img: &DynamicImage) -> DynamicImage {
    img.flipv()
}

/// Crop image to specified region
/// 
/// # Arguments
/// * `img` - Input image
/// * `x` - X coordinate of top-left corner
/// * `y` - Y coordinate of top-left corner
/// * `width` - Width of crop region
/// * `height` - Height of crop region
pub fn crop(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Result<DynamicImage, StreamBitError> {
    let (img_width, img_height) = img.dimensions();
    
    if x + width > img_width || y + height > img_height {
        return Err(StreamBitError::Image(
            format!("Crop region ({}x{} at {},{}) exceeds image dimensions ({}x{})", 
                    width, height, x, y, img_width, img_height)
        ));
    }
    
    Ok(img.crop_imm(x, y, width, height))
}

/// Convert image to grayscale
pub fn to_grayscale(img: &DynamicImage) -> DynamicImage {
    img.grayscale()
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

    #[test]
    fn test_brightness() {
        let img = create_test_image();
        let bright = adjust_brightness(&img, 1.5);
        assert_eq!(bright.dimensions(), img.dimensions());
    }

    #[test]
    fn test_contrast() {
        let img = create_test_image();
        let contrasted = adjust_contrast(&img, 1.5);
        assert_eq!(contrasted.dimensions(), img.dimensions());
    }

    #[test]
    fn test_rotation() {
        let img = create_test_image();
        let rotated = rotate(&img, RotationAngle::Rotate90);
        // After 90° rotation, width and height should swap
        assert_eq!(rotated.width(), img.height());
        assert_eq!(rotated.height(), img.width());
    }

    #[test]
    fn test_flip() {
        let img = create_test_image();
        let flipped_h = flip_horizontal(&img);
        let flipped_v = flip_vertical(&img);
        assert_eq!(flipped_h.dimensions(), img.dimensions());
        assert_eq!(flipped_v.dimensions(), img.dimensions());
    }

    #[test]
    fn test_crop() {
        let img = create_test_image();
        let cropped = crop(&img, 10, 10, 50, 50).unwrap();
        assert_eq!(cropped.dimensions(), (50, 50));
    }

    #[test]
    fn test_grayscale() {
        let img = create_test_image();
        let gray = to_grayscale(&img);
        assert_eq!(gray.dimensions(), img.dimensions());
    }
}
