use crate::word::word::{query_word, Word};

pub enum InputMod {
    Normal,
    Insert,
}

pub struct App {
    pub word: Word,
    pub input: String,
    pub word_name: String,
    pub cursor_pos: usize,
    pub input_mod: InputMod,
}
impl App {
    pub fn new() -> Self {
        Self {
            word: Word::default(), input: String::new(),
            word_name: String::new(),
            cursor_pos: 0,
            input_mod: InputMod::Normal,
        }
    }

    pub fn search(&mut self) {
        self.word = query_word(&self.word_name).expect("查询单词失败");
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved = self.cursor_pos.saturating_sub(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved);
    }
    pub fn move_cursor_right(&mut self) {
        let cursor_moved = self.cursor_pos.saturating_add(1);
        self.cursor_pos = self.clamp_cursor(cursor_moved);
    }
    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_pos, new_char);
        self.move_cursor_right();
    }
    pub fn delete_char(&mut self) {
        if self.cursor_pos != 0 {
            let current_idx = self.cursor_pos;
            let from_left_to_current_idx = current_idx - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_idx);

            let after_char_to_delete = self.input.chars().skip(current_idx);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }
    pub fn reset_cursor(&mut self) {
        self.cursor_pos = 0;
    }
    pub fn submit_query(&mut self) {
        self.word_name = self.input.clone().trim().to_string();
        self.input.clear();
        self.reset_cursor();
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_search() {
        let mut app = App::new();
        app.search();
    }
    #[test]
    fn test_submit_query(){
        let mut app=App::new();
        let input_str="hello".to_string();
        app.input=input_str.clone();
        app.submit_query();
        assert_eq!(app.word_name,input_str);
    }
}
