pub struct VigenerProgressive {
    key: Vec<u32>,
}

pub fn filter_russian<I: Iterator<Item = char>>(inp: I) -> impl Iterator<Item = char> {
    inp.filter_map(|c| {
        let c = c.to_uppercase().next().unwrap();
        if ('А'..='Я').contains(&c) || ('Ё' == c) {
            Some(c)
        } else {
            None
        }
    })
}

const ALPHABET: [char; 33] = [
    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С',
    'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
];

fn rot(val: char, num: u32) -> char {
    from_ru_u32((as_ru_u32(val) + num) % ALPHABET.len() as u32)
}

fn as_ru_u32(val: char) -> u32 {
    if ('А'..='Е').contains(&val) {
        val as u32 - 'А' as u32
    } else if val == 'Ё' {
        as_ru_u32('Е') + 1
    } else if ('Ж'..='Я').contains(&val) {
        val as u32 - 'А' as u32 + 1
    } else {
        unreachable!()
    }
}

fn from_ru_u32(value: u32) -> char {
    ALPHABET[value as usize]
}

impl VigenerProgressive {
    pub fn new(key: &str) -> Option<Self> {
        let key: Vec<_> = filter_russian(key.chars()).map(as_ru_u32).collect();

        if key.is_empty() {
            return None;
        }

        Some(VigenerProgressive { key })
    }

    pub fn encrypt(&self, text: &str) -> String {
        let input = filter_russian(text.chars());

        let mut text = String::new();
        let mut cycle_num: u32 = 0;
        let mut key_iter = self.key.iter();

        for char in input {
            let num = match key_iter.next() {
                Some(num) => num,
                None => {
                    cycle_num += 1;
                    key_iter = self.key.iter();
                    key_iter.next().unwrap()
                }
            };
            let key_num = num + cycle_num;
            text.push(rot(char, key_num));
        }
        text
    }

    pub fn decrypt(&self, text: &str) -> String {
        let input = filter_russian(text.chars());

        let mut text = String::new();
        let mut cycle_num: u32 = 0;
        let mut key_iter = self.key.iter();

        for char in input {
            let key_num = match key_iter.next() {
                Some(num) => num,
                None => {
                    cycle_num += 1;
                    key_iter = self.key.iter();
                    key_iter.next().unwrap()
                }
            };
            text.push(rot(char, ALPHABET.len() as u32 - (key_num + cycle_num)));
        }
        text
    }
}
