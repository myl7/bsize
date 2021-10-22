use crate::grammar::lexer::{self, bsizeLexer};
use crate::scale::{BiScale, Scale};
use antlr_rust::token::Token;
use antlr_rust::token_stream::UnbufferedTokenStream;
use antlr_rust::InputStream;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Unit {
    Bit = 1,
    Byte = 2,
    None = 3,
}

#[derive(ToPrimitive)]
pub enum Error {
    InvalidInput = -1,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "bsize error code {}",
            self.to_isize().unwrap()
        ))
        .unwrap();
        Ok(())
    }
}

pub fn parse(s: &str, ignore_bi: bool, default_bi: bool) -> Result<(u64, Unit), Error> {
    let mut lexer = bsizeLexer::new(InputStream::new(s));
    let mut token_src = UnbufferedTokenStream::new_unbuffered(&mut lexer);
    let mut token_iter = token_src.token_iter();

    let mut num;
    match token_iter.next() {
        None => return Err(Error::InvalidInput),
        Some(t) => match t.get_text().parse::<u64>() {
            Err(_) => return Err(Error::InvalidInput),
            Ok(d) => num = d,
        },
    }

    let mut unit = Unit::None;
    let mut is_bi = false;
    let mut is_ok = false;
    let mut scale = (Scale::B, BiScale::B);
    loop {
        match token_iter.next() {
            None => break,
            Some(t) => {
                let i = t.get_token_type();
                match i {
                    lexer::Space | lexer::S => (),
                    lexer::Bit | lexer::BitBody => {
                        num *= choose_scale(scale, ignore_bi, default_bi, is_bi, false);
                        unit = Unit::Bit;
                        is_ok = true;
                    }
                    lexer::Byte | lexer::ByteBody => {
                        num *= choose_scale(scale, ignore_bi, default_bi, is_bi, false);
                        unit = Unit::Byte;
                        is_bi = true;
                    }
                    lexer::BiSign => is_bi = true,
                    lexer::Kilo | lexer::KiloWord | lexer::KibiWord => {
                        scale = (Scale::K, BiScale::K);
                        if i == lexer::KibiWord {
                            is_bi = true;
                        }
                    }
                    lexer::Mega | lexer::MegaWord | lexer::MebiWord => {
                        scale = (Scale::M, BiScale::M);
                        if i == lexer::MebiWord {
                            is_bi = true;
                        }
                    }
                    lexer::Giga | lexer::GigaWord | lexer::GibiWord => {
                        scale = (Scale::G, BiScale::G);
                        if i == lexer::GibiWord {
                            is_bi = true;
                        }
                    }
                    lexer::Tera | lexer::TeraWord | lexer::TebiWord => {
                        scale = (Scale::T, BiScale::T);
                        if i == lexer::TebiWord {
                            is_bi = true;
                        }
                    }
                    lexer::Peta | lexer::PetaWord | lexer::PebiWord => {
                        scale = (Scale::P, BiScale::P);
                        if i == lexer::PebiWord {
                            is_bi = true;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    if !is_ok {
        num *= choose_scale(scale, ignore_bi, default_bi, is_bi, true);
    }

    return Ok((num, unit));
}

fn choose_scale(
    scale: (Scale, BiScale),
    ignore_bi: bool,
    default_bi: bool,
    is_bi: bool,
    is_default: bool,
) -> u64 {
    if ignore_bi {
        return scale.0 as u64;
    }
    if (is_default && default_bi) || (!is_default && is_bi) {
        scale.1 as u64
    } else {
        scale.0 as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_cases() {
        assert_eq!(parse("10M", false, false).unwrap(), (10000000, Unit::None));
    }
}
