use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb, RgbImage};

/// Apply Gaussian blur to image
/// 
/// # Arguments
/// * `img` - Input image
/// * `sigma` - Blur strength (higher = more blur, typical range: 0.5-10.0)
pub fn gaussian_blur(img: &DynamicImage, sigma: f32) -> DynamicImage {
    // Use image crate's built-in blur
    img.blur(sigma)
}

/// Sharpen image using simple kernel
pub fn sharpen(img: &DynamicImage) -> DynamicImage {
    let kernel = [
        [ 0.0, -1.0,  0.0],
        [-1.0,  5.0, -1.0],
        [ 0.0, -1.0,  0.0],
    ];
    apply_kernel(img, &kernel)
}

/// Detect edges using Sobel operator
/// Returns grayscale image with edges highlighted
pub fn edge_detection(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    
    use imageproc::gradients::sobel_gradients;
    let edges: ImageBuffer<Luma<u16>, Vec<u16>> = sobel_gradients(&gray_img);
    
    // Convert u16 to u8 for display
    let (width, height) = edges.dimensions();
    let mut output = ImageBuffer::<Luma<u8>, Vec<u8>>::new(width, height);
    
    for (x, y, pixel) in edges.enumerate_pixels() {
        let normalized = (pixel[0] / 256) as u8;  // Scale down from u16 to u8
        output.put_pixel(x, y, Luma([normalized]));
    }
    
    DynamicImage::ImageLuma8(output)
}

/// Apply custom convolution kernel
/// 
/// # Arguments
/// * `img` - Input image
/// * `kernel` - 3x3 convolution kernel
pub fn apply_kernel(img: &DynamicImage, kernel: &[[f32; 3]; 3]) -> DynamicImage {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    let mut output = RgbImage::new(width, height);
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut r_sum = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut b_sum = 0.0f32;
            
            for ky in 0..3 {
                for kx in 0..3 {
                    let px = rgb_img.get_pixel(x + kx - 1, y + ky - 1);
                    let k = kernel[ky as usize][kx as usize];
                    
                    r_sum += px[0] as f32 * k;
                    g_sum += px[1] as f32 * k;
                    b_sum += px[2] as f32 * k;
                }
            }
            
            let r = r_sum.min(255.0).max(0.0) as u8;
            let g = g_sum.min(255.0).max(0.0) as u8;
            let b = b_sum.min(255.0).max(0.0) as u8;
            
            output.put_pixel(x, y, Rgb([r, g, b]));
        }
    }
    
    DynamicImage::ImageRgb8(output)
}

/// Emboss filter
pub fn emboss(img: &DynamicImage) -> DynamicImage {
    let kernel = [
        [-2.0, -1.0, 0.0],
        [-1.0,  1.0, 1.0],
        [ 0.0,  1.0, 2.0],
    ];
    apply_kernel(img, &kernel)
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
    fn test_gaussian_blur() {
        let img = create_test_image();
        let blurred = gaussian_blur(&img, 2.0);
        assert_eq!(blurred.dimensions(), img.dimensions());
    }

    #[test]
    fn test_sharpen() {
        let img = create_test_image();
        let sharpened = sharpen(&img);
        assert_eq!(sharpened.dimensions(), img.dimensions());
    }

    #[test]
    fn test_edge_detection() {
        let img = create_test_image();
        let edges = edge_detection(&img);
        assert_eq!(edges.dimensions(), img.dimensions());
    }

    #[test]
    fn test_emboss() {
        let img = create_test_image();
        let embossed = emboss(&img);
        assert_eq!(embossed.dimensions(), img.dimensions());
    }
}
