slint::include_modules!();

use rfd::AsyncFileDialog;

#[tokio::main]
async fn main() {
    let app = RetroTetanus::new().unwrap();

    let app_handle = app.as_weak();
    app.global::<Sorting>().on_get_source_path(move || {
        let app_handle = app_handle.clone();
        tokio::spawn(async move {
            let file = AsyncFileDialog::new()
                .set_title("Select Source")
                .pick_folder()
                .await;
            if file.is_none() {
                return;
            }

            let file_path = file.unwrap().path().to_str().unwrap().to_owned();
            slint::invoke_from_event_loop(move || {
                let strong_app = app_handle.unwrap();
                strong_app
                    .global::<Sorting>()
                    .set_source_path(file_path.into());
            })
            .expect("Error invoking main loop update");
        });
    });

    let app_handle = app.as_weak();
    app.global::<Sorting>().on_get_dest_path(move || {
        let app_handle = app_handle.clone();
        tokio::spawn(async move {
            let file = AsyncFileDialog::new()
                .set_title("Select Destination")
                .pick_folder()
                .await;
            if file.is_none() {
                return;
            }

            let file_path = file.unwrap().path().to_str().unwrap().to_owned();
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
