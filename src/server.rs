use std::cell::RefCell;
use std::fs::File;

use bodyparser::{Json, MaxBodyLength};
use iron::{AfterMiddleware, Chain, Iron, IronResult, Plugin, Request, Response};
use iron::error::IronError;
use iron::headers::{ContentType, UserAgent};
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::status;
use persistent::Read;
use router::{NoRoute, Router};
use serde_json::Value as JsonValue;
use urlencoded::UrlEncodedBody;

header! { (XGithubEvent, "X-Github-Event") => [String]  }

// TODO: Verify github sha1 checksums.
fn github_handler(req: &mut Request) -> IronResult<Response> {
    info!("Got request at `/github`: {:?}", req);

    let event_type: RefCell<Event>;

    // New scope to do immutable borrows.
    {
        let github_event = match req.headers.get::<XGithubEvent>() {
            Some(e) => e,
            None => return Ok(Response::with(status::BadRequest)),
        };

        // Check if the User-Agent header exists and is from GitHub-Hookshot
        let agent = req.headers.get::<UserAgent>()
        .map_or(false, |user_agent| {
            match user_agent {
                &UserAgent(ref raw) => raw.starts_with("GitHub-Hookshot"),
            }
        });


        // If the headers are good, try to assign an Event value
        if let (true, event) = (agent, github_event.to_owned().0) {
            match parse_event(event) {
                Ok(e) => event_type = RefCell::new(e),
                Err(bad) => return Ok(bad),
            }
        } else {
            return Ok(Response::with(status::BadRequest));
        }
    }

    let json_body = req.get::<Json>();

    match json_body {
        Ok(Some(json_body)) => {
            match event_type.into_inner() {
                Event::WildCard => debug!("Got a wildcard"),
                Event::CommitComment => debug!("Got a commit comment"),
                Event::Create => debug!("Got a create"),
                Event::Delete => debug!("Got a delete"),
                Event::Deployment => debug!("Got a deployment"),
                Event::DeploymentStatus => debug!("Got a deployment status"),
                Event::Fork => debug!("Got a fork"),
                Event::Gollum => debug!("Got a gollum"),
                Event::IssueComment => {
                    debug!("Got an issue comment");
                },
                Event::Issues => debug!("Got an issues"),
                Event::Label => debug!("Got a label"),
                Event::Member => debug!("Got a member"),
                Event::Membership => debug!("Got a membership"),
                Event::Milestone => debug!("Got a milestone"),
                Event::Organization => debug!("Got an organization"),
                Event::PageBuild => debug!("Got a page build"),
                Event::ProjectCard => debug!("Got a project card"),
                Event::ProjectColumn => debug!("Got a project column"),
                Event::Project => debug!("Got a project"),
                Event::Public => debug!("Got a public"),
                Event::PullRequestReviewComment => debug!("Got a pull request review comment"),
                Event::PullRequestReview => debug!("Got a pull request review"),
                Event::PullRequest => {
                    debug!("Got a pull request");
                },
                Event::Push => {
                    debug!("Got a push");
                },
                Event::Repository => debug!("Got a repository"),
                Event::Release => debug!("Got a release"),
                Event::Status => debug!("Got a status"),
                Event::Team => debug!("Got a team"),
                Event::TeamAdd => debug!("Got a team add"),
                Event::Watch => debug!("Got a watch"),
            };
            Ok(Response::with(status::Ok))
        },
        Ok(None) => {
            debug!("No body");
            Ok(Response::with(status::BadRequest))
        },
        Err(e) => {
            error!("Error: {:?}", e);
            Ok(Response::with(status::BadRequest)) 
        },
    }
}

// TODO: Verify webhook requests (sha2)
// https://docs.travis-ci.com/user/notifications/#Configuring-webhook-notifications
fn travis_handler(req: &mut Request) -> IronResult<Response> {
    info!("Got request at `/travis`: {:?}", req);

    let json_body: JsonValue = match req.get::<UrlEncodedBody>() {
        Ok(ref hashmap) => {
            match hashmap.get("payload") {
                Some(buf) => match ::serde_json::from_str(&buf[0]) {
                    Ok(json) => json,
                    Err(e) => {
                        error!("Could not parse travis json: {:?}", e);
                        return Ok(Response::with(status::BadRequest))
                    },
                },
                None => return Ok(Response::with(status::BadRequest)),
            }
        },
        Err(ref e) => {
            error!("Could not parse travis webhook: {:?}", e);
            return Ok(Response::with(status::BadRequest))
        },
    };

    debug!("json_body: {:?}", json_body);

    Ok(Response::with(status::Ok))
}

