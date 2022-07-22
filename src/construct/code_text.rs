//! Code (text) is a construct that occurs in the [text][] content type.
//!
//! It forms with the following BNF:
//!
//! ```bnf
//! ; Restriction: the number of markers in the closing sequence must be equal
//! ; to the number of markers in the opening sequence.
//! code_text ::= sequence 1*code sequence
//!
//! sequence ::= 1*'`'
//! ```
//!
//! The above grammar shows that it is not possible to create empty code.
//! It is possible to include grave accents (ticks) in code, by wrapping it
//! in bigger or smaller sequences:
//!
//! ```markdown
//! Include more: `a``b` or include less: ``a`b``.
//! ```
//!
//! When turning markdown into HTML, each line ending is turned into a space.
//!
//! It is also possible to include just one grave accent (tick):
//!
//! ```markdown
//! Include just one: `` ` ``.
//! ```
//!
//! Sequences are “gready”, in that they cannot be preceded or succeeded by
//! more grave accents (ticks).
//! To illustrate:
//!
//! ```markdown
//! Not code: ``x`.
//!
//! Not code: `x``.
//!
//! Escapes work, this is code: \``x`.
//!
//! Escapes work, this is code: `x`\`.
//! ```
//!
//! Yields:
//!
//! ```html
//! <p>Not code: ``x`.</p>
//! <p>Not code: `x``.</p>
//! <p>Escapes work, this is code: `<code>x</code>.</p>
//! <p>Escapes work, this is code: <code>x</code>`.</p>
//! ```
//!
//! That is because, when turning markdown into HTML, the first and last space,
//! if both exist and there is also a non-space in the code, are removed.
//! Line endings, at that stage, are considered as spaces.
//!
//! Code (text) relates to the `<code>` element in HTML.
//! See [*§ 4.5.15 The `code` element*][html-code] in the HTML spec for more
//! info.
//!
//! In markdown, it is possible to create code with the
//! [code (fenced)][code_fenced] or [code (indented)][code_indented] constructs
//! in the [flow][] content type.
//! Compared to code (indented), fenced code is more explicit and more similar
//! to code (text), and it has support for specifying the programming language
//! that the code is in, so it is recommended to use that instead of indented
//! code.
//!
//! ## Tokens
//!
//! *   [`CodeText`][Token::CodeText]
//! *   [`CodeTextData`][Token::CodeTextData]
//! *   [`CodeTextSequence`][Token::CodeTextSequence]
//! *   [`LineEnding`][Token::LineEnding]
//!
//! ## References
//!
//! *   [`code-text.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-core-commonmark/dev/lib/code-text.js)
//! *   [*§ 6.1 Code spans* in `CommonMark`](https://spec.commonmark.org/0.30/#code-spans)
//!
//! [flow]: crate::content::flow
//! [text]: crate::content::text
//! [code_indented]: crate::construct::code_indented
//! [code_fenced]: crate::construct::code_fenced
//! [html-code]: https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-code-element

use crate::token::Token;
use crate::tokenizer::{Code, State, StateFnResult, Tokenizer};

/// Start of code (text).
///
/// ```markdown
/// > | `a`
///     ^
/// > | \`a`
///      ^
/// ```
pub fn start(tokenizer: &mut Tokenizer, code: Code) -> StateFnResult {
    let len = tokenizer.events.len();

    match code {
        Code::Char('`')
            if tokenizer.parse_state.constructs.code_text
                && (tokenizer.previous != Code::Char('`')
                    || (len > 0
                        && tokenizer.events[len - 1].token_type == Token::CharacterEscape)) =>
        {
            tokenizer.enter(Token::CodeText);
            tokenizer.enter(Token::CodeTextSequence);
            sequence_open(tokenizer, code, 0)
        }
        _ => (State::Nok, 0),
    }
}

/// In the opening sequence.
///
/// ```markdown
/// > | `a`
///     ^
/// ```
fn sequence_open(tokenizer: &mut Tokenizer, code: Code, size: usize) -> StateFnResult {
    if let Code::Char('`') = code {
        tokenizer.consume(code);
        (
            State::Fn(Box::new(move |t, c| sequence_open(t, c, size + 1))),
            0,
        )
    } else {
        tokenizer.exit(Token::CodeTextSequence);
        between(tokenizer, code, size)
    }
}

/// Between something and something else
///
/// ```markdown
/// > | `a`
///      ^^
/// ```
fn between(tokenizer: &mut Tokenizer, code: Code, size_open: usize) -> StateFnResult {
    match code {
        Code::None => (State::Nok, 0),
        Code::CarriageReturnLineFeed | Code::Char('\n' | '\r') => {
            tokenizer.enter(Token::LineEnding);
            tokenizer.consume(code);
            tokenizer.exit(Token::LineEnding);
            (State::Fn(Box::new(move |t, c| between(t, c, size_open))), 0)
        }
        Code::Char('`') => {
            tokenizer.enter(Token::CodeTextSequence);
            sequence_close(tokenizer, code, size_open, 0)
        }
        _ => {
            tokenizer.enter(Token::CodeTextData);
            data(tokenizer, code, size_open)
        }
    }
}

/// In data.
///
/// ```markdown
/// > | `a`
///      ^
/// ```
fn data(tokenizer: &mut Tokenizer, code: Code, size_open: usize) -> StateFnResult {
    match code {
        Code::None | Code::CarriageReturnLineFeed | Code::Char('\n' | '\r' | '`') => {
            tokenizer.exit(Token::CodeTextData);
            between(tokenizer, code, size_open)
        }
        _ => {
            tokenizer.consume(code);
            (State::Fn(Box::new(move |t, c| data(t, c, size_open))), 0)
        }
    }
}

/// In the closing sequence.
///
/// ```markdown
/// > | `a`
///       ^
/// ```
fn sequence_close(
    tokenizer: &mut Tokenizer,
    code: Code,
    size_open: usize,
    size: usize,
) -> StateFnResult {
    match code {
        Code::Char('`') => {
            tokenizer.consume(code);
            (
                State::Fn(Box::new(move |t, c| {
                    sequence_close(t, c, size_open, size + 1)
                })),
                0,
            )
        }
        _ if size_open == size => {
            tokenizer.exit(Token::CodeTextSequence);
            tokenizer.exit(Token::CodeText);
            (State::Ok, if matches!(code, Code::None) { 0 } else { 1 })
        }
        _ => {
            let index = tokenizer.events.len();
            tokenizer.exit(Token::CodeTextSequence);
            // Change the token type.
            tokenizer.events[index - 1].token_type = Token::CodeTextData;
            tokenizer.events[index].token_type = Token::CodeTextData;
            between(tokenizer, code, size_open)
        }
    }
}
