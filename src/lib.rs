///! An INI parser struct.
/// This struct can be used to parse an INI-formatted string or file and convert it into a
/// HashMap of HashMaps where each inner HashMap contains key-value pairs of properties in a section.
/// Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use dotini::INIParser;
/// use ini_parser::INIParser;
/// let content = r#"
/// [user]
/// name = John Doe
/// email = johndoe@example.com
/// "#;
///
/// let parser = INIParser::from_string(content).unwrap();
/// let output: HashMap<String, HashMap<String, String>> = parser.into_inner();
/// assert_eq!(output["user"]["name"], "John Doe");
/// assert_eq!(output["user"]["email"], "johndoe@example.com");
///
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;
use std::fs;

/// Generic Result type for dotini.
pub type INIParserResult<T> = Result<T, InIParseError>;

/// Possible error enum for dotini.
#[derive(Debug)]
pub enum InIParseError {
    FileReadError(String),
    UnsuccessfulParse(String),
    Finished,
    Unreachable,
}

/// Ini is the main parser that does the job for us.
/// takes some set of rules from ini.pest file.
#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct Ini;

/// The INIParser struct is used to parse INI configuration files into a HashMap data structure for easy access to configuration values.
/// To use the INIParser, we only need to create a new instance of the struct using either: `INIParser::from_string` or `INIParser::from_file`. the configuration values are stored in the output field of the struct
#[derive(Debug)]
pub struct INIParser {
    pub output: HashMap<String, HashMap<String, String>>,
}

impl INIParser {
    pub fn from_string(content: &str) -> INIParserResult<Self> {
        Self::parse(content)
    }

    /**
     * Creates a new INIParser struct from an INI file.
     *
     * # Arguments
     * * `path` - A string containing the path to the INI file to parse.
     *
     * # Returns
     * Returns an `INIParserResult` containing the parsed `INIParser` struct, or an `INIParseError`
     * if there is an issue reading or parsing the file.
     */
    pub fn from_file(path: &str) -> INIParserResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|err| InIParseError::FileReadError(err.to_string()))?;

        Self::parse(&content)
    }

    /**
     * Returns the inner HashMap of the `INIParser` struct.
     *
     * # Returns
     * Returns a `HashMap` where each key is a section in the INI file and the corresponding value
     * is another `HashMap` containing key-value pairs of properties in that section.
     */
    pub fn into_inner(self) -> HashMap<String, HashMap<String, String>> {
        self.output
    }

    /**
     * Parses an INI-formatted string and returns an `INIParser` struct containing the parsed content.
     *
     * # Arguments
     * * `content` - An INI-formatted string to parse.
     *
     * # Returns
     * Returns an `INIParserResult` containing the parsed `INIParser` struct, or an `INIParseError`
     * if there is an issue parsing the content.
     */
    fn parse(content: &str) -> INIParserResult<Self> {
        let ini = Ini::parse(Rule::file, content)
            .map_err(|err| InIParseError::UnsuccessfulParse(err.to_string()))?
            .next()
            .ok_or(InIParseError::UnsuccessfulParse(
                "Unsuccessful parse".to_string(),
            ))?;
        let mut output: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut current_section = "untagged".to_string();

        for line in ini.into_inner() {
            match line.as_rule() {
                Rule::section => {
                    current_section = line.into_inner()
                        .next()
                        .ok_or(InIParseError::Finished)?
                        .as_str()
                        .to_string();
                }
                Rule::property => {
                    let mut prop = line.into_inner();
                    let name = prop
                        .next()
                        .ok_or(InIParseError::Finished)?
                        .as_str()
                        .to_string();
                    let val = prop
                        .next()
                        .ok_or(InIParseError::Finished)?
                        .as_str()
                        .to_string();

                    output.entry(current_section.to_string())
                          .or_default()
                          .insert(name, val);
                }
                Rule::EOI => (),
                _ => Err(InIParseError::Unreachable),
            };
        }
        Ok(Self { output })
    }
}
