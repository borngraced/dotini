use dotini::INIParser;

fn main() {
    let ini = INIParser::from_file("src/config.ini").unwrap();
    let output = ini.into_inner();

    for line in output {
        println!("{:?}", line.0);
        println!("{:?}", line.1);
        println!("{:=>60}", "");
    }
}
