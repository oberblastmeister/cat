use crate::Opt;

pub struct State {
    pub buffer: String,
    pub insert_buffer: Option<String>,
    pub suppress: bool,
    pub line: u32,
}

impl State {
    pub fn new(opt: &Opt) -> State {
        let insert_buffer = if opt.is_number_option() {
            Some(String::new())
        } else {
            None
        };
        State {
            buffer: String::new(),
            insert_buffer,
            suppress: false,
            line: 1,
        }
    }
}

