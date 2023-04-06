 # dotini

 `dotini` is a Rust library for parsing INI files into a HashMap.

 ## Example

 ```rust
 use dotini::{INIParser, INIParserResult};

 fn main() -> INIParserResult<()> {
     let content = "[section1]\nname1=value1\nname2=value2\n[section2]\nname3=value3";

     let parser = INIParser::from_string(content)?;

     let output = parser.into_inner();

     assert_eq!(output["section1"]["name1"], "value1");
     assert_eq!(output["section1"]["name2"], "value2");
     assert_eq!(output["section2"]["name3"], "value3");

     Ok(())
 }
 ```

 ## Usage

 Add the following to your `Cargo.toml` file:

 ```toml
 [dependencies]
 dotini = "0.1.0"
 ```

 ## API

 The `INIParser` struct has the following methods:

 * `from_string(content: &str) -> INIParserResult<Self>`

     Creates an `INIParser` instance from an INI-formatted string.

 * `from_file(path: &str) -> INIParserResult<Self>`

     Creates an `INIParser` instance from an INI-formatted file.

 * `into_inner(self) -> HashMap<String, HashMap<String, String>>`

     Returns the parsed INI data as a `HashMap<String, HashMap<String, String>>`.

 ## License

 This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
