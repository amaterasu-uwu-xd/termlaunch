use resvg::{render, usvg::{Options, Transform, Tree}, tiny_skia::Pixmap};
use image::{DynamicImage, ImageBuffer, ImageReader};
use std::{fs, path::PathBuf};

pub fn get_image(path: PathBuf) -> Result<DynamicImage, String> {
    
    // Check if the file exists
    if path.extension().unwrap_or_default() != "svg" {
        let dyn_img = ImageReader::open(path.clone()).unwrap().decode().unwrap().resize_to_fill(254, 254, ratatui_image::FilterType::Triangle);
        Ok(dyn_img)
    }
    else {
        // Read the SVG file and render it to a pixmap
        let svg_data = fs::read(path.clone()).map_err(|e| e.to_string())?;
        let opt = Options::default();
        let tree = Tree::from_data(&svg_data, &opt).map_err(|e| e.to_string())?;
        let mut pixmap = Pixmap::new(254, 254).ok_or("Failed to create pixmap")?;
        let mut pixmap_mut = pixmap.as_mut();

        let target_size = 254;
        let origiinal_size = tree.size();
        let scale = target_size as f32 / origiinal_size.width().max(origiinal_size.height());
        let transform = Transform::from_scale(scale, scale);

        render(&tree, transform, &mut pixmap_mut);

        // Convert the pixmap to a DynamicImage
        let image = ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
            .ok_or("Failed to create image buffer")?;
        let dynamic_image = DynamicImage::ImageRgba8(image);

        Ok(dynamic_image)
    }

}