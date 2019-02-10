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
    let archived = vec![
        Repo::new("playframework", "modules.playframework.com"),
        Repo::new("playframework", "modules.playframework.org"),
        Repo::new("playframework", "play-1.0-scala-module"),
        Repo::new("playframework", "play-glassfish"),
        Repo::new("playframework", "play-plugins"),
        Repo::new("playframework", "play-quota-java-example"),
        Repo::new("playframework", "play-quota-scala-example"),
        Repo::new("playframework", "play1"),
        Repo::new("playframework", "playclipse"),
        Repo::new("playframework", "prune"),
        Repo::new("playframework", "sbt-coffeescript"),
        Repo::new("lagom", "activator-lagom-cargotracker"),
        Repo::new("lagom", "activator-lagom-java"),
        Repo::new("lagom", "grpc-sbt-experiments"),
        Repo::new("lagom", "lagom-at-so"),
        Repo::new("lagom", "lagom-gameon-bazaar-service"),
        Repo::new("lagom", "lagom-gameon-example"),
        Repo::new("lagom", "lagom-gameon-maven-archetype"),
        Repo::new("lagom", "lagom-grpc-labs"),
        Repo::new("lagom", "persistence-api-experiments"),
        Repo::new("lagom", "sbt-lagom-descriptor-generator"),
    ];

    let res = online_repos.for_each(move |repo| {
        if !local_repos.contains(&repo) && !archived.contains(&repo) {
            println!(
                "- [{owner}/{name}](https://github.com/{owner}/{name})",
                owner = repo.owner,
                name = repo.name
            )
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

fn get_online_repos() -> impl Stream<Item = Repo, Error = hubcaps::Error> {
    use hubcaps::{repositories::*, *};
    let host = "https://api.github.com";
    let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let hyper_client = hyper::Client::builder().build(hyper_tls::HttpsConnector::new(4).unwrap());
    let http_cache = HttpCache::in_home_dir();
    let github = Github::custom(host, user_agent, None, hyper_client, http_cache);

    let opts = OrgRepoListOptions::default();
    let play_repos = github.org("playframework").repos().iter(&opts);
    let lagom_repos = github.org("lagom").repos().iter(&opts);
    let repos = play_repos.chain(lagom_repos);
    repos.map(|r| crate::Repo::new(r.owner.login, r.name))
}
