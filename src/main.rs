extern crate goji;

use goji::{Credentials, Jira};

struct Node {
    id: u64,
    key: String,
    summary: String,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Edge {
    from_id: u64,
    to_id: u64,
}

fn main() {
    let host = "";
    let username = "".to_string();
    let pass = "".to_string();

    let query = "Status = Open and Sprint in openSprints()";

    let jira = Jira::new(host, Credentials::Basic(username, pass)).unwrap();

    let mut points: f64 = 0.0;
    let mut unblocked_points: f64 = 0.0;
    let mut blocked_points: f64 = 0.0;

    match jira.search().iter(query, &Default::default()) {
        Ok(results) => {
            for issue in results {
                println!("Links for [{}] {}:", issue.key, issue.summary().unwrap());
                let issue_points = issue
                    .field::<f64>("customfield_10025")
                    .unwrap_or(Ok(0.0))
                    .unwrap_or(0.0);
                points += issue_points;
                let mut blocked = false;
                if let Some(Ok(links)) = issue.links() {
                    for link in links {
                        if let Some(outward) = link.outward_issue {
                            println!("\t{} {}", link.link_type.outward, outward.key);
                        } else if let Some(inward) = link.inward_issue {
                            if link.link_type.name == "Blocks" {
                                if let None = inward.resolution() {
                                    blocked = true;
                                }
                            }
                            println!("\t{} {}", link.link_type.inward, inward.key);
                        }
                    }
                    if blocked {
                        blocked_points += issue_points;
                    } else {
                        unblocked_points += issue_points;
                    }
                }
            }
            println!("Total points open: {:#?}", points);
            println!("Blocked: {:#?}", blocked_points);
            println!("Unblocked: {:#?}", unblocked_points);
        }
        Err(err) => panic!("{:#?}", err),
    }
}
