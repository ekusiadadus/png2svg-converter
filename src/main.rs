use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the PNG file
    #[arg(short, long)]
    file_path: String,

    /// Path to the SVG file
    #[arg(short, long)]
    output_path: String,

    /// Width of the SVG
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    /// Height of the SVG
    #[arg(short, long, default_value_t = 100)]
    height: u32,
}

fn read_png_file(file_path: &str) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(file_path).map_err(|e| e.to_string())?;
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().map_err(|e| e.to_string())?;

    // ここでバッファサイズを計算します。
    // PNGの色タイプに応じて、必要なバッファサイズを計算します。
    // ここでは、最も一般的なケース（例えば、RGBA）を想定しています。
    let info = &reader.info();
    let bytes_per_pixel = info.color_type.samples() * info.bit_depth as usize / 8;
    let buffer_size = info.width as usize * info.height as usize * bytes_per_pixel;

    let mut buf = vec![0; buffer_size];
    reader.next_frame(&mut buf).map_err(|e| e.to_string())?;

    Ok(buf)
}

fn convert_pixels_to_vectors(pixels: Vec<u8>, width: u32, height: u32) -> Vec<Vec<u8>> {
    let bytes_per_pixel = pixels.len() / (width * height) as usize;
    pixels
        .chunks_exact(width as usize * bytes_per_pixel)
        .map(|row| row.to_vec())
        .collect()
}

fn create_svg_from_vectors(vectors: Vec<Vec<u8>>, width: u32, height: u32) -> String {
    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
        width, height
    ));
    for (y, row) in vectors.iter().enumerate() {
        for (x, pixel) in row.chunks_exact(4).enumerate() {
            let color = format!("#{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2]);
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="1" height="1" fill="{}" />"#,
                x, y, color
            ));
        }
    }
    svg.push_str("</svg>");
    svg
}

fn main() {
    // let args = Args::parse();

    let input_path = "./input/shapieron.png";
    let output_path = "./output/shapieron.svg";
    let width = 100;
    let height = 100;

    // let pixels = read_png_file(&args.file_path).unwrap();
    // let vectors = convert_pixels_to_vectors(pixels, args.width, args.height);
    // let svg = create_svg_from_vectors(vectors, args.width, args.height);

    let pixels = read_png_file(input_path).unwrap();
    let vectors = convert_pixels_to_vectors(pixels, width, height);
    let svg = create_svg_from_vectors(vectors, width, height);

    // std::fs::write(&args.output_path, svg).unwrap();

    std::fs::write(output_path, svg).unwrap();
}
