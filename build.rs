use std::io::Error;

fn main() -> Result<(), Error> {
    slint_build::compile("ui/Window.slint").unwrap();

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ui/icons/radio-solid.ico")
            .set("OriginalFilename", "radar.exe")
            .set("InternalName", "radar.exe");

        res.compile()?;
    }

    Ok(())
}