fn index_handler(req: &mut Request) -> IronResult<Response> {
    info!("Got request at `/`: {:?}", req);

    let file = File::open("html/index.html").expect("index.html not found");
    let mut res = Response::with((status::Ok, file));
    res.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html,
                                     vec![(Attr::Charset, Value::Utf8)])));
    Ok(res)
}

fn favicon_handler(req: &mut Request) -> IronResult<Response> {
    info!("Got request at `/favicon.ico`: {:?}", req);

    let file = File::open("html/favicon.ico").expect("favicon.ico not found");
    let mut res = Response::with((status::Ok, file));
    res.headers.set(ContentType(Mime(TopLevel::Image, SubLevel::Ext("ico".to_string()), vec![])));
    Ok(res)
}

struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        debug!("Got 404: {:?}", req);

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "404: Not Found")))
        } else {
            Err(err)
        }
    }
}

#[derive(Debug)]
enum Event {
    WildCard,
    CommitComment,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    Member,
    Membership,
    Milestone,
    Organization,
    PageBuild,
    ProjectCard,
    ProjectColumn,
    Project,
    Public,
    PullRequestReviewComment,
    PullRequestReview,
    PullRequest,
    Push,
    Repository,
    Release,
    Status,
    Team,
    TeamAdd,
    Watch,
}

fn parse_event(event: String) -> Result<Event, Response> {
    match &*event {
        "*" => Ok(Event::WildCard),
        "commit_comment" => Ok(Event::CommitComment),
        "create" => Ok(Event::Create),
        "delete" => Ok(Event::Delete),
        "deployment" => Ok(Event::Deployment),
        "deployment_status" => Ok(Event::DeploymentStatus),
        "fork" => Ok(Event::Fork),
        "gollum" => Ok(Event::Gollum),
        "issue_comment" => Ok(Event::IssueComment),
        "issues" => Ok(Event::Issues),
        "label" => Ok(Event::Label),
        "member" => Ok(Event::Member),
        "membership" => Ok(Event::Membership),
        "milestone" => Ok(Event::Milestone),
        "organization" => Ok(Event::Organization),
        "page_build" => Ok(Event::PageBuild),
        "project_card" => Ok(Event::ProjectCard),
        "project_column" => Ok(Event::ProjectColumn),
        "project" => Ok(Event::Project),
        "public" => Ok(Event::Public),
        "pull_request_review_comment" => Ok(Event::PullRequestReviewComment),
        "pull_request_review" => Ok(Event::PullRequestReview),
        "pull_request" => Ok(Event::PullRequest),
        "push" => Ok(Event::Push),
        "repository" => Ok(Event::Repository),
        "release" => Ok(Event::Release),
        "status" => Ok(Event::Status),
        "team" => Ok(Event::Team),
        "team_add" => Ok(Event::TeamAdd),
        "watch" => Ok(Event::Watch),
        _ => Err(Response::with(status::BadRequest)),
    }
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

// The server for managing webhooks and http requests.
pub struct Server;

impl Server {
    /// Starts listening on the server.
    pub fn run(config: ::Config) {
        let mut router = Router::new();
        router.get("/", index_handler, "root");
        router.get("/index.html", index_handler, "index");
        router.get("/favicon.ico", favicon_handler, "favicon");
        router.post("/github", github_handler, "github");
        router.post("/travis", travis_handler, "travis");

        let mut chain = Chain::new(router);
        chain.link_before(Read::<MaxBodyLength>::one(MAX_BODY_LENGTH));
        chain.link_after(Custom404);

        match Iron::new(chain).http(config.url()) {
            Ok(l) => info!("Listening with listener: {:?}", l),
            Err(e) => error!("Failed to connect: {:?}", e),
        }
    }
}
