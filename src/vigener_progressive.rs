pub struct VigenerProgressive {
    key: Vec<u32>,
}

fn filter_russian<I: Iterator<Item = char>>(inp: I) -> impl Iterator<Item = char> {
    inp.filter_map(|c| {
        let c = c.to_uppercase().next().unwrap();
        if ('А'..='Я').contains(&c) || ('Ё' == c) {
            Some(c)
        } else {
            None
        }
    })
}

trait Rot {
    fn rot(self, num: u32) -> Self;
    fn as_ru_u32(&self) -> u32;
    fn from_ru_u32(value: u32) -> Self;
}

const ALPHABET: [char; 33] = [
    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С',
    'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю', 'Я',
];

impl Rot for char {
    fn rot(self, num: u32) -> Self {
        Self::from_ru_u32((self.as_ru_u32() + num) % ALPHABET.len() as u32)
    }

    fn as_ru_u32(&self) -> u32 {
        if ('А'..='Е').contains(self) {
            *self as u32 - 'А' as u32
        } else if *self == 'Ё' {
            dbg!('Е'.as_ru_u32() + 1)
        } else if ('Ж'..='Я').contains(self) {
            *self as u32 - 'А' as u32 + 1
        } else {
            unreachable!()
        }
    }

    fn from_ru_u32(value: u32) -> Self {
        ALPHABET[value as usize]
    }
}

impl VigenerProgressive {
    pub fn new(key: &str) -> Option<Self> {
        let key: Vec<_> = filter_russian(key.chars()).map(|c| c.as_ru_u32()).collect();

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
            text.push(char.rot(dbg!(key_num)));
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
                Some(num) => num + cycle_num,
                None => {
                    cycle_num += 1;
                    key_iter = self.key.iter();
                    *key_iter.next().unwrap()
                }
            };
            text.push(char.rot(ALPHABET.len() as u32 - key_num));
        }
        text
    }
}
