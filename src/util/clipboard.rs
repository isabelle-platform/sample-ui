use wasm_bindgen_futures::spawn_local;

pub fn copy_to_clipboard(text: String) {
    let _task = spawn_local(async move {
        let window = web_sys::window().expect("window");
        let nav = window.navigator().clipboard();
        match nav {
            Some(a) => {
                let p = a.write_text(&text);
                let _result = wasm_bindgen_futures::JsFuture::from(p)
                    .await
                    .expect("clipboard populated");
            }
            None => {}
        };
    });
}
