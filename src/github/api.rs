use octocrab::Octocrab;

pub fn client(token: &str) -> Octocrab {
    Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .expect("octocrab build")
}
