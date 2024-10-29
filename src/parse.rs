use std::str::FromStr;

use anyhow::anyhow;
use winnow::{
    combinator::{alt, opt, repeat, terminated},
    error::{ErrMode, ErrorKind, ParserError},
    prelude::*,
    token::take_until,
};

use crate::types::*;

impl FromStr for Questionaire {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_questionaire.parse(s).map_err(|err| anyhow!("{err}"))
    }
}

fn parse_tab(input: &mut &str) -> PResult<()> {
    alt(("\t", "    ")).void().parse_next(input)
}

fn parse_ws(input: &mut &str) -> PResult<()> {
    repeat(0.., alt(("\r", "\n", "\t", " "))).parse_next(input)
}

fn parse_questionaire(input: &mut &str) -> PResult<Questionaire> {
    repeat(0.., parse_question)
        .map(|questions| Questionaire { questions })
        .parse_next(input)
}

fn parse_question(input: &mut &str) -> PResult<Question> {
    let _ws = parse_ws.parse_next(input)?;
    let problem = parse_problem.parse_next(input)?;
    let answers = alt((
        parse_multis.map(Answers::Multi),
        parse_choices.map(Answers::Choice),
        parse_text.map(Answers::Text),
    ))
    .parse_next(input)?;

    Ok(Question { problem, answers })
}

fn parse_problem(input: &mut &str) -> PResult<String> {
    terminated(take_until(1.., "\n").map(ToOwned::to_owned), "\n").parse_next(input)
}

fn parse_multis(input: &mut &str) -> PResult<Vec<(String, bool)>> {
    repeat(2.., parse_multi).parse_next(input)
}

fn parse_multi(input: &mut &str) -> PResult<(String, bool)> {
    let _tab = parse_tab.parse_next(input)?;
    let correct = alt(("+".value(true), "-".value(false))).parse_next(input)?;
    let answer = parse_option.parse_next(input)?;

    Ok((answer, correct))
}

fn parse_choices(input: &mut &str) -> PResult<(Vec<String>, usize)> {
    let choices: Vec<_> = repeat(2.., parse_choice).parse_next(input)?;

    if choices.iter().filter(|(_, c)| *c).count() != 1 {
        Err(ErrMode::from_error_kind(input, ErrorKind::Verify))
    } else {
        let correct = choices.iter().position(|(_, c)| *c).unwrap();
        let choices = choices.into_iter().map(|(a, _)| a).collect();
        Ok((choices, correct))
    }
}

fn parse_choice(input: &mut &str) -> PResult<(String, bool)> {
    let _tab = parse_tab.parse_next(input)?;
    let correct = opt("*").map(|o| o.is_some()).parse_next(input)?;
    let answer = parse_option.parse_next(input)?;

    Ok((answer, correct))
}

fn parse_text(input: &mut &str) -> PResult<String> {
    let _tab = parse_tab.parse_next(input)?;
    let _arrow = ">".parse_next(input)?;
    let text = parse_option.parse_next(input)?;

    Ok(text)
}

fn parse_option(input: &mut &str) -> PResult<String> {
    (take_until(1.., "\n"), "\n")
        .map(|(ans, _): (&str, &str)| ans.trim().to_string())
        .parse_next(input)
}
