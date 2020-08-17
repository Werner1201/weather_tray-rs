use webbrowser;

pub fn open_browser() {
    if webbrowser::open("https://home.openweathermap.org/users/sign_up").is_ok() {}
}
