use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
   pub static ref ENCODE_MAP:HashMap<char, &'static str>={
    let mut m = HashMap::new();
        m.insert('A', ".-");
        m.insert('B', "-...");
        m.insert('C', "-.-.");
        m.insert('D', "-..");
        m.insert('E', ".");
        m.insert('F', "..-.");
        m.insert('G', "--.");
        m.insert('H', "....");
        m.insert('I', "..");
        m.insert('J', ".---");
        m.insert('K', "-.-");
        m.insert('L', ".-..");
        m.insert('M', "--");
        m.insert('N', "-.");
        m.insert('O', "---");
        m.insert('P', ".--.");
        m.insert('Q', "--.-");
        m.insert('R', ".-.");
        m.insert('S', "...");
        m.insert('T', "-");
        m.insert('U', "..-");
        m.insert('V', "...-");
        m.insert('W', ".--");
        m.insert('X', "-..-");
        m.insert('Y', "-.--");
        m.insert('Z', "--..");

        // numbers
        m.insert('0', "-----");
        m.insert('1', ".----");
        m.insert('2', "..---");
        m.insert('3', "...--");
        m.insert('4', "....-");
        m.insert('5', ".....");
        m.insert('6', "-....");
        m.insert('7', "--...");
        m.insert('8', "---..");
        m.insert('9', "----.");
    m
   };

   pub static ref DECODE_MAP:HashMap<&'static str, char>={
    let mut m = HashMap::new();
    for (k,v) in ENCODE_MAP.iter(){
        m.insert(*v, *k);
    }
    m
   };
}
