use std::io;

use facebook_sdk_rs::auth::{
    AppClient,
    AppPermission,
    LongLivedUserToken,
    PageToken
};
use facebook_sdk_rs::graph::{
    GraphClient,
    Method,
    Fields,
    QueryParams
};
use facebook_sdk_rs::api::{
    user::{UserApi, User},
    page::{PageApi, Page},
    post::PostOperations,
    message::MessagingType,
    webhook::{WebhookField}
};


async fn get_user_access_token() {
    let app_client = AppClient::new(
        "4337245509845486",
        "b926b029e5ca523463946d665aae7a81",
        "http://localhost:8080/api/v1/auth/facebook/callback"
    );

    let url = app_client.get_oauth_url(
        "test",
        &[
            AppPermission::PublicProfile,
            AppPermission::PagesShowList,
            AppPermission::PagesMessaging,
        ],
        None
    ).unwrap();


    println!("{}", url);

    let mut code = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line");

    let code = code.trim();

    let long_lived_token = app_client.login(code).await.unwrap();
    println!("Long lived token: {:?}", long_lived_token.as_str());

}

async fn testing() {
    let user_token = LongLivedUserToken::new("EAA9os6nrAe4BR7mH0ZCJdXyoJ3lS8iCWuVEHzW3KVhQKRsR59rrTsKjXpodRVa8kw5uDM41BkmbGXpyrpQoiVyFuI9QwTl0jwUZBX4L9nGQJZBB78i9KJ7zdN5PeMTs29I2RDIYhZALDTFhh0q4mhqIo3opOfgnMzzZB0700EAuhnGOZAEv6BaxOk3AzfL");
    let user_client = GraphClient::new(user_token);
    let user_api = UserApi::new(user_client);

    let page_api = user_api.get_page_api();
    let pages = page_api.collect_paginated_pages(Some(20)).await.unwrap();
    println!("{:#?}", pages);
    let first_page = pages.first().unwrap();

    let convo_api = page_api.get_conversation_api(first_page).unwrap();
    let convos = convo_api.collect_paginated_conversations(Some(20)).await.unwrap();
    let first_convo = convos.first().unwrap();

    let message_api = convo_api.get_message_api(first_convo).unwrap();

    let response = message_api.send_message("what's good yo!", MessagingType::Response).await.unwrap();
    println!("{:#?}", response);


}

#[tokio::main]
async fn main() {
    // get_user_access_token().await;
    testing().await;
}
