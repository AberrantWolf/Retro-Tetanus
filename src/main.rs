slint::include_modules!();

fn main() {
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
        let strong_app = weak_app.unwrap();
        strong_app
            .global::<Sorting>()
            .set_dest_path("C:\\Some\\Dest\\Path".into());
    });

    app.run().unwrap()
}
