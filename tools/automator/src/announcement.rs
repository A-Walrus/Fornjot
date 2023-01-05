use std::{collections::HashSet, fmt::Write, path::PathBuf};

use anyhow::Context;
use chrono::{Datelike, Utc};
use map_macro::set;
use octocrab::Octocrab;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{
    pull_requests::{Author, PullRequest, PullRequestsSinceLastRelease},
    sponsors::Sponsors,
};

pub async fn create_release_announcement(
    octocrab: &Octocrab,
) -> anyhow::Result<()> {
    let now = Utc::now();

    let year = now.year();
    let week = format!("{:02}", now.iso_week().week());
    let date = format!("{year}-{:02}-{:02}", now.month(), now.day());

    let pull_requests_since_last_release =
        PullRequestsSinceLastRelease::fetch(octocrab).await?;

    let pull_requests =
        pull_requests_since_last_release.pull_requests.into_values();

    // For now, it's good enough to just release a new minor version every week.
    // We could also determine whether there were breaking changes to make sure
    // we actually need it, but as of now, breaking changes every week are
    // pretty much a given.
    let mut version = pull_requests_since_last_release.version_of_last_release;
    version.minor += 1;

    let min_dollars = 32;
    let for_readme = false;
    let sponsors = Sponsors::query(octocrab)
        .await?
        .as_markdown(min_dollars, for_readme)?;

    let mut file = create_file(year, &week).await?;
    generate_announcement(
        &week,
        date,
        version.to_string(),
        sponsors,
        pull_requests,
        &mut file,
    )
    .await?;

    Ok(())
}

async fn create_file(year: i32, week: &str) -> anyhow::Result<File> {
    let dir =
        PathBuf::from(format!("content/blog/weekly-release/{year}-w{week}"));
    let file = dir.join("index.md");

    // VS Code (and probably other editors/IDEs) renders the path in the output
    // as a clickable link, so the user can open the file easily.
    println!("Generating release announcement at {}", file.display());

    fs::create_dir_all(&dir).await.with_context(|| {
        format!("Failed to create directory `{}`", dir.display())
    })?;
    let file = File::create(&file).await.with_context(|| {
        format!("Failed to create file `{}`", file.display())
    })?;

    Ok(file)
}

async fn generate_announcement(
    week: &str,
    date: String,
    version: String,
    sponsors: String,
    pull_requests: impl IntoIterator<Item = PullRequest>,
    file: &mut File,
) -> anyhow::Result<()> {
    let mut pull_request_list = String::new();
    let mut pull_request_links = String::new();
    let mut author_links = String::new();

    let author_blacklist = set! {
        "hannobraun",
        "dependabot[bot]"
    };
    let mut authors = HashSet::new();

    for pull_request in pull_requests {
        let PullRequest {
            number,
            title,
            url,
            author,
            ..
        } = pull_request;

        let author = if author_blacklist.contains(author.name.as_str()) {
            None
        } else {
            Some(author)
        };

        let thanks = match author.as_ref() {
            Some(author) => format!("; thank you, [@{}]!", author.name),
            None => String::new(),
        };

        let item = format!("- {title} ([#{number}]{thanks})\n");
        pull_request_list.push_str(&item);

        let link = format!("[#{number}]: {url}\n");
        pull_request_links.push_str(&link);

        if let Some(Author { name, profile }) = author {
            if !authors.contains(&name) {
                let author_link = format!("[@{name}]: {profile}\n");
                author_links.push_str(&author_link);

                authors.insert(name.clone());
            }
        }
    }

    let mut buf = String::new();
    write!(
        buf,
        "\
+++
# TASK: Replace the calendar week with a descriptive title.
title = \"Weekly Release - 2022-W{week}\"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = {date}

# Uncomment to generate the HTML for the email newsletter.
# template = \"newsletter/weekly-release.html\"

[extra]
version = \"{version}\"
# TASK: Choose a descriptive subtitle.
subtitle = \"This is a subtitle\"
+++

**TASK: Write introduction.**


### Sponsors

{sponsors}

<strong class=\"call-to-action\">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href=\"https://github.com/sponsors/hannobraun\">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

**TASK: Add end-user improvements.**


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

**TASK: Add ecosystem improvements.**


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

**TASK: Add internal improvements.**


### Unsorted pull requests

**TASK: Sort into the categories above; update/merge as appropriate.**

{pull_request_list}
{pull_request_links}
{author_links}\
    "
    )?;

    file.write_all(buf.as_bytes()).await?;

    Ok(())
}
