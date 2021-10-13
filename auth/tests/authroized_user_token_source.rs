use gcpauth::*;
use std::fs::File;
use std::io::Write;
use gcpauth::token_source::token_source::TokenSource;

#[tokio::test]
async fn test() -> Result<(), error::Error> {
    let authorized_user_credentials = std::env::var("TEST_USER_CREDENTIALS")
        .map_err(error::Error::VarError)?;

    let json = base64::decode(authorized_user_credentials).unwrap();
    let mut file = File::create(".cred.json")?;
    file.write_all(json.as_slice())?;

    std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", ".cred.json");
    let credentials = credentials::CredentialsFile::new().await?;
    let ts = token_source::authorized_user_token_source::UserAccountTokenSource::new(&credentials)?;
    let token = ts.token().await?;
    assert_eq!("Bearer", token.token_type);
    assert_eq!(true, token.expiry.unwrap().timestamp() > 0);
    Ok(())
}