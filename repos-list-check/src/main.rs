use std::fs;
use std::path::Path;

use futures::stream::Stream;

#[derive(Debug, PartialEq)]
struct Repo {
    pub owner: String,
    pub name: String,
}

impl Repo {
    fn new<O: Into<String>, N: Into<String>>(owner: O, name: N) -> Repo {
        Repo {
            owner: owner.into(),
            name: name.into(),
        }
    }
}

fn main() {
    let online_repos = get_online_repos();
    let local_repos = get_local_repos();
    let excluded = vec![
        Repo::new("playframework", "modules.playframework.com"),
        Repo::new("playframework", "play1"),
        Repo::new("lagom", "lagom-at-so"),
    ];

    let res = online_repos.for_each(move |r| {
        let repo = Repo::new(r.owner.login.as_ref(), r.name.as_str());
        if !local_repos.contains(&repo) && !excluded.contains(&repo) && !r.archived {
            println!(
                "- [{owner}/{name}](https://github.com/{owner}/{name})",
                owner = repo.owner,
                name = repo.name,
            )
        }
        if r.archived && excluded.contains(&repo) {
            println!("Can drop {:?} from excluded", repo);
        }
        if local_repos.contains(&repo) && r.archived {
            println!("Can drop {:?} from repos.md", repo);
        }
        Ok(())
    });

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(res).unwrap();
}

fn get_local_repos() -> Vec<Repo> {
    use comrak::*;
    let text = fs::read_to_string(Path::new("../repos.md")).unwrap();
    let arena = Arena::new();
    let root = comrak::parse_document(&arena, &text, &ComrakOptions::default());
    root.descendants().fold(vec![], |mut repos, node| {
        match &node.data.borrow().value {
            nodes::NodeValue::Link(link) => {
                let str = String::from_utf8(link.url.clone()).unwrap();
                let re = regex::Regex::new(r"^https://github\.com/([^/]+)/([^/]+)$").unwrap();
                re.captures(&str).map(|c| {
                    let owner = c.get(1).unwrap().as_str();
                    let name = c.get(2).unwrap().as_str();
                    repos.push(Repo::new(owner, name));
                });
            }
            _ => (),
        }
        repos
    })
}

fn get_online_repos() -> impl Stream<Item = hubcaps::repositories::Repo, Error = hubcaps::Error> {
    use hubcaps::{repositories::*, *};
    let host = "https://api.github.com";
    let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let hyper_client = hyper::Client::builder().build(hyper_tls::HttpsConnector::new(4).unwrap());
    let http_cache = HttpCache::in_home_dir();
    let github = Github::custom(host, user_agent, None, hyper_client, http_cache);

    let opts = OrgRepoListOptions::default();
    let play_repos = github.org("playframework").repos().iter(&opts);
    let lagom_repos = github.org("lagom").repos().iter(&opts);
    play_repos.chain(lagom_repos)
}
