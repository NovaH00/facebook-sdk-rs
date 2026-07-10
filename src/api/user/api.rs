
use crate::graph::{
    GraphClient,
    Method,
    GraphError,
    UserGraphClient
};
use crate::auth::LongLivedUserToken;
use crate::api::page::PageApi;

use super::models::User;


#[derive(Debug, Clone)]
pub struct UserApi {
    user_graph_client: UserGraphClient
}

impl UserApi {
    pub fn new(user_access_token: LongLivedUserToken) -> Self {
        Self {
            user_graph_client: GraphClient::new(user_access_token)
        }
    }

    pub async fn me(&self) -> Result<User, GraphError> {
        self.user_graph_client
            .request(Method::GET, "/me")
            .fields(User::fields())
            .send::<User>()
            .await
    }

    pub fn get_page_api(&self) -> PageApi {
        PageApi::new(&self.user_graph_client)
    }
}
