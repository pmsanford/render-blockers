extern crate dot;
extern crate goji;
extern crate structopt;

mod blocker_graph;

use blocker_graph::BlockerGraph;
use goji::{Credentials, Jira};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "render-blockers")]
struct Opt {
    /// Username for JIRA Cloud (probably your email)
    #[structopt(name = "username")]
    username: String,

    /// API Key for JIRA Cloud. Get here: https://id.atlassian.com/manage/api-tokens
    #[structopt(name = "api_key")]
    api_key: String,

    /// Address of your JIRA instance, including http/https prefix
    #[structopt(name = "jira_address")]
    jira_address: String,
}

fn main() {
    let opt = Opt::from_args();

    let query = "Sprint in openSprints()";

    let jira = Jira::new(
        opt.jira_address,
        Credentials::Basic(opt.username, opt.api_key),
    ).unwrap();

    let mut graph = BlockerGraph::new();

    match jira.search().iter(query, &Default::default()) {
        Ok(results) => {
            for issue in results {
                if let Some(Ok(links)) = issue.links() {
                    for link in links {
                        if link.link_type.name != "Blocks" {
                            continue;
                        }
                        if let Some(outward) = link.outward_issue {
                            graph.add(&issue, &outward);
                        } else if let Some(inward) = link.inward_issue {
                            graph.add(&inward, &issue);
                        }
                    }
                }
            }
        }
        Err(err) => panic!("{:#?}", err),
    }

    use std::fs::File;
    let mut f = File::create("blockers.dot").unwrap();
    dot::render(&graph, &mut f).unwrap()
}
