/* Алгоритм вращающейся решётки */

use rand::{self, Rng};
use std::fmt::Debug;

pub type CardboardMatrix = [[bool; 4]; 4];
pub type CharMatrix = [[char; 4]; 4];

pub struct Grille {
    matrix: CardboardMatrix,
}

/* Rotate char matrix */
fn matrix_transpose<T: Copy + Debug>(m: [[T; 4]; 4]) -> [[T; 4]; 4] {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    let v: Vec<[T; 4]> = t.into_iter().map(|x| x.try_into().unwrap()).collect();
    v.try_into().unwrap()
}

pub fn rot_90<T: Copy + Debug>(m: [[T; 4]; 4]) -> [[T; 4]; 4] {
    let mut m = matrix_transpose(m);
    m.iter_mut().map(|row| row.reverse()).count();
    m
}

impl Grille {
    pub fn new(matrix: CardboardMatrix) -> Self {
        Grille { matrix }
    }

    pub fn encrypt(&self, text: &str) -> CharMatrix {
        let mut char_matrix: CharMatrix = [['-'; 4]; 4];
        let mut cardboard_cutout = self.matrix;
        let mut text_iter = text.chars().filter_map(|c| {
            if c.is_ascii_alphabetic() {
                Some(c.to_ascii_uppercase())
            } else {
                None
            }
        });

        for _rotation in 0..4 {
            for i in 0..4 {
                for j in 0..4 {
                    if cardboard_cutout[i][j] {
                        let char = text_iter
                            .next()
                            .unwrap_or(rand::thread_rng().gen_range('A'..='Z'));
                        char_matrix[i][j] = char;
                    }
                }
            }
            cardboard_cutout = rot_90(cardboard_cutout);
        }

        char_matrix
    }

    pub fn decrypt(&self, text: CharMatrix) -> String {
        let mut cardboard_cutout = self.matrix;
        let mut result = String::new();

        for _rotation in 0..4 {
            for i in 0..4 {
                for j in 0..4 {
                    if cardboard_cutout[i][j] {
                        result.push(text[i][j]);
                    }
                }
            }
            cardboard_cutout = rot_90(cardboard_cutout);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot90() {
        let matrix_s = [
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ];

        let res = rot_90(matrix_s);

        let matrix_e = [
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ];
        assert_eq!(res, matrix_e);
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn enc_dec_works(s in "\\PC*") {
            let matrix_s = [
                [true, false, false, false],
                [false, true, false, false],
                [false, false, false, true],
                [false, false, true, false],
            ];
            let grille = Grille::new(matrix_s);
            let enc = grille.encrypt(&s);
            let dec = grille.decrypt(enc);
            let enc2 = grille.encrypt(&dec);
            let dec2 = grille.decrypt(enc2);
            prop_assert_eq!(dec, dec2);
        }


        #[test]
        fn enc_dec_works_100(s in "[A-Z]{16}") {
            let matrix_s = [
                [true, false, false, false],
                [false, true, false, false],
                [false, false, false, true],
                [false, false, true, false],
            ];

            let grille = Grille::new(matrix_s);
            let enc = grille.encrypt(&s);
            let enc2 = grille.encrypt(&s);
            prop_assert_eq!(enc, enc2);

            let dec = grille.decrypt(enc);
            let dec2 = grille.decrypt(enc2);

            //prop_assert_eq!(dec.clone(), dec2);
            prop_assert_eq!(s, dec);
        }

        fn enc_dec_works_any_len(s in "([A-Z]{16})+") {
            let matrix_s = [
                [true, false, false, false],
                [false, true, false, false],
                [false, false, false, true],
                [false, false, true, false],
            ];

            let grille = Grille::new(matrix_s);
            let mut chars = s.chars();

            let mut output = String::new();
            'outer: loop {
                let mut char_matrix = [[' '; 4]; 4];
                for row in &mut char_matrix {
                    for ch in row {
                        if let Some(char) = chars.next() {
                            *ch = char;
                        } else {
                            break 'outer;
                        }
                    }
                }
                output.push_str(&grille.decrypt(char_matrix));
            }
            prop_assert_eq!(s, output);
        }
    }
}
