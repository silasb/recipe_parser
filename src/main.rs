extern crate nom;
use std::println;

//use std::fs;

use nom::{
    IResult,
    error::ParseError,
    bytes::complete::{tag, take_while_m_n, take_while, take_until1, take_till, take_until, is_not, take_till1, is_a},
    combinator::{success, cut, recognize, rest, map_res, value, peek, eof, opt, fail},
    character::complete::{newline, multispace0, multispace1, char, space0, space1, alphanumeric0, alphanumeric1, alpha1, digit1},
    character::{is_newline},
    number::complete::{recognize_float},
    branch::alt,
    multi::{many1, many0, separated_list1, separated_list0, fold_many0},
    sequence::{pair, tuple, delimited, terminated, preceded},
};


//use recipe_parser::take_until_unbalanced;
//use crate::take_until_unbalanced;

#[derive(Debug,PartialEq)]
pub struct Target {
    pub name: String,
    pub comments: Vec<String>,
    pub ingredients: Option<Vec<Ingredient>>,
    pub instructions: Vec<Instruction>,
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

    let (input, name) = terminated(
        alphanumeric1,
        tag(":"),
    )(input)?;

    let name = String::from(name);

    println!("{:?}\n\n", input);
    let (input, ingredients) = terminated(ingredients, newline)(input)?;
    println!("ingredients: {:?} {:?}", input, ingredients);

    let (input, instructions) = instructions(input)?;
    println!("instructions: {:?} {:?}", input, instructions);

    Ok((input, Target { name, comments: comments2, ingredients, instructions }))
}

fn end_of_expression(i: &str) -> IResult<&str, ()> {
    alt((
        value((), eof),
        value((), peek(char('('))),
        //value((), char(' ')),
    ))(i)
}


//fn syllable(i: &str) -> IResult<&str, Syllable> {
    //let (rest, (_, pronunciation)) = tuple((
        //space0,
        //alpha1,
    //))(i)?;

    //Ok((rest, Syllable::new(pronunciation, tone)))
//}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

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

    let (input, mut out) = 
        many1(
            alt((space1, alphanumeric1))
        )
    (i)?;

    if let Some(last) = out.last() {
        if last.trim().is_empty() {
            out.pop();
        }
    }

    let static_ref: &'static mut str = out.join("").leak();

    Ok((input, static_ref))
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

//#[test]
//fn test_parse_measurement_empty() {
    //assert_eq!(parse_measurement(""), Ok(("", "")));
//}


fn ingredient(i: &str) -> IResult<&str, Ingredient> {
    let (input, name) = preceded(
            space0,
            parse_name,
        )(i)?;

    println!("here1: {:?} {:?}\n", input, name);

    let (input2, name2) = terminated(
        tag(name),
        opt(space1),
    )(name)?;
    //if input == "" {
        //return Ok((input, Ingredient { name: String::from(name), amount: String::from(""), unit: String::from("") }));
    //}
    println!("here2: {:?} {:?}\n", input2, name2);
    //println!("here2: {:?} {:?}\n", input2, name2);

    //let (_, measurement) = delimited(tag("("), alphanumeric1, tag(")"))(input)?;
    let (input, measurement) = opt(parse_measurement)(input)?;
    if let Some(x) = measurement {
        let (input2, amount) = recognize_float(x)?;
        let (_, unit) = alpha1(input2)?;
        let amount = String::from(amount);
        let unit = String::from(unit);
        Ok((input, Ingredient { name: String::from(name), amount, unit }))
    } else {
        Ok((input, Ingredient { name: String::from(name), amount: String::from(""), unit: String::from("") }))
    }
}

fn ingredients(i: &str) -> IResult<&str, Option<Vec<Ingredient>>> {
    opt(
        separated_list1(alt((tag(", "), tag(" ,"), tag(","))), ingredient),
        )(i)
}

fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    block(i)
}

fn block(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, ident)(i)
}

