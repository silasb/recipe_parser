extern crate nom;
use std::println;

use std::fs;

use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_while, take_until1, take_until, is_not, take_till1, is_a},
    combinator::{recognize, rest, map_res, value, peek, eof, opt},
    character::complete::{newline, multispace0, char, space0, space1, alphanumeric1, alpha1, digit1},
    character::{is_newline},
    number::complete::{recognize_float},
    branch::alt,
    multi::{many1, many0, separated_list1, separated_list0},
    sequence::{pair, tuple, delimited, terminated, preceded},
};


//use recipe_parser::take_until_unbalanced;
//use crate::take_until_unbalanced;

#[derive(Debug,PartialEq)]
pub struct Target {
    pub name: String,
    pub comments: Vec<String>,
    //pub instructions: Vec<Instruction>,
}

#[derive(Debug,PartialEq)]
pub struct Ingredient {
    pub name:  String,
    pub amount: String,
    pub unit: String,
}

#[derive(Debug,PartialEq)]
pub struct Instruction {
    pub body: String,
}


#[derive(Debug,PartialEq)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex
        )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn comments(i: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        multispace0,
    many0(
        terminated(comment, newline)
        //tuple((
            //comment,
            //newline,
                //))
        )
    )(i)
        //separated_list0(newline, comment)(i)
}

#[test]
fn test_comments() {
    let input = r#"# test1
# test2
# test3
"#;
    assert_eq!(comments(input), Ok(("", vec!["test1", "test2", "test3"])));
}

fn comment(i: &str) -> IResult<&str, &str> {
    preceded(tag("#"), preceded(space0, take_until("\n")))(i)
}

#[test]
fn test_comment() {
    assert_eq!(comment("# hello world, 123 - blah\n"), Ok(("\n", "hello world, 123 - blah")));
}

fn target(input: &str) -> IResult<&str, Target> {
    let (input, comments) = comments(input)?;
    let comments2 = comments.iter().map(|s| s.to_string()).collect();

    let (input, name) =
        terminated(
            alphanumeric1,
            tag(":"),
            )(input)?;

    let name = String::from(name);

    //let (input, ingredients) = ingredients(input).expect("ingreidents");
    //println!("{:?} {:?}\n", input, target);


    //let (input, instructions) = instructions(input).expect("instructions");
    //println!("{:?} {:?}\n", input, target);

    Ok((input, Target { name, comments: comments2 }))
}

fn end_of_expression(i: &str) -> IResult<&str, ()> {
    alt((
        value((), eof),
        value((), peek(char('('))),
        //value((), char(' ')),
    ))(i)
}

//fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    //take_while1(ingredient)(input)
    //terminated(
        //alt((ingredient, terminated(parse_token, end_of_expression))),
        //multispace0,
    //)(i)
//}

fn parse_name(i: &str) -> IResult<&str, &str> {
    //alt((alphanumeric1, space1))(i)
    //recognize(
      //many1(
              //alt((alphanumeric1, space0)),
      //)
    //)(i)
    //terminated(
        //take_while(not_left_paren),
        //end_of_line,
        //)(i)
        //take_while(not_left_paren)(i)
    terminated(
    alt((
            take_until(" ("),
            take_till1(|c| c == '\n'),
            )),
            space0,
    )
    (i)
}

//fn end_of_line(i: &str) -> IResult<&str, &str> {
//}

fn not_left_paren(ch: char) -> bool {
    ch != '('
}

//fn parse_parens(i: &str) -> IResult<&str, &str> {
    //delimited(char('('), parse_token, char(')'))(i)
//}

fn parse_measurement(i: &str) -> IResult<&str, &str> {
    delimited(char('('), recognize(pair(recognize_float, alphanumeric1)), char(')'))(i)
}

#[test]
fn test_parse_measurement() {
    assert_eq!(parse_measurement("(1.4g)"), Ok(("", "1.4g")));
}


fn ingredient(i: &str) -> IResult<&str, Ingredient> {
    let (input, name) = preceded(
        space0,
        parse_name,
        )(i)?;

    if input == "" {
        return Ok((input, Ingredient { name: String::from(name), amount: String::from(""), unit: String::from("") }));
    }

    //let (_, measurement) = delimited(tag("("), alphanumeric1, tag(")"))(input)?;
    let (input, measurement) = parse_measurement(input)?;
    let (input2, amount) = recognize_float(measurement)?;
    let (_, unit) = alpha1(input2)?;

    let amount = String::from(amount);
    let unit = String::from(unit);

    Ok((input, Ingredient { name: String::from(name), amount, unit }))
}

fn ingredients(i: &str) -> IResult<&str, Vec<Ingredient>> {
    terminated(
        separated_list1(alt((tag(", "), tag(" ,"), tag(","))), ingredient),
        opt(newline),
        )(i)
}

fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    block(i)
    //separated_list1(tag("\n"), instruction)(i)
}

fn block(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, ident)(input)
}

fn ident(input: &str) -> IResult<&str, Instruction> {
    preceded(space1, instruction)(input)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (input, body) = alt((take_until("\n"), rest))(i)?;

    Ok((input, Instruction { body: String::from(body) }))
}

