slint::include_modules!();

use std::fs::File;

use rfd::FileDialog;

#[tokio::main]
async fn main() {
    let app = RetroTetanus::new().unwrap();

    let weak_app = app.as_weak();
    app.global::<Sorting>().on_get_source_path(move || {
        let strong_app = weak_app.unwrap();
        strong_app
            .global::<Sorting>()
            .set_source_path("C:\\Some\\Source\\Path".into());
    });

    let weak_app = app.as_weak();
    app.global::<Sorting>().on_get_dest_path(move || {
        let weak_app = weak_app.clone();
        std::thread::spawn(move || {
            let file = FileDialog::new().pick_folder();
            if file.is_none() {
                return;
            }

            let file_path = file.unwrap().to_str().unwrap().to_owned();
            let app_handle = weak_app.clone();

            slint::invoke_from_event_loop(move || {
                let strong_app = app_handle.unwrap();
                strong_app
                    .global::<Sorting>()
                    .set_dest_path(file_path.into());
            })
            .expect("Error invoking main loop update");
        });
    });

    app.run().unwrap()
}
