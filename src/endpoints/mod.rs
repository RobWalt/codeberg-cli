pub mod endpoint_generator;

pub const CODEBERG_API_BASE: &str = "https://codeberg.org/api/v1/";

pub const AUTHENTIFICATION_VERIFICATION: &str = "user";

pub const USER_INFO: &str = "user";
pub const USER_FOLLOWERS: &str = "user/followers";
pub const USER_FOLLOWING: &str = "user/following";
pub const USER_REPOS: &str = "user/repos";

// repos/{}/{}/issues
pub const ISSUE_LIST_START: &str = "repos";
pub const ISSUE_LIST_END: &str = "issues";
