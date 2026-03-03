use std::fs::File;
use std::io::Read;

fn getValteraDraugi() -> Result<Vec<String>,String> {
    let mut file = match File::open("friends.txt") {
        Ok(f) => f,
        Err(e) => return Err(format!("Nevareja atvert draugu sarakstu: {}", e)),
    };
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        return Err(format!("Nevareja salasit draugu sarakstu: {}", e));
    }
    let draugi: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    Ok(draugi)
}

fn getContestStandings(contestId: &str, handles: Vec<String>) -> Result<Vec<String>,String> {
    let happyUsers: Vec<String> = Vec::new();

    Ok(happyUsers)
}

fn main() {
    let handles = vec!["A".to_string(), "B".to_string()];
    match getValteraDraugi() {
        Ok(res) => {
            println!("draugi: {:?}", res);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
