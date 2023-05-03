use crate::{settings::structs::Settings, ApiError, IpAddr};
use actix_web::dev::ConnectionInfo;
use itertools::Itertools;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::{Regex, RegexBuilder};
use time;
lazy_static! {
  static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").expect("compile regex");
  static ref SLUR_REGEX: Regex = RegexBuilder::new(r"(fag(g|got|tard)?\b|cock\s?sucker(s|ing)?|\bn(i|1)g(\b|g?(a|er)?(s|z)?)\b|mudslime?s?|kikes?|\bspi(c|k)s?\b|\bchinks?|gooks?|bitch(es|ing|y)?|whor(es?|ing)|\btr(a|@)nn?(y|ies?)|\b(b|re|r)tard(ed)?s?)").case_insensitive(true).build().expect("compile regex");
  static ref USERNAME_MATCHES_REGEX: Regex = Regex::new(r"/users/[a-zA-Z][0-9a-zA-Z_]*").expect("compile regex");
  // TODO keep this old one, it didn't work with port well tho
  // static ref MENTIONS_REGEX: Regex = Regex::new(r"@(?P<name>[\w.]+)@(?P<domain>[a-zA-Z0-9._-]+\.[a-zA-Z0-9_-]+)").expect("compile regex");
  static ref MENTIONS_REGEX: Regex = Regex::new(r"@(?P<name>[\w.]+)@(?P<domain>[a-zA-Z0-9._:-]+)").expect("compile regex");
  static ref VALID_USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]{3,20}$").expect("compile regex");
  static ref VALID_SCHOOL_NAME_REGEX: Regex = Regex::new(r"^[a-z0-9_]{3,20}$").expect("compile regex");
  static ref VALID_CATEGORY_TITLE_REGEX: Regex = Regex::new(r"^[a-z0-9_]{3,20}$").expect("compile regex");
  static ref VALID_COURSE_TITLE_REGEX: Regex = Regex::new(r".*\S.*").expect("compile regex");
}



pub fn remove_slurs(test: &str) -> String {
    SLUR_REGEX.replace_all(test, "*removed*").to_string()
}

pub(crate) fn slur_check(test: &str) -> Result<(), Vec<&str>> {
    let mut matches: Vec<&str> = SLUR_REGEX.find_iter(test).map(|mat| mat.as_str()).collect();

    // Unique
    matches.sort_unstable();
    matches.dedup();

    if matches.is_empty() {
        Ok(())
    } else {
        Err(matches)
    }
}

pub fn check_slurs(text: &str) -> Result<(), ApiError> {
    if let Err(slurs) = slur_check(text) {
        Err(ApiError::err(&slurs_vec_to_str(slurs)))
    } else {
        Ok(())
    }
}

pub fn check_slurs_opt(text: &Option<String>) -> Result<(), ApiError> {
    match text {
        Some(t) => check_slurs(t),
        None => Ok(()),
    }
}

pub(crate) fn slurs_vec_to_str(slurs: Vec<&str>) -> String {
    let start = "No slurs - ";
    let combined = &slurs.join(", ");
    [start, combined].concat()
}

pub fn generate_random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(30)
        .collect()
}

pub fn markdown_to_html(text: &str) -> String {
    comrak::markdown_to_html(text, &comrak::ComrakOptions::default())
}

// TODO nothing is done with schools / group webfingers yet, so just ignore those for now
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MentionData {
    pub name: String,
    pub domain: String,
}

impl MentionData {
    pub fn is_local(&self) -> bool {
        Settings::get().hostname().eq(&self.domain)
    }
    pub fn full_name(&self) -> String {
        format!("@{}@{}", &self.name, &self.domain)
    }
}

pub fn scrape_text_for_mentions(text: &str) -> Vec<MentionData> {
    let mut out: Vec<MentionData> = Vec::new();
    for caps in MENTIONS_REGEX.captures_iter(text) {
        out.push(MentionData {
            name: caps["name"].to_string(),
            domain: caps["domain"].to_string(),
        });
    }
    out.into_iter().unique().collect()
}

pub fn is_valid_username(name: &str) -> bool {
    VALID_USERNAME_REGEX.is_match(name)
}

// Can't do a regex here, reverse lookarounds not supported
pub fn is_valid_display_name(display_name: &str) -> bool {
    !display_name.starts_with('@')
        && display_name.chars().count() >= 3
        && display_name.chars().count() <= 200
}

pub fn is_valid_school_name(name: &str) -> bool {
    VALID_SCHOOL_NAME_REGEX.is_match(name)
}

pub fn is_valid_category_title(title: &str) -> bool {
    VALID_CATEGORY_TITLE_REGEX.is_match(title)
}

pub fn is_valid_course_title(title: &str) -> bool {
    VALID_COURSE_TITLE_REGEX.is_match(title)
}

pub fn get_ip(conn_info: &ConnectionInfo) -> IpAddr {
    IpAddr(
        conn_info
            .realip_remote_addr()
            .unwrap_or("localhost:3003")
            .split(':')
            .next()
            .unwrap_or("localhost")
            .to_string(),
    )
}
pub fn system_time() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}