fn ident(i: &str) -> IResult<&str, Instruction> {
    preceded(space1, instruction)(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    //let (input, body) = alt((take_until("\n"), eof))(i)?;
    //let (input, body) = take_while(|c| c != '\n')(i)?;
    let (input, body) = take_until("\n")(i)?;
    //println!("here2343 {:?} {:?}\n", input, body);

    if body == "" {
        fail::<_, &str, _>(input)?;
    }
    //let (input, test) = eof(input)?;
    //println!("{:?}", test);

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

test2: blah
	test


# comment 1
test3: blah (100g), simple sugar(100g)
	hello world
	hi

    "#);
    //let file_path = "/home/silas/repos/sbaronda_blog/apps/recipes/recipes/pizza.recipe";
    //let recipe = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", recipe);
    let (input, target2) = target(&recipe).expect("recipe");
    println!("{:?} {:?}\n", input, target2);

    let (input, target3) = target(input).expect("recipe");
    println!("{:?} {:?}\n", input, target3);
    assert_eq!(target3, Target {
        name: "test2".to_string(),
        comments: vec![],
        ingredients: Some(vec![Ingredient { name: "blah".to_string(), amount: "".to_string(), unit: "".to_string() }]),
        instructions: vec![Instruction { body: "test".to_string() }],
    });

    let (input, target4) = target(input).expect("recipe");
    println!("{:?} {:?}\n", input, target4);
    assert_eq!(target4, Target {
        name: "test3".to_string(),
        comments: vec!["comment 1".to_string()],
        ingredients: Some(vec![
            Ingredient { name: "blah".to_string(), amount: "100".to_string(), unit: "g".to_string() },
            Ingredient { name: "simple sugar".to_string(), amount: "100".to_string(), unit: "g".to_string() },
        ]),
        instructions: vec![Instruction { body: "hello world".to_string() }, Instruction { body: "hi".to_string() }],
    });

    //Recipe::parse(&contents)
}

fn main() {
    //Recipe::parse()
}

#[test]
fn test_parse_target() {
    let input = r#"
pizza:
	blah
"#;
    assert_eq!(target(input), Ok(("\n", Target {
        name: String::from("pizza"),
        comments: vec![],
        ingredients: None,
        instructions: vec![Instruction { body: "blah".to_string() }],
    })));
}

#[test]
fn test_parse_target_with_ingredients() {
    let input = r#"
pizza: ingredient1
	blah
"#;
    assert_eq!(target(input), Ok(("\n", Target {
        name: String::from("pizza"),
        comments: vec![],
        ingredients: Some(vec![Ingredient { name: "ingredient1".to_string(), amount: "".to_string(), unit: "".to_string()}]),
        instructions: vec![Instruction { body: "blah".to_string() }],
    })));
}

#[test]
fn test_parse_target_with_comments() {
    let input = r#"
# makes 4 dough balls
pizza:
	blah
	test2
"#;
    assert_eq!(target(input), Ok(("\n", Target {
        name: String::from("pizza"),
        comments: vec!["makes 4 dough balls".to_string()],
        ingredients: None,
        instructions: vec![Instruction { body: "blah".to_string() }, Instruction { body: "test2".to_string() }],
    })));
}

#[test]
fn parse_instructions() {
    let input = r#"	mix water, yeast, salt together into wet mixture # until everything disolves
	warm water # to a little more than luke warm
    "#;
    assert_eq!(instructions(input), Ok(("\n    ", vec![
                                                                Instruction {
                                                                    body: String::from("mix water, yeast, salt together into wet mixture # until everything disolves"),
                                                                },
                                                                Instruction {
                                                                    body: String::from("warm water # to a little more than luke warm"),
                                                                }
    ])));
}

#[test]
fn parse_empty_ingredients() {
    assert_eq!(ingredients(""), Ok(("", None)));
}

#[test]
fn parse_ingredients() {
    assert_eq!(ingredients("sauce (16g), 00 flour (368g)"), Ok(("", Some(vec![
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
    ]))));
}

#[test]
fn parse_ingredients_with_no_space_between_comma() {
    assert_eq!(ingredients("sauce (16g),00 flour (368g)"), Ok(("", Some(vec![
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
    ]))));
}

#[test]
fn parse_ingredients_with_space_after_paren() {
    assert_eq!(ingredients("sauce (16g) ,00 flour (368g)"), Ok(("", Some(vec![
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
    ]))));
}

#[test]
fn parse_ingredients_with_only_one() {
    assert_eq!(ingredients("sauce (16g)\n"), Ok(("\n", Some(vec![
                                                                Ingredient {
                                                                    name: String::from("sauce"),
                                                                    amount: String::from("16"),
                                                                    unit: String::from("g"),
                                                                },
    ]))));
}

#[test]
fn parse_ingredients_with_only_one_with_no_amount_unit() {
    assert_eq!(ingredients("sauce\n"), Ok(("\n", Some(vec![
                                                                Ingredient {
                                                                    name: String::from("sauce"),
                                                                    amount: String::from(""),
                                                                    unit: String::from(""),
                                                                },
    ]))));
}

#[test]
fn parse_ingredients_with_no_amount_unit_and_one_with() {
    assert_eq!(ingredients("sauce, blah    (213g), simple  sugar \n"), Ok(("\n", Some(vec![
                                                                Ingredient {
                                                                    name: String::from("sauce"),
                                                                    amount: String::from(""),
                                                                    unit: String::from(""),
                                                                },
                                                                Ingredient {
                                                                    name: String::from("blah"),
                                                                    amount: String::from("213"),
                                                                    unit: String::from("g"),
                                                                },
                                                                Ingredient {
                                                                    name: String::from("simple  sugar"),
                                                                    amount: String::from(""),
                                                                    unit: String::from(""),
                                                                },
    ]))));
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
