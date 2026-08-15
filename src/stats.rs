let entries = parser::parse_readme("test1.md");

let category_stats = stats::count_by_category(&entries);
let difficulty_stats = stats::count_by_difficulty(&entries);
let approach_stats = stats::count_approaches(&entries);
let average_gap = stats::calculate_average_gap(&entries);

report::generate_report(
    &category_stats,
    &difficulty_stats,
    &approach_stats,
    average_gap,
);