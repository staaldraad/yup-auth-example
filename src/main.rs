use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator, ServiceAccountImpersonationAuthenticatorS};

// pub struct AuthorizedUserSecret {
//     pub client_id: String,
//     pub client_secret: String,
//     pub refresh_token: String,
//     pub key_type: String,
// }

#[tokio::main]
async fn main() {
    let svc_email = std::env::args().nth(1).unwrap();
    //let home = std::env::var("HOME").unwrap();

    let creds = read_service_account_key(".config/service-token.json")
    .await
    .expect("user secret");

    let sa = ServiceAccountAuthenticator::builder(creds.clone())
            .build()
            .await
            .unwrap();
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];

    let tok = sa.token(scopes).await.unwrap();
    println!("token is: {:?}", tok);

    let auth = ServiceAccountImpersonationAuthenticatorS::builder(creds.clone(), &svc_email)
        .await
        .build()
        .await
        .unwrap();


    let scopes = &["https://www.googleapis.com/auth/cloud-platform", "https://www.googleapis.com/auth/bigquery"];
    match auth.token(scopes).await {
        Err(e) => println!("error: {:?}", e),
        Ok(t) => println!("token: {:?}", t),
    }

    // // If you configure the authenticator to request id tokens, it will give back id tokens
    // // instead of access tokens.
    // let auth = ServiceAccountImpersonationAuthenticator::builder(user_secret, &svc_email)
    //     .request_id_token()
    //     .build()
    //     .await
    //     .expect("authenticator");

    // let scopes = &["https://www.googleapis.com/auth/youtube.readonly"];
    // match auth.token(scopes).await {
    //     Err(e) => println!("error: {:?}", e),
    //     Ok(t) => println!("token: {:?}", t),
    // }
}
