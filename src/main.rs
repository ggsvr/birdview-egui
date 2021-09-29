fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = birdview::BirdView::new()?;
    eframe::run_native(Box::new(app), eframe::NativeOptions::default());

    Ok(())
}