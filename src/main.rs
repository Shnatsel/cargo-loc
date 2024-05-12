use std::{collections::BTreeMap, path::Path};

use cargo_metadata::{MetadataCommand, PackageId};
use tokei::Languages;

fn main() {
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    if Some("loc") == args.first().map(|s| s.as_str()) {
        args.remove(0);
    }

    // TODO: rewrite --target into --filter-platform, other similar stuff
    let metadata = MetadataCommand::new()
        .other_options(args) // forwards our own arguments for feature selection, --offline etc
        .verbose(true) // forwards errors to stdout
        .exec()
        .unwrap();

    let resolve = metadata.resolve.unwrap();
    if !resolve.root.is_some() {
        unimplemented!(
            "Running in the workspace root is not yet supported. Please run on a specific crate."
        )
    }

    // TODO: filter out dev-dependencies? And optionally build-dependencies?
    // I've written this once for `cargo auditable`,
    // maybe I should uplift this into the `cargo_metadata` crate
    // to share code and stop everyone from reimplementing it over and over

    let mut reports: BTreeMap<PackageId, Languages> = BTreeMap::new();

    for node in &resolve.nodes {
        // TODO: technically quadratic, but does it matter?
        // Could be optimized with a BTreeMap but probably won't show up on profile
        let pkg = metadata.packages.iter().find(|p| p.id == node.id).unwrap();

        // Chop off the "/Cargo.toml" from the end of the manifest path
        let dir = pkg.manifest_path.as_std_path().parent().unwrap();

        // TODO: parallelize this even more? This is embarrassingly parallel. But is it worth the complexity?
        let report = count_crate_loc(dir);
        reports.insert(pkg.id.clone(), report);
    }

    // print N largest dependencies
    {
        // satisfy the borrow checker
        let n = 20;
        println!("Top {n} largest depdendencies:");
        for (pkg, stats) in top_n(&reports, n) {
            let total = stats.total();
            println!("{} lines ({} code): {pkg}", total.lines(), total.code);
        }
        println!(); // blank line for padding
    }

    let mut combined_report = Languages::new();
    for (_pkg, report) in reports {
        merge_reports(&mut combined_report, report)
    }

    // TODO: fancy pretty-printing like in tokei itself
    println!("Breakdown of the total lines by language:");
    for (lang, stats) in &combined_report {
        println!("{lang}, {}", stats.lines());
    }
}

fn count_crate_loc(dir: &Path) -> tokei::Languages {
    let included = &[dir];

    // The `tests` dir is not excluded because we cannot exclude same-file unit tests.
    // If we cannot exclude them, at least handle them consistently so we can document the behavior.
    // TODO: verify this really does the intended thing
    let excluded = &["benches", "examples"];

    let mut config = tokei::Config::default();
    config.sort = Some(tokei::Sort::Lines);

    let mut languages = tokei::Languages::new();
    languages.get_statistics(included, excluded, &config);
    languages
}

fn merge_reports(destination: &mut Languages, source: Languages) {
    for (lang, source_stats) in source {
        // += seems to be implemented only for `Language`, not `&mut Language`
        // so we work around that with `mem::take`
        let mut dest = std::mem::take(destination.entry(lang).or_default());
        dest += source_stats;
        destination.insert(lang, dest);
    }
}

fn top_n(reports: &BTreeMap<PackageId, Languages>, n: usize) -> Vec<(&PackageId, &Languages)> {
    let mut result: Vec<(&PackageId, &Languages)> = reports.iter().collect();
    result.sort_by_key(|(_pkg, stats)| stats.total().lines());
    // sorting puts smaller elements first, so the ones we need are at the end
    if result.len() > n {
        result.drain(..result.len() - n);
    }
    // reverse the remaining elements to put the largest one in front
    result.reverse();
    result
}
