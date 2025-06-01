use crate::logic::converter::json_input_to_json_entries::convert_user_input_to_entries;
use crate::logic::converter::json_to_atom;
use crate::logic::templating;
use crate::model::json::json_entries::JsonEntries;
use crate::model::json::json_entry::JsonEntry;
use crate::utils::atom_writer;
use crate::utils::json_reader::read_and_validate_input_json;
use crate::utils::json_writer::write_json_to_file;
use crate::{cli::Cli, utils::json_reader::read_json_from_file};
use env_logger::Env;
use inquire::Confirm;
use log::{debug, error, info};
use similar::{ChangeTag, TextDiff};

pub fn start_flow(args: Cli) -> std::io::Result<()> {
    // Set log level based on --debug flag
    let log_level = if args.debug { "debug" } else { "info" };
    env_logger::Builder::from_env(Env::default().default_filter_or(log_level)).init();
    match args.entries {
        Some(json) => {
            info!("Using JSON file: {}", json);

            let user_input = match read_and_validate_input_json(&json) {
                Ok(entries) => {
                    println!("Successfully read and validated input.json:");
                    for entry in &entries {
                        println!("{:?}", entry);
                    }
                    entries
                }
                Err(e) => {
                    error!("Error reading or validating input.json: {:?}", e);
                    return Err(e);
                }
            };

            let json_entries = convert_user_input_to_entries(user_input);

            let previous_json_entries: JsonEntries = read_json_from_file("entries.json")?;

            let updated_entries =
                JsonEntries::new(update_feed(&previous_json_entries, &json_entries));

            write_json_to_file(&updated_entries, "entries.json", Some(true))?;

            let atom_output_path = args
                .output_feed
                .clone()
                .unwrap_or_else(|| "feed.xml".to_string());

            info!("Writing Atom feed to: {}", atom_output_path);
            let atom_feed = json_to_atom::json_entries_to_atom(&updated_entries.entries);
            match atom_writer::write_atom_feed_to_file(&atom_feed, &atom_output_path) {
                Ok(_) => info!("Atom feed successfully written to {}", atom_output_path),
                Err(e) => {
                    error!("Failed to write Atom feed: {}", e);
                    return Err(e);
                }
            }
        }
        None => {
            info!("No file provided, creating default template entries.json!");
            let result: Result<(), std::io::Error> = templating::create_feed_json_template();

            if let Err(e) = result {
                error!("Error occurred: {}", e);
            }
        }
    }

    info!("MICHAEL DON'T LEAVE ME HERE MICHAELLLLL!!!");

    Ok(())
}

pub fn update_feed(old: &JsonEntries, new: &JsonEntries) -> Vec<JsonEntry> {
    // for every entry in the new feed,
    // check if it exists in the old feed
    // if it does, calculate the diff
    // if it doesn't, add it to the old feed
    // get ratio for all entries,
    // if it's below a certain threshold - that means there are changes but not enough to warrant a full update
    // prompt user and present ;) the diff
    // if user accepts, update the old feed AND update the updated field in the entry
    // if user rejects, update the entry without updating the updated field

    // now we have the diff entries + their diffs
    let (updated_old_entries, diffed_entries) = parse_new_entries(old, new);

    // show them to the user and let them decide their fate
    let user_input = diffed_entries
        .iter()
        .map(|(entry, ratio)| {
            let response = get_user_input(entry, ratio.clone());
            (entry.clone(), ratio, response)
        })
        .collect::<Vec<_>>();

    let updated_old_entries: Vec<JsonEntry> = updated_old_entries
        .into_iter()
        .map(|entry| {
            user_input
                .iter()
                .find(|(diff_entry, _, _)| entry.internal_id == diff_entry.internal_id)
                .map(|(_, _, response)| update_feed_entry(&entry, response.clone()))
                .unwrap_or(entry)
        })
        .collect();

    updated_old_entries
}

pub fn update_feed_entry(
    entry_to_update: &JsonEntry,
    updated_entry: (JsonEntry, f32, bool),
) -> JsonEntry {
    let (entry, ratio, response) = updated_entry;

    let mut updated_entry = entry_to_update.clone();

    if response {
        // User accepted the update
        updated_entry.updated = chrono::Utc::now().into();
        info!(
            "Entry '{}' updated with new content. Similarity ratio: {:.2}%",
            updated_entry.title,
            ratio * 100.0
        );
    } else {
        // User rejected the update, keep the original timestamp
        info!(
            "Entry '{}' not updated. Similarity ratio: {:.2}%",
            updated_entry.title,
            ratio * 100.0
        );
    }

    updated_entry.content = entry.content.clone();
    updated_entry.summary = entry.summary.clone();
    updated_entry.url = entry.url.clone();
    updated_entry.title = entry.title.clone();

    updated_entry
}

pub fn get_user_input(entry: &JsonEntry, ratio: f32) -> (JsonEntry, f32, bool) {
    println!(
        "Entry: {} - Similarity Ratio: {:.2}%",
        entry.title,
        ratio * 100.0
    );

    let user_input = Confirm::new(&format!(
        "Looks like you made a change to this entry: {}, do you want to mark it as updated?",
        entry.title
    ))
    .with_default(false)
    .with_help_message("Responding with 'no' will still update the entry, but will not change the updated timestamp on it. :)")
    .prompt();

    let response = match user_input {
        Ok(bool) => bool,
        Err(_) => false,
    };

    (entry.clone(), ratio, response)
}

pub fn parse_new_entries(
    old: &JsonEntries,
    new: &JsonEntries,
) -> (Vec<JsonEntry>, Vec<(JsonEntry, f32)>) {
    let mut old_entries = old.entries.to_owned();
    let new_entries = new.entries.clone();

    let mut diff_entries = Vec::new();

    let mut brand_new_entries = Vec::new();

    // Iterate over new entries and compare with old entries
    new_entries.iter().for_each(|new_entry| {
        old_entries.iter().for_each(|old_entry| {
            if old_entry.internal_id == new_entry.internal_id {
                // If diff ratio is over a certain threshold, we consider it a significant change
                // and we save it to diff_entries to be evaluated by the user
                let ratio = calculate_diff(old_entry, new_entry).unwrap_or(0.0);

                // otherwise, if there is no change, we keep the original entry, as it is the same
                if ratio != 1.0 {
                    diff_entries.push((new_entry.clone(), ratio));
                }
            } else {
                // Store entries that are brand new and can be added to the feed as is
                brand_new_entries.push(new_entry.clone());
            }
        })
    });

    old_entries.extend(brand_new_entries);

    (old_entries, diff_entries)
}

pub fn calculate_diff(old: &JsonEntry, new: &JsonEntry) -> std::io::Result<f32> {
    let old = old.content.clone();
    let new = new.content.clone();

    let diff = TextDiff::from_lines(&old, &new);

    let ratio = diff.ratio();
    debug!("Calculated similarity ratio: {:.2}%", ratio * 100.0);
    Ok(ratio)
}
