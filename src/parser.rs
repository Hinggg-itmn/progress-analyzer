use std::fs;

#[derive(Debug)]
pub struct Entry {
    pub date: String,
    pub name: String,
    pub category: String,
    pub difficulty: String,
    pub approach: String,
    pub complexity: String,
}

pub fn parse_readme(path: &str) -> Vec<Entry> {
    let content = fs::read_to_string(path)
        .expect("Failed to read README.md");

    let mut entries = Vec::new();
    let mut inside_rows = false;

    for line in content.lines() {
        if line.contains("<!-- ROWS -->") {
            inside_rows = true;
            continue;
        }

        if !inside_rows {
            continue;
        }

        if !line.starts_with('|') {
            continue;
        }

        let columns: Vec<&str> = line
            .split('|')
            .map(str::trim)
            .collect();

        if columns.len() < 8 {
            continue;
        }

        // Bỏ header/separator nếu sau này chúng xuất hiện
        if columns[1] == "#" || columns[1].starts_with("---") {
            continue;
        }

        let entry = Entry {
            date: columns[2].to_string(),
            name: clean_name(columns[3]),
            category: columns[4].to_string(),
            difficulty: columns[5].to_string(),
            approach: columns[6].to_string(),
            complexity: columns[7].to_string(),
        };

        entries.push(entry);
    }

    entries
}

fn clean_name(name: &str) -> String {
    name.replace(" (Update Approach)", "")
        .replace("[", "")
        .replace("]", "")
}