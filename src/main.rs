//! Creates a text-based adventure based on a series of yaml files in the `pages/` folder.

#![allow(dead_code)]

use log::{error, info, warn, Level};
use std::{collections::HashMap, env, error, fs};

mod story;
use story::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Web or console?
    let args: Vec<String> = env::args().collect();
    let is_console = args.contains(&"--cmd".to_string());
    if is_console {
        simple_log::quick!("info");
    } else {
        console_log::init_with_level(Level::Info)?;
    }

    // Load pages
    let mut pages: Pages = HashMap::new();
    let mut flags: Flags = HashMap::new();

    let pages_dir = fs::read_dir("pages")?;
    for entry in pages_dir {
        if let Ok(file) = entry {
            let file_path = file.path();
            let path_name = file_path
                .to_str()
                .ok_or("Could not convert file path to str")?;
            if path_name.ends_with(".yml") || path_name.ends_with(".yaml") {
                // Load the file
                let file_contents = fs::read_to_string(path_name)?;
                let page: Page = serde_yaml::from_str(&file_contents)?;
                pages.insert(page.id, page);
            }
        }
    }

    // Add all flags to list
    for (_, page) in &pages {
        if let Some(ref actions) = page.actions {
            for action in actions {
                flags.insert(action.flag, 0);
            }
        }

        for link in &page.links {
            if let Link::Action {
                action: ActionLink { actions, .. },
            } = link
            {
                for action in actions {
                    flags.insert(action.flag, 0);
                }
            }
        }
    }
    // Check all links for valid link ids and valid flags
    for (_, page) in &pages {
        for link in &page.links {
            if let Link::Page {
                page: PageLink { id: link_id, .. },
            } = link
            {
                if !pages.contains_key(&link_id) {
                    warn!("Invalid link id: `{}` on page id: `{}`", link_id, page.id);
                }
            }
        }
    }

    // Run in console if `--cmd` argument passed, otherwise run with Yew
    if is_console {
        error!("Sorry! Not supporting command line story yet!");
        return Err("Not available".into());
    } else {
        info!("Closing down!");
    }

    Ok(())
}
