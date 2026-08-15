use std::collections::HashMap;

pub fn generate_report(
    category_stats: &HashMap<String, usize>,
    difficulty_stats: &HashMap<String, usize>,
    approach_stats: &HashMap<String, usize>,
    average_gap: f64,
) -> String {
    let total = category_stats.values().sum::<usize>();

    let mut report = String::new();
    report.push_str("# NeetCode 150 Progress Report\n\n");

    // === SUMMARY ===
    report.push_str("## 📊 Summary\n\n");
    report.push_str(&format!("- **Total Problems Solved**: {} / 150\n", total));
    report.push_str(&format!("- **Progress**: {:.1}%\n", (total as f64 / 150.0) * 100.0));
    report.push_str(&format!("- **Average Gap**: {:.1} days\n\n", average_gap));

    // === BY CATEGORY ===
    report.push_str("## 📚 By Category\n\n");
    report.push_str("| Category | Count |\n");
    report.push_str("|----------|-------|\n");

    let mut sorted_categories: Vec<_> = category_stats.iter().collect();
    sorted_categories.sort_by_key(|k| std::cmp::Reverse(k.1));

    for (category, count) in sorted_categories {
        report.push_str(&format!("| {} | {} |\n", category, count));
    }
    report.push_str("\n");

    // === BY DIFFICULTY ===
    report.push_str("## 🎯 By Difficulty\n\n");
    report.push_str("| Difficulty | Count |\n");
    report.push_str("|------------|-------|\n");

    let difficulty_order = ["Easy", "Medium", "Hard"];
    for diff in &difficulty_order {
        if let Some(count) = difficulty_stats.get(*diff) {
            report.push_str(&format!("| {} | {} |\n", diff, count));
        }
    }
    report.push_str("\n");

    // === BY APPROACH ===
    report.push_str("## 🛠️ Top Approaches\n\n");
    report.push_str("| Approach | Used |\n");
    report.push_str("|----------|------|\n");

    let mut sorted_approaches: Vec<_> = approach_stats.iter().collect();
    sorted_approaches.sort_by_key(|k| std::cmp::Reverse(k.1));

    for (approach, count) in sorted_approaches.iter().take(5) {
        report.push_str(&format!("| {} | {} |\n", approach, count));
    }

    report
}