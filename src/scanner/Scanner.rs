// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::char::from_u32;

use crate::scanner::BasicToken::{BasicToken, BasicTokenKind, IntFormat};
use crate::scanner::BufferScanner::{BufferScanner, EOFError};
use crate::scanner::Position::Position;
use crate::scanner::PosRange::PosRange;
use crate::string_vec;

pub enum ScannerError {
    EOFError(EOFError),
    FormatError(FormatError),
}

impl ScannerError {
    pub fn Error(self) -> String {
        match self {
            ScannerError::EOFError(err) => { err.Error() }
            ScannerError::FormatError(err) => { err.Error() }
        }
    }
}

pub struct FormatError {
    pub PosRange: PosRange,
}

impl FormatError {
    pub fn Error(&self) -> String { format!("{}: format error", self.PosRange.to_string()) }
}

pub struct Scanner {
    pub BufferScanner: BufferScanner,

    pub Delimiters: Vec<char>,
    pub Whitespaces: Vec<char>,
}

pub fn NewBufferScanner(buffer: Vec<char>) -> BufferScanner {
    BufferScanner {
        Pos: Position {
            Offset: 0,
            Line: 0,
            Column: 0,
        },
        Buffer: buffer,
    }
}

macro_rules! from_to {
    ($begin: expr, $vec: expr) => {
		$vec.BufferScanner.Buffer[$begin.Offset..$vec.GetPos().Offset].to_owned()
	};
}

impl Scanner {
    pub fn GetChar(&self) -> Result<char, ScannerError> {
        match self.BufferScanner.GetChar() {
            Ok(ch) => { Ok(ch) }
            Err(_) => { Err(ScannerError::EOFError(EOFError { Pos: self.GetPos() })) }
        }
    }

    pub fn Move(&mut self) -> Result<char, ScannerError> {
        match self.BufferScanner.Move() {
            Ok(ch) => { Ok(ch) }
            Err(_) => { Err(ScannerError::EOFError(EOFError { Pos: self.GetPos() })) }
        }
    }

    pub fn GotoNextLine(&mut self) -> Result<(), ScannerError> {
        match self.BufferScanner.GotoNextLine() {
            Ok(_) => { Ok(()) }
            Err(err) => { Err(ScannerError::EOFError(err)) }
        }
    }

    pub fn GetPos(&self) -> Position { self.BufferScanner.Pos.clone() }

    pub fn SkipWhitespaces(&mut self) -> Result<(), ScannerError> {
        while self.Whitespaces.contains(&self.GetChar()?) {
            self.Move()?;
        }

        Ok(())
    }

    pub fn ScanLineComment(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        self.GotoNextLine()?;

        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Comment,
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanQuotedComment(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            if self.Move()? == '*' {
                if self.Move()? == '/' {
                    break;
                }
            }
        }


        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Comment,
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanComment(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        return match self.Move()? {
            '/' => { self.ScanLineComment() }
            '*' => { self.ScanQuotedComment() }
            _ => {
                return Err(ScannerError::FormatError(FormatError {
                    PosRange: PosRange { Begin: begin, End: self.GetPos() },
                }));
            }
        };
    }

    pub fn ScanIdent(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if ch.is_ascii_alphabetic() || ch.is_numeric() || ch == '_' {
                self.Move()?;
            } else {
                break;
            }
        }

        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Ident,
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanHex(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if '0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'f' {
                self.Move()?;
            } else {
                break;
            }
        }

        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Int(IntFormat::HEX),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanDec(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if '0' <= ch && ch <= '9' {
                self.Move()?;
            } else {
                break;
            }
        }


        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Int(IntFormat::DEC),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanOct(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if '0' <= ch && ch <= '7' {
                self.Move()?;
            } else {
                break;
            }
        }


        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Int(IntFormat::OCT),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanBin(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            let ch = self.GetChar()?;
            if ch == '0' || ch == '1' {
                self.Move()?;
            } else {
                break;
            }
        }


        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Int(IntFormat::BIN),
            Literal: from_to!(begin, self),
        })
    }

    pub fn ScanDigit(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        match self.Move()? {
            '0' => {
                match self.Move()? {
                    'x' => { self.ScanHex() }
                    'o' => { self.ScanOct() }
                    'b' => { self.ScanBin() }
                    _ => {
                        return Err(ScannerError::FormatError(FormatError {
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

    pub fn ScanUnicodeHex(&mut self, runesN: u8) -> Result<char, ScannerError> {
        let begin = self.GetPos();

        let mut seq: Vec<char> = vec![];
        for _ in 0..runesN {
            seq.push(self.Move()?);
        }

        let ch = match u32::from_str_radix(&string_vec!(seq), 16) {
            Ok(ch) => {
                match from_u32(ch) {
                    None => {
                        return Err(ScannerError::FormatError(FormatError {
                            PosRange: PosRange { Begin: begin, End: self.GetPos() }
                        }));
                    }
                    Some(ch) => { ch }
                }
            }
            Err(err) => {
                println!("{}", err.to_string());
                return Err(ScannerError::FormatError(FormatError {
                    PosRange: PosRange { Begin: begin, End: self.GetPos() },
                }));
            }
        };

        Ok(ch)
    }

    pub fn ScanEscapeChar(&mut self, quote: char) -> Result<char, ScannerError> {
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
                return Err(ScannerError::FormatError(FormatError { PosRange: PosRange { Begin: begin, End: self.GetPos() } }));
            }
        })
    }

    pub fn ScanString(&mut self, quote: char) -> Result<BasicToken, ScannerError> {
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

        Ok(BasicToken {
            Pos: PosRange { Begin: begin, End: self.GetPos() },
            Kind: BasicTokenKind::String,
            Literal: seq,
        })
    }

    pub fn ScanOperator(&mut self) -> Result<BasicToken, ScannerError> {
        let begin = self.GetPos();

        loop {
            match self.GetChar()? {
                '"' => { break; }
                '\'' => { break; }
                ch if !ch.is_ascii_punctuation() => { break; }
                ch if self.Delimiters.contains(&ch) => { break; }
                _ => { self.Move()?; }
            }
        }

        Ok(BasicToken {
            Pos: PosRange { Begin: begin.clone(), End: self.GetPos() },
            Kind: BasicTokenKind::Operator,
            Literal: from_to!(begin, self),
        })
    }

    pub fn Scan(&mut self) -> Result<BasicToken, ScannerError> {
        self.SkipWhitespaces()?;

        let begin = self.GetPos();

        match self.GetChar()? {
            ch if ch.is_alphabetic() => { self.ScanIdent() }
            ch if ch.is_numeric() => { self.ScanDigit() }
            ch if self.Delimiters.contains(&ch) => {
                Ok(BasicToken {
                    Pos: PosRange { Begin: begin, End: self.GetPos() },
                    Kind: BasicTokenKind::Delimiter,
                    Literal: vec![self.Move()?],
                })
            }
            '"' => { self.ScanString('"') }
            '\'' => { self.ScanString('\'') } // TODO
            '/' => { self.ScanComment() }
            ch if ch.is_ascii_punctuation() => { self.ScanOperator() }
            _ => { Err(ScannerError::EOFError(EOFError { Pos: self.GetPos() })) }
        }
    }
}
