use std::io::Error;

fn main() -> Result<(), Error> {
    slint_build::compile("ui/Window.slint").unwrap();

    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ui/icons/radio-solid.ico")
            .set("OriginalFilename", "radio.exe")
            .set("InternalName", "radio.exe");

        res.compile()?;
    }

    Ok(())
}