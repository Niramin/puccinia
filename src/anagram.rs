use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

   let mut ans_set = HashSet:: new();

   for other_word in possible_anagrams {
       let other_word_chars = other_word.to_lowercase();
       
       let this_word = word.to_lowercase();
      
      if this_word != other_word_chars
      {
        let mut other_normalized: Vec<char>  = other_word_chars.chars().collect();
        other_normalized.sort();
        let mut this_normalized: Vec<char>  =  this_word.chars().collect();
        this_normalized.sort();
        if(this_normalized == other_normalized)
        {
            ans_set.insert(*other_word);
        }
      }
   }

  ans_set


}


