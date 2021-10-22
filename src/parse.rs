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

/// Parse human-readable byte size str to the number and unit
///
/// # Arguments
///
/// * `s`: Parsed str
/// * `ignore_bi`: If enable, ignore all bi sign and always use decimal, otherwise, recognize bi sign
/// * `default_bi`: If enable, use binary when no bi sign, otherwise, use decimal when no bi sign.
/// When there is a bi sign, the behavior is controlled by `ignore_bi`.
///
/// returns: Result<(u64, Unit), Error>
///
/// # Examples
///
/// ```
/// parse("10MB", false, false)
/// ```
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
                        num *= choose_scale(scale, ignore_bi, default_bi, is_bi);
                        unit = Unit::Bit;
                        is_ok = true;
                    }
                    lexer::Byte | lexer::ByteBody => {
                        num *= choose_scale(scale, ignore_bi, default_bi, is_bi);
                        unit = Unit::Byte;
                        is_ok = true;
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
        num *= choose_scale(scale, ignore_bi, default_bi, is_bi);
    }

    return Ok((num, unit));
}

fn choose_scale(scale: (Scale, BiScale), ignore_bi: bool, default_bi: bool, is_bi: bool) -> u64 {
    if ignore_bi {
        return scale.0 as u64;
    }
    if is_bi || default_bi {
        scale.1 as u64
    } else {
        scale.0 as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abbr() {
        assert_eq!(parse("10", false, false).unwrap(), (10, Unit::None));
        assert_eq!(parse("10K", false, false).unwrap(), (10000, Unit::None));
        assert_eq!(parse("10KB", false, false).unwrap(), (10000, Unit::Byte));
        assert_eq!(parse("10Kb", false, false).unwrap(), (10000, Unit::Bit));
        assert_eq!(parse("10KiB", false, false).unwrap(), (10240, Unit::Byte));

        assert_eq!(parse("10", false, true).unwrap(), (10, Unit::None));
        assert_eq!(parse("10K", false, true).unwrap(), (10240, Unit::None));
        assert_eq!(parse("10KB", false, true).unwrap(), (10240, Unit::Byte));
        assert_eq!(parse("10Kb", false, true).unwrap(), (10240, Unit::Bit));
        assert_eq!(parse("10KiB", false, true).unwrap(), (10240, Unit::Byte));

        assert_eq!(parse("10", true, true).unwrap(), (10, Unit::None));
        assert_eq!(parse("10K", true, true).unwrap(), (10000, Unit::None));
        assert_eq!(parse("10KB", true, true).unwrap(), (10000, Unit::Byte));
        assert_eq!(parse("10Kb", true, true).unwrap(), (10000, Unit::Bit));
        assert_eq!(parse("10KiB", true, true).unwrap(), (10000, Unit::Byte));
    }
}
