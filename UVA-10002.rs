// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=943

use std::io::{self, Read, Result, StdinLock};

struct WordsIterator<'a> {
  stdin: &'a mut StdinLock<'static>,
  has_read: bool,
  words: Vec<String>,
  index: usize,
}

impl<'a> WordsIterator<'a> {
  fn new(stdin: &'a mut StdinLock<'static>) -> WordsIterator<'a> {
    WordsIterator {
      stdin,
      has_read: false,
      words: Vec::new(),
      index: 0,
    }
  }
}

impl<'a> Iterator for WordsIterator<'a> {
  type Item = Result<String>;

  fn next(&mut self) -> Option<Result<String>> {
    if !self.has_read {
      let mut buf: Vec<u8> = Vec::new();
      
      if let Err(e) = self.stdin.read_to_end(&mut buf) {
        return Some(Err(e));
      };

      let something = match String::from_utf8(buf) {
        Ok(something) => something,
        Err(_) => return Some(Err(io::Error::new(io::ErrorKind::InvalidData, "invalid utf8 input"))),
      };

      self.words = something.split_whitespace().map(String::from).collect();
      self.has_read = true;
    }

    if self.index < self.words.len() {
      let result = &self.words[self.index];
      self.index += 1;
      return Some(Ok(result.clone()));
    }

    return None;
  }
}

fn main() {
  let mut stdin = io::stdin().lock();
  let mut words = WordsIterator::new(&mut stdin);

  while let Some(Ok(word)) = words.next() {
    println!("word is {word}");
  }

}
