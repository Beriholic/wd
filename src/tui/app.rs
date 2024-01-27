use crate::word::word::{mock_word, Word};
#[derive(Debug, Default)]
pub struct App{
    pub is_quit:bool,
    pub word: Word,
}
impl App{
    pub fn new()->Self{
        Self::default()
    }
    
    pub fn quit(&mut self){
        self.is_quit=true;
    }
    pub fn search(&mut self){
        self.word=mock_word().expect("get word is failed");
    }
}


#[cfg(test)]
mod test{

    use super::*;
    #[test]
    fn test_app_quit(){
        let mut app=App::new();
        app.quit();
        assert_eq!(app.is_quit,true);
    }
    #[test]
    fn test_search(){
        let mut app=App::new();
        app.search();
    }
}
