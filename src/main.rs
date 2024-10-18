use core::str;
use std::fmt;
use colored::Colorize;

use std::str::FromStr;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Debug)]
enum Article {
    Der,
    Die,
    Das
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Article::Die => write!(f, "{}", "Die".blue()),
            Article::Der => write!(f, "{}", "Der".red()),
            Article::Das => write!(f, "{}", "Das".green()),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.article, self.noun)
    }
}

impl str::FromStr for Article {
    type Err = () ;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let query = s.to_lowercase();
        match query.as_str() {
            "der" => Ok(Article::Der),
            "die" => Ok(Article::Die),
            "das" => Ok(Article::Das),
            _ => Err(())
        }
    }
}


#[derive(Debug)]
struct Entry {
    noun: String,
    article: Article,
}

/*
* Goal: prompt the user a noun and let him choose between der, die and das.
*
* Requirements
* ------------
* - The prompted noun should highlight why it has its gender.
* For example, termination in "ung" are almost always in "die".
*
* TODO
* ----
* - command 1: Given an ad-hoc list of words:
*   - List them, displaying color highlights for the 
*     article and the part of the word causing its gender, using the grammar rules.
*
* - Given a user input word:
*   - predict its gender based on grammar rules,
*   - make sure the prediction match reality (need a dictionary of Truth)
*
*/
fn main() {
    let truth  = [
        Entry{noun: format!("Zeit{}", "ung".blue()), article: Article::Die},
        Entry{noun: format!("Haus" ), article: Article::Das},
        Entry{noun: format!("Aut{}", "or".red() ), article: Article::Der},
    ];
    println!("====START 1====");
    for word in truth.iter() {
        println!("{}", word);
    }
    println!("====END 1====");

    println!("-----START 2-----");
    let query = "der zeitung";
    let entry = create_entry(query);
    verify_entry(&entry, truth);
    println!("query: {}", query);
    println!("guess: {}", entry);
    println!("-----END 2-----");
}


fn create_entry(query: &str)-> Entry {
    let words = query.split_whitespace().collect::<Vec<&str>>();
    let article = Article::from_str(words[0]).unwrap();
    let noun = words[1];
    Entry{noun: noun.to_string(), article: article}
}

fn verify_entry(entry: &Entry, truth: [Entry; 3]){
}

fn guess_article(noun: &str) -> Article{

}

