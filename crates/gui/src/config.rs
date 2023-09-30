use lazy_static;

lazy_static! {
    pub static ref CONFIG: Config<'static> = Config {
        // machine screen colors
        machine_screen_front: "#70a4b2",
        machine_screen_back: "#352879",

        memory_view_size: 200

    };
}

pub struct Config<'a> {
    pub machine_screen_back: &'a str,
    pub machine_screen_front: &'a str,

    pub memory_view_size: u16
}
