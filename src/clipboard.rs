use jabba_lib::jclipboard;

pub fn set_text(text: &str) {
    jclipboard::check();  // may panic

    jclipboard::set_text(text).unwrap();
}

#[allow(dead_code)]
pub fn get_text() -> String {
    jclipboard::get_text().unwrap()
}

#[allow(dead_code)]
pub fn clear() {
    set_text("");
}

// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_text_and_get_text_test() {
        let backup = get_text();
        //
        let text = "";
        set_text(text);
        assert_eq!(get_text(), text);
        //
        let text = "a";
        set_text(text);
        assert_eq!(get_text(), text);
        //
        let text = "hello";
        set_text(text);
        assert_eq!(get_text(), text);
        //
        set_text(&backup);
        assert_eq!(get_text(), backup);
    }

    #[test]
    fn clear_test() {
        let backup = get_text();
        //
        clear();
        assert_eq!(get_text(), "");
        //
        set_text(&backup);
        assert_eq!(get_text(), backup);
    }
}
