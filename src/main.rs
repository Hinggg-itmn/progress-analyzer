mod parser;
mod report;
mod stats;

use std::fs;

fn main() {
    // 1. Extract: đọc README, parse ra danh sách entry
    let entries = parser::parse_readme("test1.md");
    println!("Đã parse {} entry từ README.\n", entries.len());

    // 2. Transform: tính các thống kê từ entries
    let category_stats = stats::count_by_category(&entries);
    let difficulty_stats = stats::count_by_difficulty(&entries);
    let approach_stats = stats::count_approaches(&entries);
    let average_gap = stats::calculate_average_gap(&entries);

    // 3. Load: build report dạng markdown, in ra + ghi ra file
    let report = report::generate_report(
        &category_stats,
        &difficulty_stats,
        &approach_stats,
        average_gap,
    );

    println!("{report}");

    match fs::write("progress_report.md", &report) {
        Ok(()) => println!("Đã ghi report vào progress_report.md"),
        Err(e) => eprintln!("Lỗi khi ghi file report: {e}"),
    }
}