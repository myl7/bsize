use crate::grammar::lexer::{self, bsizeLexer};
use crate::scale::{BiScale, Scale};
use antlr_rust::token::Token;
use antlr_rust::token_stream::UnbufferedTokenStream;
use antlr_rust::InputStream;

pub enum Unit {
    Bit = 1,
    Byte = 2,
    None = 3,
}

pub enum Error {
    InvalidInput = -1,
}

pub fn parse(s: &str, ignore_bi: bool) -> Result<(u64, Unit), Error> {
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
    let mut f = |is_bi: bool, scale: (Scale, BiScale)| {
        if is_ok {
            return;
        }
        if !is_bi | ignore_bi {
            num *= scale.0 as u64
        } else {
            num *= scale.1 as u64
        }
        is_ok = true;
    };
    loop {
        match token_iter.next() {
            None => break,
            Some(t) => {
                let i = t.get_token_index();
                match i {
                    lexer::Space | lexer::S => (),
                    lexer::Bit | lexer::BitBody => {
                        f(is_bi, scale);
                        unit = Unit::Bit
                    }
                    lexer::Byte | lexer::ByteBody => {
                        f(is_bi, scale);
                        unit = Unit::Byte
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
    f(is_bi, scale);

    return Ok((num, unit));
}
