// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::char::from_u32;

use crate::BufferScanner::{BufferScanner, EOFError};
use crate::ErrScanner::{Error, FormatError};
use crate::Position::Position;
use crate::PosRange::PosRange;
use crate::string_vec;
use crate::Token::{IntFormat, Token, TokenKind};

pub struct Scanner {
    pub BufferScanner: BufferScanner,

    pub Delimiters: Vec<char>,
    pub Whitespaces: Vec<char>,
}

macro_rules! from_to {
    ($begin: expr, $vec: expr) => {
		$vec.BufferScanner.Buffer[$begin.Offset..$vec.GetPos().Offset].to_owned()
	};
}

impl Scanner {
    pub fn GetChar(&self) -> Result<char, Error> {
        match self.BufferScanner.GetChar() {
            Ok(ch) => { Ok(ch) }
            Err(_) => { Err(Error::EOF(EOFError { Pos: self.GetPos() })) }
        }
    }

    pub fn Move(&mut self) -> Result<char, Error> {
        match self.BufferScanner.Move() {
            Ok(ch) => { Ok(ch) }
            Err(_) => { Err(Error::EOF(EOFError { Pos: self.GetPos() })) }
        }
    }

    pub fn GotoNextLine(&mut self) -> Result<(), Error> {
        match self.BufferScanner.GotoNextLine() {
            Ok(_) => { Ok(()) }
            Err(err) => { Err(Error::EOF(err)) }
        }
    }

    pub fn GetPos(&self) -> Position { self.BufferScanner.Pos.clone() }

    pub fn SkipWhitespaces(&mut self) -> Result<(), Error> {
        while self.Whitespaces.contains(&self.GetChar()?) {
            self.Move()?;
        }

        Ok(())
    }

    pub fn ScanLineComment(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        self.GotoNextLine()?;

        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::COMMENT,
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanQuotedComment(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            if self.Move()? == '*' {
                if self.Move()? == '/' {
                    break;
                }
            }
        }


        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::COMMENT,
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanComment(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        return match self.Move()? {
            '/' => { self.ScanLineComment() }
            '*' => { self.ScanQuotedComment() }
            _ => {
                return Err(Error::Format(FormatError {
                    PosRange: PosRange { Begin: begin, End: self.GetPos() },
                }));
            }
        };
    }

    pub fn ScanIdent(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if ch.is_ascii_alphabetic() || ch.is_numeric() || ch == '_' {
                self.Move()?;
            } else {
                break;
            }
        }

        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::IDENT,
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanHex(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if '0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'f' {
                self.Move()?;
            } else {
                break;
            }
        }

        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::INT(IntFormat::HEX),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanDec(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if '0' <= ch && ch <= '9' {
                self.Move()?;
            } else {
                break;
            }
        }


        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::INT(IntFormat::DEC),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanOct(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if '0' <= ch && ch <= '7' {
                self.Move()?;
            } else {
                break;
            }
        }


        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::INT(IntFormat::OCT),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanBin(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if ch == '0' || ch == '1' {
                self.Move()?;
            } else {
                break;
            }
        }


        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::INT(IntFormat::BIN),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanDigit(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        match self.Move()? {
            '0' => {
                match self.Move()? {
                    'x' => { self.ScanHex() }
                    'o' => { self.ScanOct() }
                    'b' => { self.ScanBin() }
                    _ => {
                        return Err(Error::Format(FormatError {
                            PosRange: PosRange {
                                Begin: begin,
                                End: self.GetPos(),
                            },
                        }));
                    }
                }
            }
            _ => {
                self.BufferScanner.Pos = begin.clone();
                self.ScanDec()
            }
        }
    }

    pub fn ScanUnicodeHex(&mut self, runesN: u8) -> Result<char, Error> {
        let begin = self.GetPos();

        let mut seq: Vec<char> = vec![];
        for _ in 0..runesN {
            seq.push(self.Move()?);
        }

        let ch = match u32::from_str_radix(&string_vec!(seq), 16) {
            Ok(ch) => {
                match from_u32(ch) {
                    None => {
                        return Err(Error::Format(FormatError {
                            PosRange: PosRange { Begin: begin, End: self.GetPos() }
                        }));
                    }
                    Some(ch) => { ch }
                }
            }
            Err(err) => {
                println!("{}", err.to_string());
                return Err(Error::Format(FormatError {
                    PosRange: PosRange { Begin: begin, End: self.GetPos() },
                }));
            }
        };

        Ok(ch)
    }

    pub fn ScanEscapeChar(&mut self, quote: char) -> Result<char, Error> {
        let begin = self.GetPos();

        let ch = self.Move()?;

        Ok(match ch {
            'n' => { '\n' }
            't' => { '\t' }
            'r' => { '\r' }
            '\\' => { '\\' }
            'x' => { // 1 byte
                self.ScanUnicodeHex(2)?
            }
            'u' => { // 2 byte
                self.ScanUnicodeHex(4)?
            }
            'U' => { // 4 byte
                self.ScanUnicodeHex(8)?
            }
            _ if ch == quote => { quote }
            _ => {
                return Err(Error::Format(FormatError { PosRange: PosRange { Begin: begin, End: self.GetPos() } }));
            }
        })
    }

    pub fn ScanString(&mut self, quote: char) -> Result<Token, Error> {
        let begin = self.GetPos();

        self.Move()?; // skip quote

        let mut seq: Vec<char> = vec![];

        loop {
            let ch = self.Move()?;
            match ch {
                '\\' => {
                    let esc = self.ScanEscapeChar(quote)?;
                    seq.push(esc)
                }
                _ if ch == quote => {
                    break;
                }
                _ => { seq.push(ch) }
            }
        }

        Ok(Token {
            Pos: begin,
            Kind: TokenKind::STRING,
            Literal: seq,
        })
    }

    pub fn ScanOperator(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if !ch.is_ascii_punctuation() || self.Delimiters.contains(&ch) {
                break;
            }
            self.Move()?;
        }


        Ok(Token {
            Pos: begin.clone(),
            Kind: TokenKind::OPERATOR,
            Literal: from_to!(begin, self),
        })
    }

    pub fn Scan(&mut self) -> Result<Token, Error> {
        let begin = self.GetPos();

        self.SkipWhitespaces()?;

        let ch = self.GetChar()?;


        if ch.is_alphabetic() {
            self.ScanIdent()
        } else if ch.is_numeric() {
            return Ok(self.ScanDigit()?);
        } else if ch.is_ascii_punctuation() {
            if self.Delimiters.contains(&ch) {
                Ok(Token{
                    Pos: begin,
                    Kind: TokenKind::DELIMITER,
                    Literal: vec![self.Move()?],
                })
            } else {
                match ch {
                    '"' => { self.ScanString('"') }
                    _ => { self.ScanOperator() }
                }
            }
        } else {
            match ch {
                '/' => { self.ScanComment() }
                _ => { return Err(Error::EOF(EOFError { Pos: self.GetPos() })); }
                // TODO
            }
            // TODO
        }
    }
}
