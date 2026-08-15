mod parser;
mod stats;
mod report;

fn main() {
    let entries = parser::parse_readme("README.md");

    for entry in &entries {
        println!("{entry:?}");
    }

    println!("Total: {}", entries.len());
}