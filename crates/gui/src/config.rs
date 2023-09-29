use lazy_static;

lazy_static! {
    pub static ref CONFIG: Config<'static> = Config {
        // machine screen colors
        machine_screen_front: "#70a4b2",
        machine_screen_back: "#352879",

        // cpu state colors
        cpu_front: "#333333",
        cpu_back: "#333333",
        cpu_label: "#333333",
    };
}

pub struct Config<'a> {
    pub machine_screen_back: &'a str,
    pub machine_screen_front: &'a str,

    pub cpu_front: &'a str,
    pub cpu_back: &'a str,
    pub cpu_label: &'a str,
}
