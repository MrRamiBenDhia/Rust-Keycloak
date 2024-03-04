extern crate core;
extern crate keycloak;

use keycloak::{
    types::*,
    {KeycloakAdmin, KeycloakAdminToken},
};
// pub extern crate reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("KEYCLOAK_ADDR").unwrap_or_else(|_| "http://localhost:8080".into());
    let user = std::env::var("KEYCLOAK_USER").unwrap_or_else(|_| "admin".into());
    let password = std::env::var("KEYCLOAK_PASSWORD").unwrap_or_else(|_| "admin".into());
    let client = reqwest::Client::new();
    let admin_token = KeycloakAdminToken::acquire(&url, &user, &password, &client).await?;

    eprintln!("{:?}", admin_token);

    let admin = KeycloakAdmin::new(&url, admin_token, client);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    // admin
    //     .post(RealmRepresentation {
    //         realm: Some("sandbox".into()),
    //         ..Default::default()
    //     })
    //     .await?;
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    admin
        .realm_users_post(
            "sandbox",
            UserRepresentation {
                username: Some("Ibrahim".into()),
                ..Default::default()
            },
        )
        .await?;
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    let users = admin
        .realm_users_get(
            "sandbox", None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        )
        .await?;

    eprintln!("{:?}", users);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    // let id = users
    //     .iter()
    //     .find(|u| u.username == Some("user".into()))
    //     .unwrap()
    //     .id
    //     .as_ref()
    //     .unwrap()
    //     .to_string();

    // admin
    //     .realm_users_with_id_delete("sandbox", id.as_str())
    //     .await?;

    // admin.realm_delete("sandbox").await?;

    Ok(())
}
