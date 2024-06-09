use wasm_cookies::{CookieOptions, SameSite};

fn get_current_domain() -> String {
    // Get the window object
    let window = web_sys::window().ok_or("No window object found").unwrap();

    // Get the location object from the window
    let location = window.location();

    // Get the hostname from the location object
    let hostname = location.hostname().unwrap();

    // Check if the hostname is not empty
    if !hostname.is_empty() {
        hostname
    } else {
        "".to_string()
    }
}

pub fn set_cookie(name: &str, value: &str) {
    let dom = get_current_domain();
    let cookie_options = CookieOptions {
        expires: None,
        path: Some("/"),
        domain: Some(&dom),
        secure: true,
        same_site: SameSite::None,
    };
    wasm_cookies::set(name, value, &cookie_options);
}

pub fn get_cookie(name: &str) -> String {
    let r = wasm_cookies::get(name);
    if r.is_some() {
        let v = r.unwrap();
        if v.is_ok() {
            return v.unwrap();
        }
    }
    return "".to_string();
}