#[derive(PartialEq, Eq, Debug)]
struct Recipe {
    first_name: String,
    last_name: String,
}

impl Recipe {
    //fn parse(i: &str) -> IResult<&str, &str> {
        //let mut recipe_parser = map(
        //);
    //}
}

#[test]
fn test_recipe_parser() {
    let recipe = String::from(r#"
# makes 4 dough balls
dough: water (368g), salt (18g), instant dry yeast (1.4g), 00 flour (613g)
	warm water # to a little more than luke warm
	mix water, yeast, salt together into wet mixture # until everything disolves
	combine flour and wet mixture together into stand mixer
	mix for about 6m-8m on low setting
	proof in the bowl for about 2-3 hours
	divide into 250g balls and place into a regfregiated tube for 48+ hours
	put a little olive oil on balls to prevent skinning
    "#);
    //let file_path = "/home/silas/repos/sbaronda_blog/apps/recipes/recipes/pizza.recipe";
    //let recipe = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //println!("{:?}", recipe);
    let (input, target) = target(&recipe).expect("target");
    println!("{:?} {:?}\n", input, target);
    //Recipe::parse(&contents)

}

fn main() {
    //Recipe::parse()
}

#[test]
fn test_parse_target() {
    assert_eq!(target("pizza:"), Ok(("", Target {
        name: String::from("pizza"),
        comments: vec![],
    })));
}

#[test]
fn test_parse_target_with_comments() {
    assert_eq!(target("\n# makes 4 dough balls\npizza:"), Ok(("", Target {
        name: String::from("pizza"),
        comments: vec!["makes 4 dough balls".to_string()],
    })));
}

#[test]
fn parse_instructions() {
    let input = r#"	mix water, yeast, salt together into wet mixture # until everything disolves
	warm water # to a little more than luke warm"#;
    assert_eq!(instructions(input), Ok(("", vec![
                                                                Instruction {
                                                                    body: String::from("mix water, yeast, salt together into wet mixture # until everything disolves"),
                                                                },
                                                                Instruction {
                                                                    body: String::from("warm water # to a little more than luke warm"),
                                                                }
    ])));
}

#[test]
fn parse_ingredients() {
    assert_eq!(ingredients("sauce (16g), 00 flour (368g)"), Ok(("", vec![
                                                                Ingredient {
                                                                    name: String::from("sauce"),
                                                                    amount: String::from("16"),
                                                                    unit: String::from("g"),
                                                                },
                                                                Ingredient {
                                                                    name: String::from("00 flour"),
                                                                    amount: String::from("368"),
                                                                    unit: String::from("g"),
                                                                }
    ])));
}

#[test]
fn parse_ingredients_with_no_space_between_comma() {
    assert_eq!(ingredients("sauce (16g),00 flour (368g)"), Ok(("", vec![
                                                                Ingredient {
                                                                    name: String::from("sauce"),
                                                                    amount: String::from("16"),
                                                                    unit: String::from("g"),
                                                                },
                                                                Ingredient {
                                                                    name: String::from("00 flour"),
                                                                    amount: String::from("368"),
                                                                    unit: String::from("g"),
                                                                }
    ])));
}

#[test]
fn parse_ingredients_with_space_after_paren() {
    assert_eq!(ingredients("sauce (16g) ,00 flour (368g)"), Ok(("", vec![
                                                                Ingredient {
                                                                    name: String::from("sauce"),
                                                                    amount: String::from("16"),
                                                                    unit: String::from("g"),
                                                                },
                                                                Ingredient {
                                                                    name: String::from("00 flour"),
                                                                    amount: String::from("368"),
                                                                    unit: String::from("g"),
                                                                }
    ])));
}

#[test]
fn parse_ingredient() {
    assert_eq!(ingredient("sauce (16g)"), Ok(("", Ingredient {
        name: String::from("sauce"),
        amount: String::from("16"),
        unit: String::from("g"),
    })));
}

#[test]
fn parse_ingredient_with_float_value() {
    assert_eq!(ingredient("instant dry yeast (1.4g)"), Ok(("", Ingredient {
        name: String::from("instant dry yeast"),
        amount: String::from("1.4"),
        unit: String::from("g"),
    })));
}

#[test]
fn parse_ingredient_with_numeric_value() {
    assert_eq!(ingredient("00 flour (16g)"), Ok(("", Ingredient {
        name: String::from("00 flour"),
        amount: String::from("16"),
        unit: String::from("g"),
    })));
}

#[test]
fn parse_ingredient_name_with_spaces() {
    assert_eq!(ingredient("sauce mez (16g)"), Ok(("", Ingredient {
        name: String::from("sauce mez"),
        amount: String::from("16"),
        unit: String::from("g"),
    })));
}


#[test]
fn parse_ingredient_without_unit() {
    assert_eq!(ingredient("sauce"), Ok(("", Ingredient {
        name: String::from("sauce"),
        amount: String::from(""),
        unit: String::from(""),
    })));
}

#[test]
fn parse_ingredient_without_unit_and_name_with_spaces() {
    assert_eq!(ingredient("sauce mez"), Ok(("", Ingredient {
        name: String::from("sauce mez"),
        amount: String::from(""),
        unit: String::from(""),
    })));
}
