#[derive(Clone, Debug)]
///Client for Verge.io API
///
///API to interact with the Verge.io Cloud API
///
///http://verge.io/terms/
///
///Version: 4.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "4.0"
    }
}
impl Client {
    ///Sends a `GET` request to `/permissions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.permissions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn permissions_get(&self) -> builder::PermissionsGet {
        builder::PermissionsGet::new(self)
    }
    ///Sends a `POST` request to `/permissions`
    ///
    ///Arguments:
    /// - `body`: permissions body object
    ///```ignore
    /// let response = client.permissions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn permissions_post(&self) -> builder::PermissionsPost {
        builder::PermissionsPost::new(self)
    }
    ///Sends a `GET` request to `/permissions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.permissions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn permissions_get_by_id(&self) -> builder::PermissionsGetById {
        builder::PermissionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/permissions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: permissions body object
    ///```ignore
    /// let response = client.permissions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn permissions_put_by_id(&self) -> builder::PermissionsPutById {
        builder::PermissionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/permissions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.permissions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn permissions_delete_by_id(&self) -> builder::PermissionsDeleteById {
        builder::PermissionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/auth_source_states`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.auth_source_states_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_source_states_get(&self) -> builder::AuthSourceStatesGet {
        builder::AuthSourceStatesGet::new(self)
    }
    ///Sends a `POST` request to `/auth_source_states`
    ///
    ///Arguments:
    /// - `body`: auth_source_states body object
    ///```ignore
    /// let response = client.auth_source_states_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_source_states_post(&self) -> builder::AuthSourceStatesPost {
        builder::AuthSourceStatesPost::new(self)
    }
    ///Sends a `GET` request to `/auth_source_states/{state}`
    ///
    ///Arguments:
    /// - `state`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.auth_source_states_get_by_state()
    ///    .state(state)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_source_states_get_by_state(
        &self,
    ) -> builder::AuthSourceStatesGetByState {
        builder::AuthSourceStatesGetByState::new(self)
    }
    ///Sends a `PUT` request to `/auth_source_states/{state}`
    ///
    ///Arguments:
    /// - `state`: resource id
    /// - `body`: auth_source_states body object
    ///```ignore
    /// let response = client.auth_source_states_put_by_state()
    ///    .state(state)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_source_states_put_by_state(
        &self,
    ) -> builder::AuthSourceStatesPutByState {
        builder::AuthSourceStatesPutByState::new(self)
    }
    ///Sends a `DELETE` request to `/auth_source_states/{state}`
    ///
    ///Arguments:
    /// - `state`: resource id
    ///```ignore
    /// let response = client.auth_source_states_delete_by_state()
    ///    .state(state)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_source_states_delete_by_state(
        &self,
    ) -> builder::AuthSourceStatesDeleteByState {
        builder::AuthSourceStatesDeleteByState::new(self)
    }
    ///Sends a `GET` request to `/auth_sources`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.auth_sources_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_sources_get(&self) -> builder::AuthSourcesGet {
        builder::AuthSourcesGet::new(self)
    }
    ///Sends a `POST` request to `/auth_sources`
    ///
    ///Arguments:
    /// - `body`: auth_sources body object
    ///```ignore
    /// let response = client.auth_sources_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_sources_post(&self) -> builder::AuthSourcesPost {
        builder::AuthSourcesPost::new(self)
    }
    ///Sends a `GET` request to `/auth_sources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.auth_sources_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_sources_get_by_id(&self) -> builder::AuthSourcesGetById {
        builder::AuthSourcesGetById::new(self)
    }
    ///Sends a `PUT` request to `/auth_sources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: auth_sources body object
    ///```ignore
    /// let response = client.auth_sources_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_sources_put_by_id(&self) -> builder::AuthSourcesPutById {
        builder::AuthSourcesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/auth_sources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.auth_sources_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn auth_sources_delete_by_id(&self) -> builder::AuthSourcesDeleteById {
        builder::AuthSourcesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/billing`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.billing_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_get(&self) -> builder::BillingGet {
        builder::BillingGet::new(self)
    }
    ///Sends a `POST` request to `/billing`
    ///
    ///Arguments:
    /// - `body`: billing body object
    ///```ignore
    /// let response = client.billing_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_post(&self) -> builder::BillingPost {
        builder::BillingPost::new(self)
    }
    ///Sends a `GET` request to `/billing/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.billing_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_get_by_id(&self) -> builder::BillingGetById {
        builder::BillingGetById::new(self)
    }
    ///Sends a `PUT` request to `/billing/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: billing body object
    ///```ignore
    /// let response = client.billing_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_put_by_id(&self) -> builder::BillingPutById {
        builder::BillingPutById::new(self)
    }
    ///Sends a `DELETE` request to `/billing/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.billing_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_delete_by_id(&self) -> builder::BillingDeleteById {
        builder::BillingDeleteById::new(self)
    }
    ///Sends a `GET` request to `/billing_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.billing_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_actions_get(&self) -> builder::BillingActionsGet {
        builder::BillingActionsGet::new(self)
    }
    ///Sends a `POST` request to `/billing_actions`
    ///
    ///Arguments:
    /// - `body`: billing_actions body object
    ///```ignore
    /// let response = client.billing_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_actions_post(&self) -> builder::BillingActionsPost {
        builder::BillingActionsPost::new(self)
    }
    ///Sends a `GET` request to `/billing_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.billing_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_actions_get_by_id(&self) -> builder::BillingActionsGetById {
        builder::BillingActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/billing_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: billing_actions body object
    ///```ignore
    /// let response = client.billing_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_actions_put_by_id(&self) -> builder::BillingActionsPutById {
        builder::BillingActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/billing_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.billing_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn billing_actions_delete_by_id(&self) -> builder::BillingActionsDeleteById {
        builder::BillingActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/catalog_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.catalog_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_logs_get(&self) -> builder::CatalogLogsGet {
        builder::CatalogLogsGet::new(self)
    }
    ///Sends a `POST` request to `/catalog_logs`
    ///
    ///Arguments:
    /// - `body`: catalog_logs body object
    ///```ignore
    /// let response = client.catalog_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_logs_post(&self) -> builder::CatalogLogsPost {
        builder::CatalogLogsPost::new(self)
    }
    ///Sends a `GET` request to `/catalog_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.catalog_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_logs_get_by_id(&self) -> builder::CatalogLogsGetById {
        builder::CatalogLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/catalog_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: catalog_logs body object
    ///```ignore
    /// let response = client.catalog_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_logs_put_by_id(&self) -> builder::CatalogLogsPutById {
        builder::CatalogLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/catalog_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.catalog_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_logs_delete_by_id(&self) -> builder::CatalogLogsDeleteById {
        builder::CatalogLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/catalog_repositories`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.catalog_repositories_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repositories_get(&self) -> builder::CatalogRepositoriesGet {
        builder::CatalogRepositoriesGet::new(self)
    }
    ///Sends a `POST` request to `/catalog_repositories`
    ///
    ///Arguments:
    /// - `body`: catalog_repositories body object
    ///```ignore
    /// let response = client.catalog_repositories_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repositories_post(&self) -> builder::CatalogRepositoriesPost {
        builder::CatalogRepositoriesPost::new(self)
    }
    ///Sends a `GET` request to `/catalog_repositories/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.catalog_repositories_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repositories_get_by_id(&self) -> builder::CatalogRepositoriesGetById {
        builder::CatalogRepositoriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/catalog_repositories/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: catalog_repositories body object
    ///```ignore
    /// let response = client.catalog_repositories_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repositories_put_by_id(&self) -> builder::CatalogRepositoriesPutById {
        builder::CatalogRepositoriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/catalog_repositories/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.catalog_repositories_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repositories_delete_by_id(
        &self,
    ) -> builder::CatalogRepositoriesDeleteById {
        builder::CatalogRepositoriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/catalog_repository_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.catalog_repository_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_actions_get(
        &self,
    ) -> builder::CatalogRepositoryActionsGet {
        builder::CatalogRepositoryActionsGet::new(self)
    }
    ///Sends a `POST` request to `/catalog_repository_actions`
    ///
    ///Arguments:
    /// - `body`: catalog_repository_actions body object
    ///```ignore
    /// let response = client.catalog_repository_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_actions_post(
        &self,
    ) -> builder::CatalogRepositoryActionsPost {
        builder::CatalogRepositoryActionsPost::new(self)
    }
    ///Sends a `GET` request to `/catalog_repository_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.catalog_repository_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_actions_get_by_id(
        &self,
    ) -> builder::CatalogRepositoryActionsGetById {
        builder::CatalogRepositoryActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/catalog_repository_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: catalog_repository_actions body object
    ///```ignore
    /// let response = client.catalog_repository_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_actions_put_by_id(
        &self,
    ) -> builder::CatalogRepositoryActionsPutById {
        builder::CatalogRepositoryActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/catalog_repository_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.catalog_repository_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_actions_delete_by_id(
        &self,
    ) -> builder::CatalogRepositoryActionsDeleteById {
        builder::CatalogRepositoryActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/catalog_repository_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.catalog_repository_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_logs_get(&self) -> builder::CatalogRepositoryLogsGet {
        builder::CatalogRepositoryLogsGet::new(self)
    }
    ///Sends a `POST` request to `/catalog_repository_logs`
    ///
    ///Arguments:
    /// - `body`: catalog_repository_logs body object
    ///```ignore
    /// let response = client.catalog_repository_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_logs_post(&self) -> builder::CatalogRepositoryLogsPost {
        builder::CatalogRepositoryLogsPost::new(self)
    }
    ///Sends a `GET` request to `/catalog_repository_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.catalog_repository_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_logs_get_by_id(
        &self,
    ) -> builder::CatalogRepositoryLogsGetById {
        builder::CatalogRepositoryLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/catalog_repository_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: catalog_repository_logs body object
    ///```ignore
    /// let response = client.catalog_repository_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_logs_put_by_id(
        &self,
    ) -> builder::CatalogRepositoryLogsPutById {
        builder::CatalogRepositoryLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/catalog_repository_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.catalog_repository_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_logs_delete_by_id(
        &self,
    ) -> builder::CatalogRepositoryLogsDeleteById {
        builder::CatalogRepositoryLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/catalog_repository_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.catalog_repository_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_status_get(&self) -> builder::CatalogRepositoryStatusGet {
        builder::CatalogRepositoryStatusGet::new(self)
    }
    ///Sends a `POST` request to `/catalog_repository_status`
    ///
    ///Arguments:
    /// - `body`: catalog_repository_status body object
    ///```ignore
    /// let response = client.catalog_repository_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_status_post(
        &self,
    ) -> builder::CatalogRepositoryStatusPost {
        builder::CatalogRepositoryStatusPost::new(self)
    }
    ///Sends a `GET` request to `/catalog_repository_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.catalog_repository_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_status_get_by_id(
        &self,
    ) -> builder::CatalogRepositoryStatusGetById {
        builder::CatalogRepositoryStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/catalog_repository_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: catalog_repository_status body object
    ///```ignore
    /// let response = client.catalog_repository_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_status_put_by_id(
        &self,
    ) -> builder::CatalogRepositoryStatusPutById {
        builder::CatalogRepositoryStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/catalog_repository_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.catalog_repository_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalog_repository_status_delete_by_id(
        &self,
    ) -> builder::CatalogRepositoryStatusDeleteById {
        builder::CatalogRepositoryStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/catalogs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.catalogs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalogs_get(&self) -> builder::CatalogsGet {
        builder::CatalogsGet::new(self)
    }
    ///Sends a `POST` request to `/catalogs`
    ///
    ///Arguments:
    /// - `body`: catalogs body object
    ///```ignore
    /// let response = client.catalogs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalogs_post(&self) -> builder::CatalogsPost {
        builder::CatalogsPost::new(self)
    }
    ///Sends a `GET` request to `/catalogs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.catalogs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalogs_get_by_id(&self) -> builder::CatalogsGetById {
        builder::CatalogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/catalogs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: catalogs body object
    ///```ignore
    /// let response = client.catalogs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalogs_put_by_id(&self) -> builder::CatalogsPutById {
        builder::CatalogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/catalogs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.catalogs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn catalogs_delete_by_id(&self) -> builder::CatalogsDeleteById {
        builder::CatalogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/certificates`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.certificates_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn certificates_get(&self) -> builder::CertificatesGet {
        builder::CertificatesGet::new(self)
    }
    ///Sends a `POST` request to `/certificates`
    ///
    ///Arguments:
    /// - `body`: certificates body object
    ///```ignore
    /// let response = client.certificates_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn certificates_post(&self) -> builder::CertificatesPost {
        builder::CertificatesPost::new(self)
    }
    ///Sends a `GET` request to `/certificates/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.certificates_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn certificates_get_by_id(&self) -> builder::CertificatesGetById {
        builder::CertificatesGetById::new(self)
    }
    ///Sends a `PUT` request to `/certificates/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: certificates body object
    ///```ignore
    /// let response = client.certificates_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn certificates_put_by_id(&self) -> builder::CertificatesPutById {
        builder::CertificatesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/certificates/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.certificates_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn certificates_delete_by_id(&self) -> builder::CertificatesDeleteById {
        builder::CertificatesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/clone_iso`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.clone_iso_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_get(&self) -> builder::CloneIsoGet {
        builder::CloneIsoGet::new(self)
    }
    ///Sends a `POST` request to `/clone_iso`
    ///
    ///Arguments:
    /// - `body`: clone_iso body object
    ///```ignore
    /// let response = client.clone_iso_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_post(&self) -> builder::CloneIsoPost {
        builder::CloneIsoPost::new(self)
    }
    ///Sends a `GET` request to `/clone_iso/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.clone_iso_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_get_by_id(&self) -> builder::CloneIsoGetById {
        builder::CloneIsoGetById::new(self)
    }
    ///Sends a `PUT` request to `/clone_iso/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: clone_iso body object
    ///```ignore
    /// let response = client.clone_iso_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_put_by_id(&self) -> builder::CloneIsoPutById {
        builder::CloneIsoPutById::new(self)
    }
    ///Sends a `DELETE` request to `/clone_iso/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.clone_iso_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_delete_by_id(&self) -> builder::CloneIsoDeleteById {
        builder::CloneIsoDeleteById::new(self)
    }
    ///Sends a `GET` request to `/clone_iso_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.clone_iso_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_actions_get(&self) -> builder::CloneIsoActionsGet {
        builder::CloneIsoActionsGet::new(self)
    }
    ///Sends a `POST` request to `/clone_iso_actions`
    ///
    ///Arguments:
    /// - `body`: clone_iso_actions body object
    ///```ignore
    /// let response = client.clone_iso_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_actions_post(&self) -> builder::CloneIsoActionsPost {
        builder::CloneIsoActionsPost::new(self)
    }
    ///Sends a `GET` request to `/clone_iso_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.clone_iso_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_actions_get_by_id(&self) -> builder::CloneIsoActionsGetById {
        builder::CloneIsoActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/clone_iso_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: clone_iso_actions body object
    ///```ignore
    /// let response = client.clone_iso_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_actions_put_by_id(&self) -> builder::CloneIsoActionsPutById {
        builder::CloneIsoActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/clone_iso_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.clone_iso_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clone_iso_actions_delete_by_id(&self) -> builder::CloneIsoActionsDeleteById {
        builder::CloneIsoActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_restore`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_restore_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_restore_get(&self) -> builder::CloudRestoreGet {
        builder::CloudRestoreGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_restore`
    ///
    ///Arguments:
    /// - `body`: cloud_restore body object
    ///```ignore
    /// let response = client.cloud_restore_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_restore_post(&self) -> builder::CloudRestorePost {
        builder::CloudRestorePost::new(self)
    }
    ///Sends a `GET` request to `/cloud_restore/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_restore_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_restore_get_by_id(&self) -> builder::CloudRestoreGetById {
        builder::CloudRestoreGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_restore/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_restore body object
    ///```ignore
    /// let response = client.cloud_restore_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_restore_put_by_id(&self) -> builder::CloudRestorePutById {
        builder::CloudRestorePutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_restore/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_restore_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_restore_delete_by_id(&self) -> builder::CloudRestoreDeleteById {
        builder::CloudRestoreDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_snapshot_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_actions_get(&self) -> builder::CloudSnapshotActionsGet {
        builder::CloudSnapshotActionsGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_snapshot_actions`
    ///
    ///Arguments:
    /// - `body`: cloud_snapshot_actions body object
    ///```ignore
    /// let response = client.cloud_snapshot_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_actions_post(&self) -> builder::CloudSnapshotActionsPost {
        builder::CloudSnapshotActionsPost::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_snapshot_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_actions_get_by_id(
        &self,
    ) -> builder::CloudSnapshotActionsGetById {
        builder::CloudSnapshotActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_snapshot_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_snapshot_actions body object
    ///```ignore
    /// let response = client.cloud_snapshot_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_actions_put_by_id(
        &self,
    ) -> builder::CloudSnapshotActionsPutById {
        builder::CloudSnapshotActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_snapshot_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_snapshot_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_actions_delete_by_id(
        &self,
    ) -> builder::CloudSnapshotActionsDeleteById {
        builder::CloudSnapshotActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_tenant_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_snapshot_tenant_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenant_actions_get(
        &self,
    ) -> builder::CloudSnapshotTenantActionsGet {
        builder::CloudSnapshotTenantActionsGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_snapshot_tenant_actions`
    ///
    ///Arguments:
    /// - `body`: cloud_snapshot_tenant_actions body object
    ///```ignore
    /// let response = client.cloud_snapshot_tenant_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenant_actions_post(
        &self,
    ) -> builder::CloudSnapshotTenantActionsPost {
        builder::CloudSnapshotTenantActionsPost::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_tenant_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_snapshot_tenant_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenant_actions_get_by_id(
        &self,
    ) -> builder::CloudSnapshotTenantActionsGetById {
        builder::CloudSnapshotTenantActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_snapshot_tenant_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_snapshot_tenant_actions body object
    ///```ignore
    /// let response = client.cloud_snapshot_tenant_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenant_actions_put_by_id(
        &self,
    ) -> builder::CloudSnapshotTenantActionsPutById {
        builder::CloudSnapshotTenantActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_snapshot_tenant_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_snapshot_tenant_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenant_actions_delete_by_id(
        &self,
    ) -> builder::CloudSnapshotTenantActionsDeleteById {
        builder::CloudSnapshotTenantActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_tenants`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_snapshot_tenants_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenants_get(&self) -> builder::CloudSnapshotTenantsGet {
        builder::CloudSnapshotTenantsGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_snapshot_tenants`
    ///
    ///Arguments:
    /// - `body`: cloud_snapshot_tenants body object
    ///```ignore
    /// let response = client.cloud_snapshot_tenants_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenants_post(&self) -> builder::CloudSnapshotTenantsPost {
        builder::CloudSnapshotTenantsPost::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_snapshot_tenants_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenants_get_by_id(
        &self,
    ) -> builder::CloudSnapshotTenantsGetById {
        builder::CloudSnapshotTenantsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_snapshot_tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_snapshot_tenants body object
    ///```ignore
    /// let response = client.cloud_snapshot_tenants_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenants_put_by_id(
        &self,
    ) -> builder::CloudSnapshotTenantsPutById {
        builder::CloudSnapshotTenantsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_snapshot_tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_snapshot_tenants_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_tenants_delete_by_id(
        &self,
    ) -> builder::CloudSnapshotTenantsDeleteById {
        builder::CloudSnapshotTenantsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_vm_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_snapshot_vm_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vm_actions_get(&self) -> builder::CloudSnapshotVmActionsGet {
        builder::CloudSnapshotVmActionsGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_snapshot_vm_actions`
    ///
    ///Arguments:
    /// - `body`: cloud_snapshot_vm_actions body object
    ///```ignore
    /// let response = client.cloud_snapshot_vm_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vm_actions_post(&self) -> builder::CloudSnapshotVmActionsPost {
        builder::CloudSnapshotVmActionsPost::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_vm_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_snapshot_vm_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vm_actions_get_by_id(
        &self,
    ) -> builder::CloudSnapshotVmActionsGetById {
        builder::CloudSnapshotVmActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_snapshot_vm_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_snapshot_vm_actions body object
    ///```ignore
    /// let response = client.cloud_snapshot_vm_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vm_actions_put_by_id(
        &self,
    ) -> builder::CloudSnapshotVmActionsPutById {
        builder::CloudSnapshotVmActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_snapshot_vm_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_snapshot_vm_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vm_actions_delete_by_id(
        &self,
    ) -> builder::CloudSnapshotVmActionsDeleteById {
        builder::CloudSnapshotVmActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_vms`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_snapshot_vms_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vms_get(&self) -> builder::CloudSnapshotVmsGet {
        builder::CloudSnapshotVmsGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_snapshot_vms`
    ///
    ///Arguments:
    /// - `body`: cloud_snapshot_vms body object
    ///```ignore
    /// let response = client.cloud_snapshot_vms_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vms_post(&self) -> builder::CloudSnapshotVmsPost {
        builder::CloudSnapshotVmsPost::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshot_vms/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_snapshot_vms_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vms_get_by_id(&self) -> builder::CloudSnapshotVmsGetById {
        builder::CloudSnapshotVmsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_snapshot_vms/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_snapshot_vms body object
    ///```ignore
    /// let response = client.cloud_snapshot_vms_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vms_put_by_id(&self) -> builder::CloudSnapshotVmsPutById {
        builder::CloudSnapshotVmsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_snapshot_vms/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_snapshot_vms_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshot_vms_delete_by_id(
        &self,
    ) -> builder::CloudSnapshotVmsDeleteById {
        builder::CloudSnapshotVmsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshots`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloud_snapshots_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshots_get(&self) -> builder::CloudSnapshotsGet {
        builder::CloudSnapshotsGet::new(self)
    }
    ///Sends a `POST` request to `/cloud_snapshots`
    ///
    ///Arguments:
    /// - `body`: cloud_snapshots body object
    ///```ignore
    /// let response = client.cloud_snapshots_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshots_post(&self) -> builder::CloudSnapshotsPost {
        builder::CloudSnapshotsPost::new(self)
    }
    ///Sends a `GET` request to `/cloud_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloud_snapshots_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshots_get_by_id(&self) -> builder::CloudSnapshotsGetById {
        builder::CloudSnapshotsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloud_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloud_snapshots body object
    ///```ignore
    /// let response = client.cloud_snapshots_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshots_put_by_id(&self) -> builder::CloudSnapshotsPutById {
        builder::CloudSnapshotsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloud_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloud_snapshots_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloud_snapshots_delete_by_id(&self) -> builder::CloudSnapshotsDeleteById {
        builder::CloudSnapshotsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cloudinit_files`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cloudinit_files_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloudinit_files_get(&self) -> builder::CloudinitFilesGet {
        builder::CloudinitFilesGet::new(self)
    }
    ///Sends a `POST` request to `/cloudinit_files`
    ///
    ///Arguments:
    /// - `body`: cloudinit_files body object
    ///```ignore
    /// let response = client.cloudinit_files_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloudinit_files_post(&self) -> builder::CloudinitFilesPost {
        builder::CloudinitFilesPost::new(self)
    }
    ///Sends a `GET` request to `/cloudinit_files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cloudinit_files_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloudinit_files_get_by_id(&self) -> builder::CloudinitFilesGetById {
        builder::CloudinitFilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/cloudinit_files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cloudinit_files body object
    ///```ignore
    /// let response = client.cloudinit_files_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloudinit_files_put_by_id(&self) -> builder::CloudinitFilesPutById {
        builder::CloudinitFilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cloudinit_files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cloudinit_files_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cloudinit_files_delete_by_id(&self) -> builder::CloudinitFilesDeleteById {
        builder::CloudinitFilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_actions_get(&self) -> builder::ClusterActionsGet {
        builder::ClusterActionsGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_actions`
    ///
    ///Arguments:
    /// - `body`: cluster_actions body object
    ///```ignore
    /// let response = client.cluster_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_actions_post(&self) -> builder::ClusterActionsPost {
        builder::ClusterActionsPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_actions_get_by_id(&self) -> builder::ClusterActionsGetById {
        builder::ClusterActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_actions body object
    ///```ignore
    /// let response = client.cluster_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_actions_put_by_id(&self) -> builder::ClusterActionsPutById {
        builder::ClusterActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_actions_delete_by_id(&self) -> builder::ClusterActionsDeleteById {
        builder::ClusterActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_long_get(&self) -> builder::ClusterStatsHistoryLongGet {
        builder::ClusterStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: cluster_stats_history_long body object
    ///```ignore
    /// let response = client.cluster_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_long_post(
        &self,
    ) -> builder::ClusterStatsHistoryLongPost {
        builder::ClusterStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_long_get_by_id(
        &self,
    ) -> builder::ClusterStatsHistoryLongGetById {
        builder::ClusterStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_stats_history_long body object
    ///```ignore
    /// let response = client.cluster_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_long_put_by_id(
        &self,
    ) -> builder::ClusterStatsHistoryLongPutById {
        builder::ClusterStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_long_delete_by_id(
        &self,
    ) -> builder::ClusterStatsHistoryLongDeleteById {
        builder::ClusterStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_short_get(
        &self,
    ) -> builder::ClusterStatsHistoryShortGet {
        builder::ClusterStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: cluster_stats_history_short body object
    ///```ignore
    /// let response = client.cluster_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_short_post(
        &self,
    ) -> builder::ClusterStatsHistoryShortPost {
        builder::ClusterStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_short_get_by_id(
        &self,
    ) -> builder::ClusterStatsHistoryShortGetById {
        builder::ClusterStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_stats_history_short body object
    ///```ignore
    /// let response = client.cluster_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_short_put_by_id(
        &self,
    ) -> builder::ClusterStatsHistoryShortPutById {
        builder::ClusterStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_stats_history_short_delete_by_id(
        &self,
    ) -> builder::ClusterStatsHistoryShortDeleteById {
        builder::ClusterStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_status_get(&self) -> builder::ClusterStatusGet {
        builder::ClusterStatusGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_status`
    ///
    ///Arguments:
    /// - `body`: cluster_status body object
    ///```ignore
    /// let response = client.cluster_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_status_post(&self) -> builder::ClusterStatusPost {
        builder::ClusterStatusPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_status_get_by_id(&self) -> builder::ClusterStatusGetById {
        builder::ClusterStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_status body object
    ///```ignore
    /// let response = client.cluster_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_status_put_by_id(&self) -> builder::ClusterStatusPutById {
        builder::ClusterStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_status_delete_by_id(&self) -> builder::ClusterStatusDeleteById {
        builder::ClusterStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_tier_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_get(&self) -> builder::ClusterTierStatsGet {
        builder::ClusterTierStatsGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_tier_stats`
    ///
    ///Arguments:
    /// - `body`: cluster_tier_stats body object
    ///```ignore
    /// let response = client.cluster_tier_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_post(&self) -> builder::ClusterTierStatsPost {
        builder::ClusterTierStatsPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_tier_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_get_by_id(&self) -> builder::ClusterTierStatsGetById {
        builder::ClusterTierStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_tier_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_tier_stats body object
    ///```ignore
    /// let response = client.cluster_tier_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_put_by_id(&self) -> builder::ClusterTierStatsPutById {
        builder::ClusterTierStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_tier_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_tier_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_delete_by_id(
        &self,
    ) -> builder::ClusterTierStatsDeleteById {
        builder::ClusterTierStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_tier_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_long_get(
        &self,
    ) -> builder::ClusterTierStatsHistoryLongGet {
        builder::ClusterTierStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_tier_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: cluster_tier_stats_history_long body object
    ///```ignore
    /// let response = client.cluster_tier_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_long_post(
        &self,
    ) -> builder::ClusterTierStatsHistoryLongPost {
        builder::ClusterTierStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_tier_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_long_get_by_id(
        &self,
    ) -> builder::ClusterTierStatsHistoryLongGetById {
        builder::ClusterTierStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_tier_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_tier_stats_history_long body object
    ///```ignore
    /// let response = client.cluster_tier_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_long_put_by_id(
        &self,
    ) -> builder::ClusterTierStatsHistoryLongPutById {
        builder::ClusterTierStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_tier_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_tier_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_long_delete_by_id(
        &self,
    ) -> builder::ClusterTierStatsHistoryLongDeleteById {
        builder::ClusterTierStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_tier_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_short_get(
        &self,
    ) -> builder::ClusterTierStatsHistoryShortGet {
        builder::ClusterTierStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_tier_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: cluster_tier_stats_history_short body object
    ///```ignore
    /// let response = client.cluster_tier_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_short_post(
        &self,
    ) -> builder::ClusterTierStatsHistoryShortPost {
        builder::ClusterTierStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_tier_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_short_get_by_id(
        &self,
    ) -> builder::ClusterTierStatsHistoryShortGetById {
        builder::ClusterTierStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_tier_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_tier_stats_history_short body object
    ///```ignore
    /// let response = client.cluster_tier_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_short_put_by_id(
        &self,
    ) -> builder::ClusterTierStatsHistoryShortPutById {
        builder::ClusterTierStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_tier_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_tier_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_stats_history_short_delete_by_id(
        &self,
    ) -> builder::ClusterTierStatsHistoryShortDeleteById {
        builder::ClusterTierStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_tier_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_status_get(&self) -> builder::ClusterTierStatusGet {
        builder::ClusterTierStatusGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_tier_status`
    ///
    ///Arguments:
    /// - `body`: cluster_tier_status body object
    ///```ignore
    /// let response = client.cluster_tier_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_status_post(&self) -> builder::ClusterTierStatusPost {
        builder::ClusterTierStatusPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_tier_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_tier_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_status_get_by_id(&self) -> builder::ClusterTierStatusGetById {
        builder::ClusterTierStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_tier_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_tier_status body object
    ///```ignore
    /// let response = client.cluster_tier_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_status_put_by_id(&self) -> builder::ClusterTierStatusPutById {
        builder::ClusterTierStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_tier_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_tier_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tier_status_delete_by_id(
        &self,
    ) -> builder::ClusterTierStatusDeleteById {
        builder::ClusterTierStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/cluster_tiers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.cluster_tiers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tiers_get(&self) -> builder::ClusterTiersGet {
        builder::ClusterTiersGet::new(self)
    }
    ///Sends a `POST` request to `/cluster_tiers`
    ///
    ///Arguments:
    /// - `body`: cluster_tiers body object
    ///```ignore
    /// let response = client.cluster_tiers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tiers_post(&self) -> builder::ClusterTiersPost {
        builder::ClusterTiersPost::new(self)
    }
    ///Sends a `GET` request to `/cluster_tiers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.cluster_tiers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tiers_get_by_id(&self) -> builder::ClusterTiersGetById {
        builder::ClusterTiersGetById::new(self)
    }
    ///Sends a `PUT` request to `/cluster_tiers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: cluster_tiers body object
    ///```ignore
    /// let response = client.cluster_tiers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tiers_put_by_id(&self) -> builder::ClusterTiersPutById {
        builder::ClusterTiersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/cluster_tiers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.cluster_tiers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cluster_tiers_delete_by_id(&self) -> builder::ClusterTiersDeleteById {
        builder::ClusterTiersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/clusters`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.clusters_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clusters_get(&self) -> builder::ClustersGet {
        builder::ClustersGet::new(self)
    }
    ///Sends a `POST` request to `/clusters`
    ///
    ///Arguments:
    /// - `body`: clusters body object
    ///```ignore
    /// let response = client.clusters_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clusters_post(&self) -> builder::ClustersPost {
        builder::ClustersPost::new(self)
    }
    ///Sends a `GET` request to `/clusters/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.clusters_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clusters_get_by_id(&self) -> builder::ClustersGetById {
        builder::ClustersGetById::new(self)
    }
    ///Sends a `PUT` request to `/clusters/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: clusters body object
    ///```ignore
    /// let response = client.clusters_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clusters_put_by_id(&self) -> builder::ClustersPutById {
        builder::ClustersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/clusters/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.clusters_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn clusters_delete_by_id(&self) -> builder::ClustersDeleteById {
        builder::ClustersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/file_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.file_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn file_actions_get(&self) -> builder::FileActionsGet {
        builder::FileActionsGet::new(self)
    }
    ///Sends a `POST` request to `/file_actions`
    ///
    ///Arguments:
    /// - `body`: file_actions body object
    ///```ignore
    /// let response = client.file_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn file_actions_post(&self) -> builder::FileActionsPost {
        builder::FileActionsPost::new(self)
    }
    ///Sends a `GET` request to `/file_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.file_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn file_actions_get_by_id(&self) -> builder::FileActionsGetById {
        builder::FileActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/file_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: file_actions body object
    ///```ignore
    /// let response = client.file_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn file_actions_put_by_id(&self) -> builder::FileActionsPutById {
        builder::FileActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/file_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.file_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn file_actions_delete_by_id(&self) -> builder::FileActionsDeleteById {
        builder::FileActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/files`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.files_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_get(&self) -> builder::FilesGet {
        builder::FilesGet::new(self)
    }
    ///Sends a `POST` request to `/files`
    ///
    ///Arguments:
    /// - `body`: files body object
    ///```ignore
    /// let response = client.files_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_post(&self) -> builder::FilesPost {
        builder::FilesPost::new(self)
    }
    ///Sends a `GET` request to `/files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.files_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_get_by_id(&self) -> builder::FilesGetById {
        builder::FilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: files body object
    ///```ignore
    /// let response = client.files_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_put_by_id(&self) -> builder::FilesPutById {
        builder::FilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.files_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_delete_by_id(&self) -> builder::FilesDeleteById {
        builder::FilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/files_public_links`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.files_public_links_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_public_links_get(&self) -> builder::FilesPublicLinksGet {
        builder::FilesPublicLinksGet::new(self)
    }
    ///Sends a `POST` request to `/files_public_links`
    ///
    ///Arguments:
    /// - `body`: files_public_links body object
    ///```ignore
    /// let response = client.files_public_links_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_public_links_post(&self) -> builder::FilesPublicLinksPost {
        builder::FilesPublicLinksPost::new(self)
    }
    ///Sends a `GET` request to `/files_public_links/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.files_public_links_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_public_links_get_by_id(&self) -> builder::FilesPublicLinksGetById {
        builder::FilesPublicLinksGetById::new(self)
    }
    ///Sends a `PUT` request to `/files_public_links/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: files_public_links body object
    ///```ignore
    /// let response = client.files_public_links_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_public_links_put_by_id(&self) -> builder::FilesPublicLinksPutById {
        builder::FilesPublicLinksPutById::new(self)
    }
    ///Sends a `DELETE` request to `/files_public_links/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.files_public_links_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn files_public_links_delete_by_id(
        &self,
    ) -> builder::FilesPublicLinksDeleteById {
        builder::FilesPublicLinksDeleteById::new(self)
    }
    ///Sends a `GET` request to `/group_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.group_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn group_logs_get(&self) -> builder::GroupLogsGet {
        builder::GroupLogsGet::new(self)
    }
    ///Sends a `POST` request to `/group_logs`
    ///
    ///Arguments:
    /// - `body`: group_logs body object
    ///```ignore
    /// let response = client.group_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn group_logs_post(&self) -> builder::GroupLogsPost {
        builder::GroupLogsPost::new(self)
    }
    ///Sends a `GET` request to `/group_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.group_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn group_logs_get_by_id(&self) -> builder::GroupLogsGetById {
        builder::GroupLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/group_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: group_logs body object
    ///```ignore
    /// let response = client.group_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn group_logs_put_by_id(&self) -> builder::GroupLogsPutById {
        builder::GroupLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/group_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.group_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn group_logs_delete_by_id(&self) -> builder::GroupLogsDeleteById {
        builder::GroupLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/groups`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.groups_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn groups_get(&self) -> builder::GroupsGet {
        builder::GroupsGet::new(self)
    }
    ///Sends a `POST` request to `/groups`
    ///
    ///Arguments:
    /// - `body`: groups body object
    ///```ignore
    /// let response = client.groups_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn groups_post(&self) -> builder::GroupsPost {
        builder::GroupsPost::new(self)
    }
    ///Sends a `GET` request to `/groups/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.groups_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn groups_get_by_id(&self) -> builder::GroupsGetById {
        builder::GroupsGetById::new(self)
    }
    ///Sends a `PUT` request to `/groups/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: groups body object
    ///```ignore
    /// let response = client.groups_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn groups_put_by_id(&self) -> builder::GroupsPutById {
        builder::GroupsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/groups/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.groups_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn groups_delete_by_id(&self) -> builder::GroupsDeleteById {
        builder::GroupsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/help_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.help_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_actions_get(&self) -> builder::HelpActionsGet {
        builder::HelpActionsGet::new(self)
    }
    ///Sends a `POST` request to `/help_actions`
    ///
    ///Arguments:
    /// - `body`: help_actions body object
    ///```ignore
    /// let response = client.help_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_actions_post(&self) -> builder::HelpActionsPost {
        builder::HelpActionsPost::new(self)
    }
    ///Sends a `GET` request to `/help_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.help_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_actions_get_by_id(&self) -> builder::HelpActionsGetById {
        builder::HelpActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/help_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: help_actions body object
    ///```ignore
    /// let response = client.help_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_actions_put_by_id(&self) -> builder::HelpActionsPutById {
        builder::HelpActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/help_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.help_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_actions_delete_by_id(&self) -> builder::HelpActionsDeleteById {
        builder::HelpActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/help_search`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.help_search_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_search_get(&self) -> builder::HelpSearchGet {
        builder::HelpSearchGet::new(self)
    }
    ///Sends a `POST` request to `/help_search`
    ///
    ///Arguments:
    /// - `body`: help_search body object
    ///```ignore
    /// let response = client.help_search_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_search_post(&self) -> builder::HelpSearchPost {
        builder::HelpSearchPost::new(self)
    }
    ///Sends a `GET` request to `/help_search/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.help_search_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_search_get_by_id(&self) -> builder::HelpSearchGetById {
        builder::HelpSearchGetById::new(self)
    }
    ///Sends a `PUT` request to `/help_search/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: help_search body object
    ///```ignore
    /// let response = client.help_search_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_search_put_by_id(&self) -> builder::HelpSearchPutById {
        builder::HelpSearchPutById::new(self)
    }
    ///Sends a `DELETE` request to `/help_search/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.help_search_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn help_search_delete_by_id(&self) -> builder::HelpSearchDeleteById {
        builder::HelpSearchDeleteById::new(self)
    }
    ///Sends a `GET` request to `/license_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.license_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn license_actions_get(&self) -> builder::LicenseActionsGet {
        builder::LicenseActionsGet::new(self)
    }
    ///Sends a `POST` request to `/license_actions`
    ///
    ///Arguments:
    /// - `body`: license_actions body object
    ///```ignore
    /// let response = client.license_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn license_actions_post(&self) -> builder::LicenseActionsPost {
        builder::LicenseActionsPost::new(self)
    }
    ///Sends a `GET` request to `/license_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.license_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn license_actions_get_by_id(&self) -> builder::LicenseActionsGetById {
        builder::LicenseActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/license_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: license_actions body object
    ///```ignore
    /// let response = client.license_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn license_actions_put_by_id(&self) -> builder::LicenseActionsPutById {
        builder::LicenseActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/license_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.license_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn license_actions_delete_by_id(&self) -> builder::LicenseActionsDeleteById {
        builder::LicenseActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/licenses`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.licenses_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn licenses_get(&self) -> builder::LicensesGet {
        builder::LicensesGet::new(self)
    }
    ///Sends a `POST` request to `/licenses`
    ///
    ///Arguments:
    /// - `body`: licenses body object
    ///```ignore
    /// let response = client.licenses_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn licenses_post(&self) -> builder::LicensesPost {
        builder::LicensesPost::new(self)
    }
    ///Sends a `GET` request to `/licenses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.licenses_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn licenses_get_by_id(&self) -> builder::LicensesGetById {
        builder::LicensesGetById::new(self)
    }
    ///Sends a `PUT` request to `/licenses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: licenses body object
    ///```ignore
    /// let response = client.licenses_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn licenses_put_by_id(&self) -> builder::LicensesPutById {
        builder::LicensesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/licenses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.licenses_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn licenses_delete_by_id(&self) -> builder::LicensesDeleteById {
        builder::LicensesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn logs_get(&self) -> builder::LogsGet {
        builder::LogsGet::new(self)
    }
    ///Sends a `POST` request to `/logs`
    ///
    ///Arguments:
    /// - `body`: logs body object
    ///```ignore
    /// let response = client.logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn logs_post(&self) -> builder::LogsPost {
        builder::LogsPost::new(self)
    }
    ///Sends a `GET` request to `/logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn logs_get_by_id(&self) -> builder::LogsGetById {
        builder::LogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: logs body object
    ///```ignore
    /// let response = client.logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn logs_put_by_id(&self) -> builder::LogsPutById {
        builder::LogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn logs_delete_by_id(&self) -> builder::LogsDeleteById {
        builder::LogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_console`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_console_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_get(&self) -> builder::MachineConsoleGet {
        builder::MachineConsoleGet::new(self)
    }
    ///Sends a `POST` request to `/machine_console`
    ///
    ///Arguments:
    /// - `body`: machine_console body object
    ///```ignore
    /// let response = client.machine_console_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_post(&self) -> builder::MachineConsolePost {
        builder::MachineConsolePost::new(self)
    }
    ///Sends a `GET` request to `/machine_console/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_console_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_get_by_id(&self) -> builder::MachineConsoleGetById {
        builder::MachineConsoleGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_console/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_console body object
    ///```ignore
    /// let response = client.machine_console_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_put_by_id(&self) -> builder::MachineConsolePutById {
        builder::MachineConsolePutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_console/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_console_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_delete_by_id(&self) -> builder::MachineConsoleDeleteById {
        builder::MachineConsoleDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_console_active`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_console_active_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_get(&self) -> builder::MachineConsoleActiveGet {
        builder::MachineConsoleActiveGet::new(self)
    }
    ///Sends a `POST` request to `/machine_console_active`
    ///
    ///Arguments:
    /// - `body`: machine_console_active body object
    ///```ignore
    /// let response = client.machine_console_active_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_post(&self) -> builder::MachineConsoleActivePost {
        builder::MachineConsoleActivePost::new(self)
    }
    ///Sends a `GET` request to `/machine_console_active/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_console_active_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_get_by_id(
        &self,
    ) -> builder::MachineConsoleActiveGetById {
        builder::MachineConsoleActiveGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_console_active/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_console_active body object
    ///```ignore
    /// let response = client.machine_console_active_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_put_by_id(
        &self,
    ) -> builder::MachineConsoleActivePutById {
        builder::MachineConsoleActivePutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_console_active/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_console_active_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_delete_by_id(
        &self,
    ) -> builder::MachineConsoleActiveDeleteById {
        builder::MachineConsoleActiveDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_drives`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_drives_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drives_get(&self) -> builder::MachineDrivesGet {
        builder::MachineDrivesGet::new(self)
    }
    ///Sends a `POST` request to `/machine_drives`
    ///
    ///Arguments:
    /// - `body`: machine_drives body object
    ///```ignore
    /// let response = client.machine_drives_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drives_post(&self) -> builder::MachineDrivesPost {
        builder::MachineDrivesPost::new(self)
    }
    ///Sends a `GET` request to `/machine_drives/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_drives_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drives_get_by_id(&self) -> builder::MachineDrivesGetById {
        builder::MachineDrivesGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_drives/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_drives body object
    ///```ignore
    /// let response = client.machine_drives_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drives_put_by_id(&self) -> builder::MachineDrivesPutById {
        builder::MachineDrivesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_drives/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_drives_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drives_delete_by_id(&self) -> builder::MachineDrivesDeleteById {
        builder::MachineDrivesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_console_active_chat`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_console_active_chat_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_chat_get(
        &self,
    ) -> builder::MachineConsoleActiveChatGet {
        builder::MachineConsoleActiveChatGet::new(self)
    }
    ///Sends a `POST` request to `/machine_console_active_chat`
    ///
    ///Arguments:
    /// - `body`: machine_console_active_chat body object
    ///```ignore
    /// let response = client.machine_console_active_chat_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_chat_post(
        &self,
    ) -> builder::MachineConsoleActiveChatPost {
        builder::MachineConsoleActiveChatPost::new(self)
    }
    ///Sends a `GET` request to `/machine_console_active_chat/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_console_active_chat_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_chat_get_by_id(
        &self,
    ) -> builder::MachineConsoleActiveChatGetById {
        builder::MachineConsoleActiveChatGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_console_active_chat/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_console_active_chat body object
    ///```ignore
    /// let response = client.machine_console_active_chat_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_chat_put_by_id(
        &self,
    ) -> builder::MachineConsoleActiveChatPutById {
        builder::MachineConsoleActiveChatPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_console_active_chat/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_console_active_chat_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_console_active_chat_delete_by_id(
        &self,
    ) -> builder::MachineConsoleActiveChatDeleteById {
        builder::MachineConsoleActiveChatDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_gpu_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_long_get(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryLongGet {
        builder::MachineDeviceGpuStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_gpu_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: machine_device_gpu_stats_history_long body object
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_long_post(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryLongPost {
        builder::MachineDeviceGpuStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_gpu_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_long_get_by_id(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryLongGetById {
        builder::MachineDeviceGpuStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_gpu_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_gpu_stats_history_long body object
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_long_put_by_id(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryLongPutById {
        builder::MachineDeviceGpuStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to
    /// `/machine_device_gpu_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_long_delete_by_id(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryLongDeleteById {
        builder::MachineDeviceGpuStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_gpu_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_short_get(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryShortGet {
        builder::MachineDeviceGpuStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_gpu_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: machine_device_gpu_stats_history_short body object
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_short_post(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryShortPost {
        builder::MachineDeviceGpuStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_gpu_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_short_get_by_id(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryShortGetById {
        builder::MachineDeviceGpuStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_gpu_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_gpu_stats_history_short body object
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_short_put_by_id(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryShortPutById {
        builder::MachineDeviceGpuStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to
    /// `/machine_device_gpu_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_gpu_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_gpu_stats_history_short_delete_by_id(
        &self,
    ) -> builder::MachineDeviceGpuStatsHistoryShortDeleteById {
        builder::MachineDeviceGpuStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_gpu`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_settings_gpu_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_gpu_get(
        &self,
    ) -> builder::MachineDeviceSettingsGpuGet {
        builder::MachineDeviceSettingsGpuGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_settings_gpu`
    ///
    ///Arguments:
    /// - `body`: machine_device_settings_gpu body object
    ///```ignore
    /// let response = client.machine_device_settings_gpu_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_gpu_post(
        &self,
    ) -> builder::MachineDeviceSettingsGpuPost {
        builder::MachineDeviceSettingsGpuPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_gpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_settings_gpu_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_gpu_get_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsGpuGetById {
        builder::MachineDeviceSettingsGpuGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_settings_gpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_settings_gpu body object
    ///```ignore
    /// let response = client.machine_device_settings_gpu_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_gpu_put_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsGpuPutById {
        builder::MachineDeviceSettingsGpuPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_settings_gpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_settings_gpu_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_gpu_delete_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsGpuDeleteById {
        builder::MachineDeviceSettingsGpuDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_nvidia_vgpu`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_settings_nvidia_vgpu_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_nvidia_vgpu_get(
        &self,
    ) -> builder::MachineDeviceSettingsNvidiaVgpuGet {
        builder::MachineDeviceSettingsNvidiaVgpuGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_settings_nvidia_vgpu`
    ///
    ///Arguments:
    /// - `body`: machine_device_settings_nvidia_vgpu body object
    ///```ignore
    /// let response = client.machine_device_settings_nvidia_vgpu_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_nvidia_vgpu_post(
        &self,
    ) -> builder::MachineDeviceSettingsNvidiaVgpuPost {
        builder::MachineDeviceSettingsNvidiaVgpuPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_nvidia_vgpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_settings_nvidia_vgpu_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_nvidia_vgpu_get_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsNvidiaVgpuGetById {
        builder::MachineDeviceSettingsNvidiaVgpuGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_settings_nvidia_vgpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_settings_nvidia_vgpu body object
    ///```ignore
    /// let response = client.machine_device_settings_nvidia_vgpu_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_nvidia_vgpu_put_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsNvidiaVgpuPutById {
        builder::MachineDeviceSettingsNvidiaVgpuPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_settings_nvidia_vgpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_settings_nvidia_vgpu_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_nvidia_vgpu_delete_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsNvidiaVgpuDeleteById {
        builder::MachineDeviceSettingsNvidiaVgpuDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_sriov_nic`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_settings_sriov_nic_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_sriov_nic_get(
        &self,
    ) -> builder::MachineDeviceSettingsSriovNicGet {
        builder::MachineDeviceSettingsSriovNicGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_settings_sriov_nic`
    ///
    ///Arguments:
    /// - `body`: machine_device_settings_sriov_nic body object
    ///```ignore
    /// let response = client.machine_device_settings_sriov_nic_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_sriov_nic_post(
        &self,
    ) -> builder::MachineDeviceSettingsSriovNicPost {
        builder::MachineDeviceSettingsSriovNicPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_sriov_nic/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_settings_sriov_nic_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_sriov_nic_get_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsSriovNicGetById {
        builder::MachineDeviceSettingsSriovNicGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_settings_sriov_nic/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_settings_sriov_nic body object
    ///```ignore
    /// let response = client.machine_device_settings_sriov_nic_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_sriov_nic_put_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsSriovNicPutById {
        builder::MachineDeviceSettingsSriovNicPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_settings_sriov_nic/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_settings_sriov_nic_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_sriov_nic_delete_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsSriovNicDeleteById {
        builder::MachineDeviceSettingsSriovNicDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_tpm`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_settings_tpm_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_tpm_get(
        &self,
    ) -> builder::MachineDeviceSettingsTpmGet {
        builder::MachineDeviceSettingsTpmGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_settings_tpm`
    ///
    ///Arguments:
    /// - `body`: machine_device_settings_tpm body object
    ///```ignore
    /// let response = client.machine_device_settings_tpm_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_tpm_post(
        &self,
    ) -> builder::MachineDeviceSettingsTpmPost {
        builder::MachineDeviceSettingsTpmPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_tpm/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_settings_tpm_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_tpm_get_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsTpmGetById {
        builder::MachineDeviceSettingsTpmGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_settings_tpm/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_settings_tpm body object
    ///```ignore
    /// let response = client.machine_device_settings_tpm_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_tpm_put_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsTpmPutById {
        builder::MachineDeviceSettingsTpmPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_settings_tpm/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_settings_tpm_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_tpm_delete_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsTpmDeleteById {
        builder::MachineDeviceSettingsTpmDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_usb`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_settings_usb_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_usb_get(
        &self,
    ) -> builder::MachineDeviceSettingsUsbGet {
        builder::MachineDeviceSettingsUsbGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_settings_usb`
    ///
    ///Arguments:
    /// - `body`: machine_device_settings_usb body object
    ///```ignore
    /// let response = client.machine_device_settings_usb_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_usb_post(
        &self,
    ) -> builder::MachineDeviceSettingsUsbPost {
        builder::MachineDeviceSettingsUsbPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_settings_usb/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_settings_usb_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_usb_get_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsUsbGetById {
        builder::MachineDeviceSettingsUsbGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_settings_usb/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_settings_usb body object
    ///```ignore
    /// let response = client.machine_device_settings_usb_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_usb_put_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsUsbPutById {
        builder::MachineDeviceSettingsUsbPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_settings_usb/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_settings_usb_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_settings_usb_delete_by_id(
        &self,
    ) -> builder::MachineDeviceSettingsUsbDeleteById {
        builder::MachineDeviceSettingsUsbDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_stats_get(&self) -> builder::MachineDeviceStatsGet {
        builder::MachineDeviceStatsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_stats`
    ///
    ///Arguments:
    /// - `body`: machine_device_stats body object
    ///```ignore
    /// let response = client.machine_device_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_stats_post(&self) -> builder::MachineDeviceStatsPost {
        builder::MachineDeviceStatsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_stats_get_by_id(&self) -> builder::MachineDeviceStatsGetById {
        builder::MachineDeviceStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_stats body object
    ///```ignore
    /// let response = client.machine_device_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_stats_put_by_id(&self) -> builder::MachineDeviceStatsPutById {
        builder::MachineDeviceStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_stats_delete_by_id(
        &self,
    ) -> builder::MachineDeviceStatsDeleteById {
        builder::MachineDeviceStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_device_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_device_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_status_get(&self) -> builder::MachineDeviceStatusGet {
        builder::MachineDeviceStatusGet::new(self)
    }
    ///Sends a `POST` request to `/machine_device_status`
    ///
    ///Arguments:
    /// - `body`: machine_device_status body object
    ///```ignore
    /// let response = client.machine_device_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_status_post(&self) -> builder::MachineDeviceStatusPost {
        builder::MachineDeviceStatusPost::new(self)
    }
    ///Sends a `GET` request to `/machine_device_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_device_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_status_get_by_id(
        &self,
    ) -> builder::MachineDeviceStatusGetById {
        builder::MachineDeviceStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_device_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_device_status body object
    ///```ignore
    /// let response = client.machine_device_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_status_put_by_id(
        &self,
    ) -> builder::MachineDeviceStatusPutById {
        builder::MachineDeviceStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_device_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_device_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_device_status_delete_by_id(
        &self,
    ) -> builder::MachineDeviceStatusDeleteById {
        builder::MachineDeviceStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_devices`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_devices_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_devices_get(&self) -> builder::MachineDevicesGet {
        builder::MachineDevicesGet::new(self)
    }
    ///Sends a `POST` request to `/machine_devices`
    ///
    ///Arguments:
    /// - `body`: machine_devices body object
    ///```ignore
    /// let response = client.machine_devices_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_devices_post(&self) -> builder::MachineDevicesPost {
        builder::MachineDevicesPost::new(self)
    }
    ///Sends a `GET` request to `/machine_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_devices_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_devices_get_by_id(&self) -> builder::MachineDevicesGetById {
        builder::MachineDevicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_devices body object
    ///```ignore
    /// let response = client.machine_devices_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_devices_put_by_id(&self) -> builder::MachineDevicesPutById {
        builder::MachineDevicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_devices_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_devices_delete_by_id(&self) -> builder::MachineDevicesDeleteById {
        builder::MachineDevicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_phys`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_drive_phys_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_phys_get(&self) -> builder::MachineDrivePhysGet {
        builder::MachineDrivePhysGet::new(self)
    }
    ///Sends a `POST` request to `/machine_drive_phys`
    ///
    ///Arguments:
    /// - `body`: machine_drive_phys body object
    ///```ignore
    /// let response = client.machine_drive_phys_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_phys_post(&self) -> builder::MachineDrivePhysPost {
        builder::MachineDrivePhysPost::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_phys/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_drive_phys_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_phys_get_by_id(&self) -> builder::MachineDrivePhysGetById {
        builder::MachineDrivePhysGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_drive_phys/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_drive_phys body object
    ///```ignore
    /// let response = client.machine_drive_phys_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_phys_put_by_id(&self) -> builder::MachineDrivePhysPutById {
        builder::MachineDrivePhysPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_drive_phys/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_drive_phys_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_phys_delete_by_id(
        &self,
    ) -> builder::MachineDrivePhysDeleteById {
        builder::MachineDrivePhysDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_drive_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_get(&self) -> builder::MachineDriveStatsGet {
        builder::MachineDriveStatsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_drive_stats`
    ///
    ///Arguments:
    /// - `body`: machine_drive_stats body object
    ///```ignore
    /// let response = client.machine_drive_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_post(&self) -> builder::MachineDriveStatsPost {
        builder::MachineDriveStatsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_drive_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_get_by_id(&self) -> builder::MachineDriveStatsGetById {
        builder::MachineDriveStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_drive_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_drive_stats body object
    ///```ignore
    /// let response = client.machine_drive_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_put_by_id(&self) -> builder::MachineDriveStatsPutById {
        builder::MachineDriveStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_drive_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_drive_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_delete_by_id(
        &self,
    ) -> builder::MachineDriveStatsDeleteById {
        builder::MachineDriveStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_drive_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_long_get(
        &self,
    ) -> builder::MachineDriveStatsHistoryLongGet {
        builder::MachineDriveStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/machine_drive_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: machine_drive_stats_history_long body object
    ///```ignore
    /// let response = client.machine_drive_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_long_post(
        &self,
    ) -> builder::MachineDriveStatsHistoryLongPost {
        builder::MachineDriveStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_drive_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_long_get_by_id(
        &self,
    ) -> builder::MachineDriveStatsHistoryLongGetById {
        builder::MachineDriveStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_drive_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_drive_stats_history_long body object
    ///```ignore
    /// let response = client.machine_drive_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_long_put_by_id(
        &self,
    ) -> builder::MachineDriveStatsHistoryLongPutById {
        builder::MachineDriveStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_drive_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_drive_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_long_delete_by_id(
        &self,
    ) -> builder::MachineDriveStatsHistoryLongDeleteById {
        builder::MachineDriveStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_drive_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_short_get(
        &self,
    ) -> builder::MachineDriveStatsHistoryShortGet {
        builder::MachineDriveStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/machine_drive_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: machine_drive_stats_history_short body object
    ///```ignore
    /// let response = client.machine_drive_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_short_post(
        &self,
    ) -> builder::MachineDriveStatsHistoryShortPost {
        builder::MachineDriveStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_drive_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_short_get_by_id(
        &self,
    ) -> builder::MachineDriveStatsHistoryShortGetById {
        builder::MachineDriveStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_drive_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_drive_stats_history_short body object
    ///```ignore
    /// let response = client.machine_drive_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_short_put_by_id(
        &self,
    ) -> builder::MachineDriveStatsHistoryShortPutById {
        builder::MachineDriveStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_drive_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_drive_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_stats_history_short_delete_by_id(
        &self,
    ) -> builder::MachineDriveStatsHistoryShortDeleteById {
        builder::MachineDriveStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_drive_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_status_get(&self) -> builder::MachineDriveStatusGet {
        builder::MachineDriveStatusGet::new(self)
    }
    ///Sends a `POST` request to `/machine_drive_status`
    ///
    ///Arguments:
    /// - `body`: machine_drive_status body object
    ///```ignore
    /// let response = client.machine_drive_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_status_post(&self) -> builder::MachineDriveStatusPost {
        builder::MachineDriveStatusPost::new(self)
    }
    ///Sends a `GET` request to `/machine_drive_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_drive_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_status_get_by_id(&self) -> builder::MachineDriveStatusGetById {
        builder::MachineDriveStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_drive_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_drive_status body object
    ///```ignore
    /// let response = client.machine_drive_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_status_put_by_id(&self) -> builder::MachineDriveStatusPutById {
        builder::MachineDriveStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_drive_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_drive_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_drive_status_delete_by_id(
        &self,
    ) -> builder::MachineDriveStatusDeleteById {
        builder::MachineDriveStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machines`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machines_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_get(&self) -> builder::MachinesGet {
        builder::MachinesGet::new(self)
    }
    ///Sends a `POST` request to `/machines`
    ///
    ///Arguments:
    /// - `body`: machines body object
    ///```ignore
    /// let response = client.machines_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_post(&self) -> builder::MachinesPost {
        builder::MachinesPost::new(self)
    }
    ///Sends a `GET` request to `/machines/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machines_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_get_by_id(&self) -> builder::MachinesGetById {
        builder::MachinesGetById::new(self)
    }
    ///Sends a `PUT` request to `/machines/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machines body object
    ///```ignore
    /// let response = client.machines_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_put_by_id(&self) -> builder::MachinesPutById {
        builder::MachinesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machines/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machines_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machines_delete_by_id(&self) -> builder::MachinesDeleteById {
        builder::MachinesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_logs_get(&self) -> builder::MachineLogsGet {
        builder::MachineLogsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_logs`
    ///
    ///Arguments:
    /// - `body`: machine_logs body object
    ///```ignore
    /// let response = client.machine_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_logs_post(&self) -> builder::MachineLogsPost {
        builder::MachineLogsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_logs_get_by_id(&self) -> builder::MachineLogsGetById {
        builder::MachineLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_logs body object
    ///```ignore
    /// let response = client.machine_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_logs_put_by_id(&self) -> builder::MachineLogsPutById {
        builder::MachineLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_logs_delete_by_id(&self) -> builder::MachineLogsDeleteById {
        builder::MachineLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_nic_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_get(&self) -> builder::MachineNicStatsGet {
        builder::MachineNicStatsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_nic_stats`
    ///
    ///Arguments:
    /// - `body`: machine_nic_stats body object
    ///```ignore
    /// let response = client.machine_nic_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_post(&self) -> builder::MachineNicStatsPost {
        builder::MachineNicStatsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_nic_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_get_by_id(&self) -> builder::MachineNicStatsGetById {
        builder::MachineNicStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_nic_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_nic_stats body object
    ///```ignore
    /// let response = client.machine_nic_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_put_by_id(&self) -> builder::MachineNicStatsPutById {
        builder::MachineNicStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_nic_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_nic_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_delete_by_id(&self) -> builder::MachineNicStatsDeleteById {
        builder::MachineNicStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_nic_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_long_get(
        &self,
    ) -> builder::MachineNicStatsHistoryLongGet {
        builder::MachineNicStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/machine_nic_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: machine_nic_stats_history_long body object
    ///```ignore
    /// let response = client.machine_nic_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_long_post(
        &self,
    ) -> builder::MachineNicStatsHistoryLongPost {
        builder::MachineNicStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_nic_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_long_get_by_id(
        &self,
    ) -> builder::MachineNicStatsHistoryLongGetById {
        builder::MachineNicStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_nic_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_nic_stats_history_long body object
    ///```ignore
    /// let response = client.machine_nic_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_long_put_by_id(
        &self,
    ) -> builder::MachineNicStatsHistoryLongPutById {
        builder::MachineNicStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_nic_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_nic_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_long_delete_by_id(
        &self,
    ) -> builder::MachineNicStatsHistoryLongDeleteById {
        builder::MachineNicStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_nic_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_short_get(
        &self,
    ) -> builder::MachineNicStatsHistoryShortGet {
        builder::MachineNicStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/machine_nic_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: machine_nic_stats_history_short body object
    ///```ignore
    /// let response = client.machine_nic_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_short_post(
        &self,
    ) -> builder::MachineNicStatsHistoryShortPost {
        builder::MachineNicStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_nic_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_short_get_by_id(
        &self,
    ) -> builder::MachineNicStatsHistoryShortGetById {
        builder::MachineNicStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_nic_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_nic_stats_history_short body object
    ///```ignore
    /// let response = client.machine_nic_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_short_put_by_id(
        &self,
    ) -> builder::MachineNicStatsHistoryShortPutById {
        builder::MachineNicStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_nic_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_nic_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_stats_history_short_delete_by_id(
        &self,
    ) -> builder::MachineNicStatsHistoryShortDeleteById {
        builder::MachineNicStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_nic_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_status_get(&self) -> builder::MachineNicStatusGet {
        builder::MachineNicStatusGet::new(self)
    }
    ///Sends a `POST` request to `/machine_nic_status`
    ///
    ///Arguments:
    /// - `body`: machine_nic_status body object
    ///```ignore
    /// let response = client.machine_nic_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_status_post(&self) -> builder::MachineNicStatusPost {
        builder::MachineNicStatusPost::new(self)
    }
    ///Sends a `GET` request to `/machine_nic_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_nic_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_status_get_by_id(&self) -> builder::MachineNicStatusGetById {
        builder::MachineNicStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_nic_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_nic_status body object
    ///```ignore
    /// let response = client.machine_nic_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_status_put_by_id(&self) -> builder::MachineNicStatusPutById {
        builder::MachineNicStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_nic_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_nic_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nic_status_delete_by_id(
        &self,
    ) -> builder::MachineNicStatusDeleteById {
        builder::MachineNicStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_nics`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_nics_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nics_get(&self) -> builder::MachineNicsGet {
        builder::MachineNicsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_nics`
    ///
    ///Arguments:
    /// - `body`: machine_nics body object
    ///```ignore
    /// let response = client.machine_nics_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nics_post(&self) -> builder::MachineNicsPost {
        builder::MachineNicsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_nics/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_nics_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nics_get_by_id(&self) -> builder::MachineNicsGetById {
        builder::MachineNicsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_nics/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_nics body object
    ///```ignore
    /// let response = client.machine_nics_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nics_put_by_id(&self) -> builder::MachineNicsPutById {
        builder::MachineNicsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_nics/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_nics_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_nics_delete_by_id(&self) -> builder::MachineNicsDeleteById {
        builder::MachineNicsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_snapshots`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_snapshots_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_snapshots_get(&self) -> builder::MachineSnapshotsGet {
        builder::MachineSnapshotsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_snapshots`
    ///
    ///Arguments:
    /// - `body`: machine_snapshots body object
    ///```ignore
    /// let response = client.machine_snapshots_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_snapshots_post(&self) -> builder::MachineSnapshotsPost {
        builder::MachineSnapshotsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_snapshots_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_snapshots_get_by_id(&self) -> builder::MachineSnapshotsGetById {
        builder::MachineSnapshotsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_snapshots body object
    ///```ignore
    /// let response = client.machine_snapshots_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_snapshots_put_by_id(&self) -> builder::MachineSnapshotsPutById {
        builder::MachineSnapshotsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_snapshots_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_snapshots_delete_by_id(&self) -> builder::MachineSnapshotsDeleteById {
        builder::MachineSnapshotsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_get(&self) -> builder::MachineStatsGet {
        builder::MachineStatsGet::new(self)
    }
    ///Sends a `POST` request to `/machine_stats`
    ///
    ///Arguments:
    /// - `body`: machine_stats body object
    ///```ignore
    /// let response = client.machine_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_post(&self) -> builder::MachineStatsPost {
        builder::MachineStatsPost::new(self)
    }
    ///Sends a `GET` request to `/machine_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_get_by_id(&self) -> builder::MachineStatsGetById {
        builder::MachineStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_stats body object
    ///```ignore
    /// let response = client.machine_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_put_by_id(&self) -> builder::MachineStatsPutById {
        builder::MachineStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_delete_by_id(&self) -> builder::MachineStatsDeleteById {
        builder::MachineStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_long_get(&self) -> builder::MachineStatsHistoryLongGet {
        builder::MachineStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/machine_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: machine_stats_history_long body object
    ///```ignore
    /// let response = client.machine_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_long_post(
        &self,
    ) -> builder::MachineStatsHistoryLongPost {
        builder::MachineStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/machine_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_long_get_by_id(
        &self,
    ) -> builder::MachineStatsHistoryLongGetById {
        builder::MachineStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_stats_history_long body object
    ///```ignore
    /// let response = client.machine_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_long_put_by_id(
        &self,
    ) -> builder::MachineStatsHistoryLongPutById {
        builder::MachineStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_long_delete_by_id(
        &self,
    ) -> builder::MachineStatsHistoryLongDeleteById {
        builder::MachineStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_short_get(
        &self,
    ) -> builder::MachineStatsHistoryShortGet {
        builder::MachineStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/machine_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: machine_stats_history_short body object
    ///```ignore
    /// let response = client.machine_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_short_post(
        &self,
    ) -> builder::MachineStatsHistoryShortPost {
        builder::MachineStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/machine_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_short_get_by_id(
        &self,
    ) -> builder::MachineStatsHistoryShortGetById {
        builder::MachineStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_stats_history_short body object
    ///```ignore
    /// let response = client.machine_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_short_put_by_id(
        &self,
    ) -> builder::MachineStatsHistoryShortPutById {
        builder::MachineStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_stats_history_short_delete_by_id(
        &self,
    ) -> builder::MachineStatsHistoryShortDeleteById {
        builder::MachineStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/machine_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.machine_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_status_get(&self) -> builder::MachineStatusGet {
        builder::MachineStatusGet::new(self)
    }
    ///Sends a `POST` request to `/machine_status`
    ///
    ///Arguments:
    /// - `body`: machine_status body object
    ///```ignore
    /// let response = client.machine_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_status_post(&self) -> builder::MachineStatusPost {
        builder::MachineStatusPost::new(self)
    }
    ///Sends a `GET` request to `/machine_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.machine_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_status_get_by_id(&self) -> builder::MachineStatusGetById {
        builder::MachineStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/machine_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: machine_status body object
    ///```ignore
    /// let response = client.machine_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_status_put_by_id(&self) -> builder::MachineStatusPutById {
        builder::MachineStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/machine_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.machine_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn machine_status_delete_by_id(&self) -> builder::MachineStatusDeleteById {
        builder::MachineStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/sites`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.sites_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn sites_get(&self) -> builder::SitesGet {
        builder::SitesGet::new(self)
    }
    ///Sends a `POST` request to `/sites`
    ///
    ///Arguments:
    /// - `body`: sites body object
    ///```ignore
    /// let response = client.sites_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn sites_post(&self) -> builder::SitesPost {
        builder::SitesPost::new(self)
    }
    ///Sends a `GET` request to `/sites/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.sites_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn sites_get_by_id(&self) -> builder::SitesGetById {
        builder::SitesGetById::new(self)
    }
    ///Sends a `PUT` request to `/sites/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: sites body object
    ///```ignore
    /// let response = client.sites_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn sites_put_by_id(&self) -> builder::SitesPutById {
        builder::SitesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/sites/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.sites_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn sites_delete_by_id(&self) -> builder::SitesDeleteById {
        builder::SitesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/members`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.members_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn members_get(&self) -> builder::MembersGet {
        builder::MembersGet::new(self)
    }
    ///Sends a `POST` request to `/members`
    ///
    ///Arguments:
    /// - `body`: members body object
    ///```ignore
    /// let response = client.members_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn members_post(&self) -> builder::MembersPost {
        builder::MembersPost::new(self)
    }
    ///Sends a `GET` request to `/members/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.members_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn members_get_by_id(&self) -> builder::MembersGetById {
        builder::MembersGetById::new(self)
    }
    ///Sends a `PUT` request to `/members/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: members body object
    ///```ignore
    /// let response = client.members_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn members_put_by_id(&self) -> builder::MembersPutById {
        builder::MembersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/members/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.members_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn members_delete_by_id(&self) -> builder::MembersDeleteById {
        builder::MembersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/messages`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.messages_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn messages_get(&self) -> builder::MessagesGet {
        builder::MessagesGet::new(self)
    }
    ///Sends a `POST` request to `/messages`
    ///
    ///Arguments:
    /// - `body`: messages body object
    ///```ignore
    /// let response = client.messages_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn messages_post(&self) -> builder::MessagesPost {
        builder::MessagesPost::new(self)
    }
    ///Sends a `GET` request to `/messages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.messages_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn messages_get_by_id(&self) -> builder::MessagesGetById {
        builder::MessagesGetById::new(self)
    }
    ///Sends a `PUT` request to `/messages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: messages body object
    ///```ignore
    /// let response = client.messages_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn messages_put_by_id(&self) -> builder::MessagesPutById {
        builder::MessagesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/messages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.messages_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn messages_delete_by_id(&self) -> builder::MessagesDeleteById {
        builder::MessagesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/meta_data`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.meta_data_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn meta_data_get(&self) -> builder::MetaDataGet {
        builder::MetaDataGet::new(self)
    }
    ///Sends a `POST` request to `/meta_data`
    ///
    ///Arguments:
    /// - `body`: meta_data body object
    ///```ignore
    /// let response = client.meta_data_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn meta_data_post(&self) -> builder::MetaDataPost {
        builder::MetaDataPost::new(self)
    }
    ///Sends a `GET` request to `/meta_data/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.meta_data_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn meta_data_get_by_id(&self) -> builder::MetaDataGetById {
        builder::MetaDataGetById::new(self)
    }
    ///Sends a `PUT` request to `/meta_data/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: meta_data body object
    ///```ignore
    /// let response = client.meta_data_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn meta_data_put_by_id(&self) -> builder::MetaDataPutById {
        builder::MetaDataPutById::new(self)
    }
    ///Sends a `DELETE` request to `/meta_data/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.meta_data_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn meta_data_delete_by_id(&self) -> builder::MetaDataDeleteById {
        builder::MetaDataDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_actions_get(&self) -> builder::NodeActionsGet {
        builder::NodeActionsGet::new(self)
    }
    ///Sends a `POST` request to `/node_actions`
    ///
    ///Arguments:
    /// - `body`: node_actions body object
    ///```ignore
    /// let response = client.node_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_actions_post(&self) -> builder::NodeActionsPost {
        builder::NodeActionsPost::new(self)
    }
    ///Sends a `GET` request to `/node_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_actions_get_by_id(&self) -> builder::NodeActionsGetById {
        builder::NodeActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_actions body object
    ///```ignore
    /// let response = client.node_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_actions_put_by_id(&self) -> builder::NodeActionsPutById {
        builder::NodeActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_actions_delete_by_id(&self) -> builder::NodeActionsDeleteById {
        builder::NodeActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_allocated_gpus`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_allocated_gpus_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_allocated_gpus_get(&self) -> builder::NodeAllocatedGpusGet {
        builder::NodeAllocatedGpusGet::new(self)
    }
    ///Sends a `POST` request to `/node_allocated_gpus`
    ///
    ///Arguments:
    /// - `body`: node_allocated_gpus body object
    ///```ignore
    /// let response = client.node_allocated_gpus_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_allocated_gpus_post(&self) -> builder::NodeAllocatedGpusPost {
        builder::NodeAllocatedGpusPost::new(self)
    }
    ///Sends a `GET` request to `/node_allocated_gpus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_allocated_gpus_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_allocated_gpus_get_by_id(&self) -> builder::NodeAllocatedGpusGetById {
        builder::NodeAllocatedGpusGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_allocated_gpus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_allocated_gpus body object
    ///```ignore
    /// let response = client.node_allocated_gpus_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_allocated_gpus_put_by_id(&self) -> builder::NodeAllocatedGpusPutById {
        builder::NodeAllocatedGpusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_allocated_gpus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_allocated_gpus_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_allocated_gpus_delete_by_id(
        &self,
    ) -> builder::NodeAllocatedGpusDeleteById {
        builder::NodeAllocatedGpusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_device_instances`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_device_instances_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_device_instances_get(&self) -> builder::NodeDeviceInstancesGet {
        builder::NodeDeviceInstancesGet::new(self)
    }
    ///Sends a `POST` request to `/node_device_instances`
    ///
    ///Arguments:
    /// - `body`: node_device_instances body object
    ///```ignore
    /// let response = client.node_device_instances_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_device_instances_post(&self) -> builder::NodeDeviceInstancesPost {
        builder::NodeDeviceInstancesPost::new(self)
    }
    ///Sends a `GET` request to `/node_device_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_device_instances_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_device_instances_get_by_id(
        &self,
    ) -> builder::NodeDeviceInstancesGetById {
        builder::NodeDeviceInstancesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_device_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_device_instances body object
    ///```ignore
    /// let response = client.node_device_instances_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_device_instances_put_by_id(
        &self,
    ) -> builder::NodeDeviceInstancesPutById {
        builder::NodeDeviceInstancesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_device_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_device_instances_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_device_instances_delete_by_id(
        &self,
    ) -> builder::NodeDeviceInstancesDeleteById {
        builder::NodeDeviceInstancesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_drivers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_drivers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_drivers_get(&self) -> builder::NodeDriversGet {
        builder::NodeDriversGet::new(self)
    }
    ///Sends a `POST` request to `/node_drivers`
    ///
    ///Arguments:
    /// - `body`: node_drivers body object
    ///```ignore
    /// let response = client.node_drivers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_drivers_post(&self) -> builder::NodeDriversPost {
        builder::NodeDriversPost::new(self)
    }
    ///Sends a `GET` request to `/node_drivers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_drivers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_drivers_get_by_id(&self) -> builder::NodeDriversGetById {
        builder::NodeDriversGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_drivers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_drivers body object
    ///```ignore
    /// let response = client.node_drivers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_drivers_put_by_id(&self) -> builder::NodeDriversPutById {
        builder::NodeDriversPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_drivers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_drivers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_drivers_delete_by_id(&self) -> builder::NodeDriversDeleteById {
        builder::NodeDriversDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_instances`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_gpu_instances_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_instances_get(&self) -> builder::NodeGpuInstancesGet {
        builder::NodeGpuInstancesGet::new(self)
    }
    ///Sends a `POST` request to `/node_gpu_instances`
    ///
    ///Arguments:
    /// - `body`: node_gpu_instances body object
    ///```ignore
    /// let response = client.node_gpu_instances_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_instances_post(&self) -> builder::NodeGpuInstancesPost {
        builder::NodeGpuInstancesPost::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_gpu_instances_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_instances_get_by_id(&self) -> builder::NodeGpuInstancesGetById {
        builder::NodeGpuInstancesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_gpu_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_gpu_instances body object
    ///```ignore
    /// let response = client.node_gpu_instances_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_instances_put_by_id(&self) -> builder::NodeGpuInstancesPutById {
        builder::NodeGpuInstancesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_gpu_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_gpu_instances_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_instances_delete_by_id(
        &self,
    ) -> builder::NodeGpuInstancesDeleteById {
        builder::NodeGpuInstancesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_gpu_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_get(&self) -> builder::NodeGpuStatsGet {
        builder::NodeGpuStatsGet::new(self)
    }
    ///Sends a `POST` request to `/node_gpu_stats`
    ///
    ///Arguments:
    /// - `body`: node_gpu_stats body object
    ///```ignore
    /// let response = client.node_gpu_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_post(&self) -> builder::NodeGpuStatsPost {
        builder::NodeGpuStatsPost::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_gpu_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_get_by_id(&self) -> builder::NodeGpuStatsGetById {
        builder::NodeGpuStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_gpu_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_gpu_stats body object
    ///```ignore
    /// let response = client.node_gpu_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_put_by_id(&self) -> builder::NodeGpuStatsPutById {
        builder::NodeGpuStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_gpu_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_gpu_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_delete_by_id(&self) -> builder::NodeGpuStatsDeleteById {
        builder::NodeGpuStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_gpu_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_long_get(
        &self,
    ) -> builder::NodeGpuStatsHistoryLongGet {
        builder::NodeGpuStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/node_gpu_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: node_gpu_stats_history_long body object
    ///```ignore
    /// let response = client.node_gpu_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_long_post(
        &self,
    ) -> builder::NodeGpuStatsHistoryLongPost {
        builder::NodeGpuStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_gpu_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_long_get_by_id(
        &self,
    ) -> builder::NodeGpuStatsHistoryLongGetById {
        builder::NodeGpuStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_gpu_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_gpu_stats_history_long body object
    ///```ignore
    /// let response = client.node_gpu_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_long_put_by_id(
        &self,
    ) -> builder::NodeGpuStatsHistoryLongPutById {
        builder::NodeGpuStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_gpu_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_gpu_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_long_delete_by_id(
        &self,
    ) -> builder::NodeGpuStatsHistoryLongDeleteById {
        builder::NodeGpuStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_gpu_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_short_get(
        &self,
    ) -> builder::NodeGpuStatsHistoryShortGet {
        builder::NodeGpuStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/node_gpu_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: node_gpu_stats_history_short body object
    ///```ignore
    /// let response = client.node_gpu_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_short_post(
        &self,
    ) -> builder::NodeGpuStatsHistoryShortPost {
        builder::NodeGpuStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/node_gpu_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_gpu_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_short_get_by_id(
        &self,
    ) -> builder::NodeGpuStatsHistoryShortGetById {
        builder::NodeGpuStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_gpu_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_gpu_stats_history_short body object
    ///```ignore
    /// let response = client.node_gpu_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_short_put_by_id(
        &self,
    ) -> builder::NodeGpuStatsHistoryShortPutById {
        builder::NodeGpuStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_gpu_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_gpu_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpu_stats_history_short_delete_by_id(
        &self,
    ) -> builder::NodeGpuStatsHistoryShortDeleteById {
        builder::NodeGpuStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_gpus`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_gpus_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpus_get(&self) -> builder::NodeGpusGet {
        builder::NodeGpusGet::new(self)
    }
    ///Sends a `POST` request to `/node_gpus`
    ///
    ///Arguments:
    /// - `body`: node_gpus body object
    ///```ignore
    /// let response = client.node_gpus_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpus_post(&self) -> builder::NodeGpusPost {
        builder::NodeGpusPost::new(self)
    }
    ///Sends a `GET` request to `/node_gpus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_gpus_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpus_get_by_id(&self) -> builder::NodeGpusGetById {
        builder::NodeGpusGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_gpus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_gpus body object
    ///```ignore
    /// let response = client.node_gpus_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpus_put_by_id(&self) -> builder::NodeGpusPutById {
        builder::NodeGpusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_gpus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_gpus_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_gpus_delete_by_id(&self) -> builder::NodeGpusDeleteById {
        builder::NodeGpusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_lldp_neighbors`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_lldp_neighbors_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_lldp_neighbors_get(&self) -> builder::NodeLldpNeighborsGet {
        builder::NodeLldpNeighborsGet::new(self)
    }
    ///Sends a `POST` request to `/node_lldp_neighbors`
    ///
    ///Arguments:
    /// - `body`: node_lldp_neighbors body object
    ///```ignore
    /// let response = client.node_lldp_neighbors_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_lldp_neighbors_post(&self) -> builder::NodeLldpNeighborsPost {
        builder::NodeLldpNeighborsPost::new(self)
    }
    ///Sends a `GET` request to `/node_lldp_neighbors/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_lldp_neighbors_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_lldp_neighbors_get_by_id(&self) -> builder::NodeLldpNeighborsGetById {
        builder::NodeLldpNeighborsGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_lldp_neighbors/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_lldp_neighbors body object
    ///```ignore
    /// let response = client.node_lldp_neighbors_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_lldp_neighbors_put_by_id(&self) -> builder::NodeLldpNeighborsPutById {
        builder::NodeLldpNeighborsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_lldp_neighbors/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_lldp_neighbors_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_lldp_neighbors_delete_by_id(
        &self,
    ) -> builder::NodeLldpNeighborsDeleteById {
        builder::NodeLldpNeighborsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_memory`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_memory_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_memory_get(&self) -> builder::NodeMemoryGet {
        builder::NodeMemoryGet::new(self)
    }
    ///Sends a `POST` request to `/node_memory`
    ///
    ///Arguments:
    /// - `body`: node_memory body object
    ///```ignore
    /// let response = client.node_memory_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_memory_post(&self) -> builder::NodeMemoryPost {
        builder::NodeMemoryPost::new(self)
    }
    ///Sends a `GET` request to `/node_memory/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_memory_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_memory_get_by_id(&self) -> builder::NodeMemoryGetById {
        builder::NodeMemoryGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_memory/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_memory body object
    ///```ignore
    /// let response = client.node_memory_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_memory_put_by_id(&self) -> builder::NodeMemoryPutById {
        builder::NodeMemoryPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_memory/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_memory_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_memory_delete_by_id(&self) -> builder::NodeMemoryDeleteById {
        builder::NodeMemoryDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_nvidia_vgpu_devices`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_nvidia_vgpu_devices_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_devices_get(&self) -> builder::NodeNvidiaVgpuDevicesGet {
        builder::NodeNvidiaVgpuDevicesGet::new(self)
    }
    ///Sends a `POST` request to `/node_nvidia_vgpu_devices`
    ///
    ///Arguments:
    /// - `body`: node_nvidia_vgpu_devices body object
    ///```ignore
    /// let response = client.node_nvidia_vgpu_devices_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_devices_post(&self) -> builder::NodeNvidiaVgpuDevicesPost {
        builder::NodeNvidiaVgpuDevicesPost::new(self)
    }
    ///Sends a `GET` request to `/node_nvidia_vgpu_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_nvidia_vgpu_devices_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_devices_get_by_id(
        &self,
    ) -> builder::NodeNvidiaVgpuDevicesGetById {
        builder::NodeNvidiaVgpuDevicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_nvidia_vgpu_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_nvidia_vgpu_devices body object
    ///```ignore
    /// let response = client.node_nvidia_vgpu_devices_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_devices_put_by_id(
        &self,
    ) -> builder::NodeNvidiaVgpuDevicesPutById {
        builder::NodeNvidiaVgpuDevicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_nvidia_vgpu_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_nvidia_vgpu_devices_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_devices_delete_by_id(
        &self,
    ) -> builder::NodeNvidiaVgpuDevicesDeleteById {
        builder::NodeNvidiaVgpuDevicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_nvidia_vgpu_profiles`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_nvidia_vgpu_profiles_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_profiles_get(&self) -> builder::NodeNvidiaVgpuProfilesGet {
        builder::NodeNvidiaVgpuProfilesGet::new(self)
    }
    ///Sends a `POST` request to `/node_nvidia_vgpu_profiles`
    ///
    ///Arguments:
    /// - `body`: node_nvidia_vgpu_profiles body object
    ///```ignore
    /// let response = client.node_nvidia_vgpu_profiles_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_profiles_post(&self) -> builder::NodeNvidiaVgpuProfilesPost {
        builder::NodeNvidiaVgpuProfilesPost::new(self)
    }
    ///Sends a `GET` request to `/node_nvidia_vgpu_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_nvidia_vgpu_profiles_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_profiles_get_by_id(
        &self,
    ) -> builder::NodeNvidiaVgpuProfilesGetById {
        builder::NodeNvidiaVgpuProfilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_nvidia_vgpu_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_nvidia_vgpu_profiles body object
    ///```ignore
    /// let response = client.node_nvidia_vgpu_profiles_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_profiles_put_by_id(
        &self,
    ) -> builder::NodeNvidiaVgpuProfilesPutById {
        builder::NodeNvidiaVgpuProfilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_nvidia_vgpu_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_nvidia_vgpu_profiles_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_nvidia_vgpu_profiles_delete_by_id(
        &self,
    ) -> builder::NodeNvidiaVgpuProfilesDeleteById {
        builder::NodeNvidiaVgpuProfilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_pci_devices`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_pci_devices_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_pci_devices_get(&self) -> builder::NodePciDevicesGet {
        builder::NodePciDevicesGet::new(self)
    }
    ///Sends a `POST` request to `/node_pci_devices`
    ///
    ///Arguments:
    /// - `body`: node_pci_devices body object
    ///```ignore
    /// let response = client.node_pci_devices_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_pci_devices_post(&self) -> builder::NodePciDevicesPost {
        builder::NodePciDevicesPost::new(self)
    }
    ///Sends a `GET` request to `/node_pci_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_pci_devices_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_pci_devices_get_by_id(&self) -> builder::NodePciDevicesGetById {
        builder::NodePciDevicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_pci_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_pci_devices body object
    ///```ignore
    /// let response = client.node_pci_devices_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_pci_devices_put_by_id(&self) -> builder::NodePciDevicesPutById {
        builder::NodePciDevicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_pci_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_pci_devices_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_pci_devices_delete_by_id(&self) -> builder::NodePciDevicesDeleteById {
        builder::NodePciDevicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_queries`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_queries_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_queries_get(&self) -> builder::NodeQueriesGet {
        builder::NodeQueriesGet::new(self)
    }
    ///Sends a `POST` request to `/node_queries`
    ///
    ///Arguments:
    /// - `body`: node_queries body object
    ///```ignore
    /// let response = client.node_queries_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_queries_post(&self) -> builder::NodeQueriesPost {
        builder::NodeQueriesPost::new(self)
    }
    ///Sends a `GET` request to `/node_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_queries_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_queries_get_by_id(&self) -> builder::NodeQueriesGetById {
        builder::NodeQueriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_queries body object
    ///```ignore
    /// let response = client.node_queries_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_queries_put_by_id(&self) -> builder::NodeQueriesPutById {
        builder::NodeQueriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_queries_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_queries_delete_by_id(&self) -> builder::NodeQueriesDeleteById {
        builder::NodeQueriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_resources`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_resources_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_resources_get(&self) -> builder::NodeResourcesGet {
        builder::NodeResourcesGet::new(self)
    }
    ///Sends a `POST` request to `/node_resources`
    ///
    ///Arguments:
    /// - `body`: node_resources body object
    ///```ignore
    /// let response = client.node_resources_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_resources_post(&self) -> builder::NodeResourcesPost {
        builder::NodeResourcesPost::new(self)
    }
    ///Sends a `GET` request to `/node_resources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_resources_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_resources_get_by_id(&self) -> builder::NodeResourcesGetById {
        builder::NodeResourcesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_resources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_resources body object
    ///```ignore
    /// let response = client.node_resources_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_resources_put_by_id(&self) -> builder::NodeResourcesPutById {
        builder::NodeResourcesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_resources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_resources_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_resources_delete_by_id(&self) -> builder::NodeResourcesDeleteById {
        builder::NodeResourcesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_sriov_nic_devices`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_sriov_nic_devices_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_sriov_nic_devices_get(&self) -> builder::NodeSriovNicDevicesGet {
        builder::NodeSriovNicDevicesGet::new(self)
    }
    ///Sends a `POST` request to `/node_sriov_nic_devices`
    ///
    ///Arguments:
    /// - `body`: node_sriov_nic_devices body object
    ///```ignore
    /// let response = client.node_sriov_nic_devices_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_sriov_nic_devices_post(&self) -> builder::NodeSriovNicDevicesPost {
        builder::NodeSriovNicDevicesPost::new(self)
    }
    ///Sends a `GET` request to `/node_sriov_nic_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_sriov_nic_devices_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_sriov_nic_devices_get_by_id(
        &self,
    ) -> builder::NodeSriovNicDevicesGetById {
        builder::NodeSriovNicDevicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_sriov_nic_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_sriov_nic_devices body object
    ///```ignore
    /// let response = client.node_sriov_nic_devices_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_sriov_nic_devices_put_by_id(
        &self,
    ) -> builder::NodeSriovNicDevicesPutById {
        builder::NodeSriovNicDevicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_sriov_nic_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_sriov_nic_devices_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_sriov_nic_devices_delete_by_id(
        &self,
    ) -> builder::NodeSriovNicDevicesDeleteById {
        builder::NodeSriovNicDevicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_stats_get(&self) -> builder::NodeStatsGet {
        builder::NodeStatsGet::new(self)
    }
    ///Sends a `POST` request to `/node_stats`
    ///
    ///Arguments:
    /// - `body`: node_stats body object
    ///```ignore
    /// let response = client.node_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_stats_post(&self) -> builder::NodeStatsPost {
        builder::NodeStatsPost::new(self)
    }
    ///Sends a `GET` request to `/node_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_stats_get_by_id(&self) -> builder::NodeStatsGetById {
        builder::NodeStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_stats body object
    ///```ignore
    /// let response = client.node_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_stats_put_by_id(&self) -> builder::NodeStatsPutById {
        builder::NodeStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_stats_delete_by_id(&self) -> builder::NodeStatsDeleteById {
        builder::NodeStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/node_usb_devices`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.node_usb_devices_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_usb_devices_get(&self) -> builder::NodeUsbDevicesGet {
        builder::NodeUsbDevicesGet::new(self)
    }
    ///Sends a `POST` request to `/node_usb_devices`
    ///
    ///Arguments:
    /// - `body`: node_usb_devices body object
    ///```ignore
    /// let response = client.node_usb_devices_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_usb_devices_post(&self) -> builder::NodeUsbDevicesPost {
        builder::NodeUsbDevicesPost::new(self)
    }
    ///Sends a `GET` request to `/node_usb_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.node_usb_devices_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_usb_devices_get_by_id(&self) -> builder::NodeUsbDevicesGetById {
        builder::NodeUsbDevicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/node_usb_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: node_usb_devices body object
    ///```ignore
    /// let response = client.node_usb_devices_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_usb_devices_put_by_id(&self) -> builder::NodeUsbDevicesPutById {
        builder::NodeUsbDevicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/node_usb_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.node_usb_devices_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn node_usb_devices_delete_by_id(&self) -> builder::NodeUsbDevicesDeleteById {
        builder::NodeUsbDevicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/nodes`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.nodes_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nodes_get(&self) -> builder::NodesGet {
        builder::NodesGet::new(self)
    }
    ///Sends a `POST` request to `/nodes`
    ///
    ///Arguments:
    /// - `body`: nodes body object
    ///```ignore
    /// let response = client.nodes_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nodes_post(&self) -> builder::NodesPost {
        builder::NodesPost::new(self)
    }
    ///Sends a `GET` request to `/nodes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.nodes_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nodes_get_by_id(&self) -> builder::NodesGetById {
        builder::NodesGetById::new(self)
    }
    ///Sends a `PUT` request to `/nodes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: nodes body object
    ///```ignore
    /// let response = client.nodes_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nodes_put_by_id(&self) -> builder::NodesPutById {
        builder::NodesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/nodes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.nodes_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nodes_delete_by_id(&self) -> builder::NodesDeleteById {
        builder::NodesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/note_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.note_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn note_actions_get(&self) -> builder::NoteActionsGet {
        builder::NoteActionsGet::new(self)
    }
    ///Sends a `POST` request to `/note_actions`
    ///
    ///Arguments:
    /// - `body`: note_actions body object
    ///```ignore
    /// let response = client.note_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn note_actions_post(&self) -> builder::NoteActionsPost {
        builder::NoteActionsPost::new(self)
    }
    ///Sends a `GET` request to `/note_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.note_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn note_actions_get_by_id(&self) -> builder::NoteActionsGetById {
        builder::NoteActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/note_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: note_actions body object
    ///```ignore
    /// let response = client.note_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn note_actions_put_by_id(&self) -> builder::NoteActionsPutById {
        builder::NoteActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/note_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.note_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn note_actions_delete_by_id(&self) -> builder::NoteActionsDeleteById {
        builder::NoteActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/nvidia_vgpu_profiles`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.nvidia_vgpu_profiles_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nvidia_vgpu_profiles_get(&self) -> builder::NvidiaVgpuProfilesGet {
        builder::NvidiaVgpuProfilesGet::new(self)
    }
    ///Sends a `POST` request to `/nvidia_vgpu_profiles`
    ///
    ///Arguments:
    /// - `body`: nvidia_vgpu_profiles body object
    ///```ignore
    /// let response = client.nvidia_vgpu_profiles_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nvidia_vgpu_profiles_post(&self) -> builder::NvidiaVgpuProfilesPost {
        builder::NvidiaVgpuProfilesPost::new(self)
    }
    ///Sends a `GET` request to `/nvidia_vgpu_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.nvidia_vgpu_profiles_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nvidia_vgpu_profiles_get_by_id(&self) -> builder::NvidiaVgpuProfilesGetById {
        builder::NvidiaVgpuProfilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/nvidia_vgpu_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.nvidia_vgpu_profiles_put_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nvidia_vgpu_profiles_put_by_id(&self) -> builder::NvidiaVgpuProfilesPutById {
        builder::NvidiaVgpuProfilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/nvidia_vgpu_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.nvidia_vgpu_profiles_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn nvidia_vgpu_profiles_delete_by_id(
        &self,
    ) -> builder::NvidiaVgpuProfilesDeleteById {
        builder::NvidiaVgpuProfilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/oidc_application_groups`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.oidc_application_groups_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_groups_get(&self) -> builder::OidcApplicationGroupsGet {
        builder::OidcApplicationGroupsGet::new(self)
    }
    ///Sends a `POST` request to `/oidc_application_groups`
    ///
    ///Arguments:
    /// - `body`: oidc_application_groups body object
    ///```ignore
    /// let response = client.oidc_application_groups_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_groups_post(&self) -> builder::OidcApplicationGroupsPost {
        builder::OidcApplicationGroupsPost::new(self)
    }
    ///Sends a `GET` request to `/oidc_application_groups/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.oidc_application_groups_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_groups_get_by_id(
        &self,
    ) -> builder::OidcApplicationGroupsGetById {
        builder::OidcApplicationGroupsGetById::new(self)
    }
    ///Sends a `PUT` request to `/oidc_application_groups/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.oidc_application_groups_put_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_groups_put_by_id(
        &self,
    ) -> builder::OidcApplicationGroupsPutById {
        builder::OidcApplicationGroupsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/oidc_application_groups/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.oidc_application_groups_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_groups_delete_by_id(
        &self,
    ) -> builder::OidcApplicationGroupsDeleteById {
        builder::OidcApplicationGroupsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/oidc_application_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.oidc_application_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_logs_get(&self) -> builder::OidcApplicationLogsGet {
        builder::OidcApplicationLogsGet::new(self)
    }
    ///Sends a `POST` request to `/oidc_application_logs`
    ///
    ///Arguments:
    /// - `body`: oidc_application_logs body object
    ///```ignore
    /// let response = client.oidc_application_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_logs_post(&self) -> builder::OidcApplicationLogsPost {
        builder::OidcApplicationLogsPost::new(self)
    }
    ///Sends a `GET` request to `/oidc_application_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.oidc_application_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_logs_get_by_id(
        &self,
    ) -> builder::OidcApplicationLogsGetById {
        builder::OidcApplicationLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/oidc_application_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: oidc_application_logs body object
    ///```ignore
    /// let response = client.oidc_application_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_logs_put_by_id(
        &self,
    ) -> builder::OidcApplicationLogsPutById {
        builder::OidcApplicationLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/oidc_application_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.oidc_application_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_logs_delete_by_id(
        &self,
    ) -> builder::OidcApplicationLogsDeleteById {
        builder::OidcApplicationLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/oidc_application_users`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.oidc_application_users_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_users_get(&self) -> builder::OidcApplicationUsersGet {
        builder::OidcApplicationUsersGet::new(self)
    }
    ///Sends a `POST` request to `/oidc_application_users`
    ///
    ///Arguments:
    /// - `body`: oidc_application_users body object
    ///```ignore
    /// let response = client.oidc_application_users_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_users_post(&self) -> builder::OidcApplicationUsersPost {
        builder::OidcApplicationUsersPost::new(self)
    }
    ///Sends a `GET` request to `/oidc_application_users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.oidc_application_users_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_users_get_by_id(
        &self,
    ) -> builder::OidcApplicationUsersGetById {
        builder::OidcApplicationUsersGetById::new(self)
    }
    ///Sends a `PUT` request to `/oidc_application_users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.oidc_application_users_put_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_users_put_by_id(
        &self,
    ) -> builder::OidcApplicationUsersPutById {
        builder::OidcApplicationUsersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/oidc_application_users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.oidc_application_users_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_application_users_delete_by_id(
        &self,
    ) -> builder::OidcApplicationUsersDeleteById {
        builder::OidcApplicationUsersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/oidc_applications`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.oidc_applications_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_applications_get(&self) -> builder::OidcApplicationsGet {
        builder::OidcApplicationsGet::new(self)
    }
    ///Sends a `POST` request to `/oidc_applications`
    ///
    ///Arguments:
    /// - `body`: oidc_applications body object
    ///```ignore
    /// let response = client.oidc_applications_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_applications_post(&self) -> builder::OidcApplicationsPost {
        builder::OidcApplicationsPost::new(self)
    }
    ///Sends a `GET` request to `/oidc_applications/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.oidc_applications_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_applications_get_by_id(&self) -> builder::OidcApplicationsGetById {
        builder::OidcApplicationsGetById::new(self)
    }
    ///Sends a `PUT` request to `/oidc_applications/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: oidc_applications body object
    ///```ignore
    /// let response = client.oidc_applications_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_applications_put_by_id(&self) -> builder::OidcApplicationsPutById {
        builder::OidcApplicationsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/oidc_applications/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.oidc_applications_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn oidc_applications_delete_by_id(&self) -> builder::OidcApplicationsDeleteById {
        builder::OidcApplicationsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/recipe_questions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.recipe_questions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_questions_get(&self) -> builder::RecipeQuestionsGet {
        builder::RecipeQuestionsGet::new(self)
    }
    ///Sends a `POST` request to `/recipe_questions`
    ///
    ///Arguments:
    /// - `body`: recipe_questions body object
    ///```ignore
    /// let response = client.recipe_questions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_questions_post(&self) -> builder::RecipeQuestionsPost {
        builder::RecipeQuestionsPost::new(self)
    }
    ///Sends a `GET` request to `/recipe_questions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.recipe_questions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_questions_get_by_id(&self) -> builder::RecipeQuestionsGetById {
        builder::RecipeQuestionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/recipe_questions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: recipe_questions body object
    ///```ignore
    /// let response = client.recipe_questions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_questions_put_by_id(&self) -> builder::RecipeQuestionsPutById {
        builder::RecipeQuestionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/recipe_questions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.recipe_questions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_questions_delete_by_id(&self) -> builder::RecipeQuestionsDeleteById {
        builder::RecipeQuestionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/recipe_sections`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.recipe_sections_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_sections_get(&self) -> builder::RecipeSectionsGet {
        builder::RecipeSectionsGet::new(self)
    }
    ///Sends a `POST` request to `/recipe_sections`
    ///
    ///Arguments:
    /// - `body`: recipe_sections body object
    ///```ignore
    /// let response = client.recipe_sections_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_sections_post(&self) -> builder::RecipeSectionsPost {
        builder::RecipeSectionsPost::new(self)
    }
    ///Sends a `GET` request to `/recipe_sections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.recipe_sections_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_sections_get_by_id(&self) -> builder::RecipeSectionsGetById {
        builder::RecipeSectionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/recipe_sections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: recipe_sections body object
    ///```ignore
    /// let response = client.recipe_sections_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_sections_put_by_id(&self) -> builder::RecipeSectionsPutById {
        builder::RecipeSectionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/recipe_sections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.recipe_sections_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_sections_delete_by_id(&self) -> builder::RecipeSectionsDeleteById {
        builder::RecipeSectionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/recipe_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.recipe_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_status_get(&self) -> builder::RecipeStatusGet {
        builder::RecipeStatusGet::new(self)
    }
    ///Sends a `POST` request to `/recipe_status`
    ///
    ///Arguments:
    /// - `body`: recipe_status body object
    ///```ignore
    /// let response = client.recipe_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_status_post(&self) -> builder::RecipeStatusPost {
        builder::RecipeStatusPost::new(self)
    }
    ///Sends a `GET` request to `/recipe_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.recipe_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_status_get_by_id(&self) -> builder::RecipeStatusGetById {
        builder::RecipeStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/recipe_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: recipe_status body object
    ///```ignore
    /// let response = client.recipe_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_status_put_by_id(&self) -> builder::RecipeStatusPutById {
        builder::RecipeStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/recipe_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.recipe_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn recipe_status_delete_by_id(&self) -> builder::RecipeStatusDeleteById {
        builder::RecipeStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/repair_servers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.repair_servers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn repair_servers_get(&self) -> builder::RepairServersGet {
        builder::RepairServersGet::new(self)
    }
    ///Sends a `POST` request to `/repair_servers`
    ///
    ///Arguments:
    /// - `body`: repair_servers body object
    ///```ignore
    /// let response = client.repair_servers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn repair_servers_post(&self) -> builder::RepairServersPost {
        builder::RepairServersPost::new(self)
    }
    ///Sends a `GET` request to `/repair_servers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.repair_servers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn repair_servers_get_by_id(&self) -> builder::RepairServersGetById {
        builder::RepairServersGetById::new(self)
    }
    ///Sends a `PUT` request to `/repair_servers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: repair_servers body object
    ///```ignore
    /// let response = client.repair_servers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn repair_servers_put_by_id(&self) -> builder::RepairServersPutById {
        builder::RepairServersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/repair_servers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.repair_servers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn repair_servers_delete_by_id(&self) -> builder::RepairServersDeleteById {
        builder::RepairServersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/resource_group_settings_nvidia_vgpu`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.resource_group_settings_nvidia_vgpu_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_nvidia_vgpu_get(
        &self,
    ) -> builder::ResourceGroupSettingsNvidiaVgpuGet {
        builder::ResourceGroupSettingsNvidiaVgpuGet::new(self)
    }
    ///Sends a `POST` request to `/resource_group_settings_nvidia_vgpu`
    ///
    ///Arguments:
    /// - `body`: resource_group_settings_nvidia_vgpu body object
    ///```ignore
    /// let response = client.resource_group_settings_nvidia_vgpu_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_nvidia_vgpu_post(
        &self,
    ) -> builder::ResourceGroupSettingsNvidiaVgpuPost {
        builder::ResourceGroupSettingsNvidiaVgpuPost::new(self)
    }
    ///Sends a `GET` request to `/resource_group_settings_nvidia_vgpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.resource_group_settings_nvidia_vgpu_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_nvidia_vgpu_get_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsNvidiaVgpuGetById {
        builder::ResourceGroupSettingsNvidiaVgpuGetById::new(self)
    }
    ///Sends a `PUT` request to `/resource_group_settings_nvidia_vgpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: resource_group_settings_nvidia_vgpu body object
    ///```ignore
    /// let response = client.resource_group_settings_nvidia_vgpu_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_nvidia_vgpu_put_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsNvidiaVgpuPutById {
        builder::ResourceGroupSettingsNvidiaVgpuPutById::new(self)
    }
    ///Sends a `DELETE` request to `/resource_group_settings_nvidia_vgpu/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.resource_group_settings_nvidia_vgpu_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_nvidia_vgpu_delete_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsNvidiaVgpuDeleteById {
        builder::ResourceGroupSettingsNvidiaVgpuDeleteById::new(self)
    }
    ///Sends a `GET` request to `/resource_group_settings_sriov_nic`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.resource_group_settings_sriov_nic_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_sriov_nic_get(
        &self,
    ) -> builder::ResourceGroupSettingsSriovNicGet {
        builder::ResourceGroupSettingsSriovNicGet::new(self)
    }
    ///Sends a `POST` request to `/resource_group_settings_sriov_nic`
    ///
    ///Arguments:
    /// - `body`: resource_group_settings_sriov_nic body object
    ///```ignore
    /// let response = client.resource_group_settings_sriov_nic_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_sriov_nic_post(
        &self,
    ) -> builder::ResourceGroupSettingsSriovNicPost {
        builder::ResourceGroupSettingsSriovNicPost::new(self)
    }
    ///Sends a `GET` request to `/resource_group_settings_sriov_nic/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.resource_group_settings_sriov_nic_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_sriov_nic_get_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsSriovNicGetById {
        builder::ResourceGroupSettingsSriovNicGetById::new(self)
    }
    ///Sends a `PUT` request to `/resource_group_settings_sriov_nic/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: resource_group_settings_sriov_nic body object
    ///```ignore
    /// let response = client.resource_group_settings_sriov_nic_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_sriov_nic_put_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsSriovNicPutById {
        builder::ResourceGroupSettingsSriovNicPutById::new(self)
    }
    ///Sends a `DELETE` request to `/resource_group_settings_sriov_nic/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.resource_group_settings_sriov_nic_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_sriov_nic_delete_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsSriovNicDeleteById {
        builder::ResourceGroupSettingsSriovNicDeleteById::new(self)
    }
    ///Sends a `GET` request to `/resource_group_settings_usb`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.resource_group_settings_usb_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_usb_get(
        &self,
    ) -> builder::ResourceGroupSettingsUsbGet {
        builder::ResourceGroupSettingsUsbGet::new(self)
    }
    ///Sends a `POST` request to `/resource_group_settings_usb`
    ///
    ///Arguments:
    /// - `body`: resource_group_settings_usb body object
    ///```ignore
    /// let response = client.resource_group_settings_usb_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_usb_post(
        &self,
    ) -> builder::ResourceGroupSettingsUsbPost {
        builder::ResourceGroupSettingsUsbPost::new(self)
    }
    ///Sends a `GET` request to `/resource_group_settings_usb/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.resource_group_settings_usb_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_usb_get_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsUsbGetById {
        builder::ResourceGroupSettingsUsbGetById::new(self)
    }
    ///Sends a `PUT` request to `/resource_group_settings_usb/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: resource_group_settings_usb body object
    ///```ignore
    /// let response = client.resource_group_settings_usb_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_usb_put_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsUsbPutById {
        builder::ResourceGroupSettingsUsbPutById::new(self)
    }
    ///Sends a `DELETE` request to `/resource_group_settings_usb/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.resource_group_settings_usb_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_group_settings_usb_delete_by_id(
        &self,
    ) -> builder::ResourceGroupSettingsUsbDeleteById {
        builder::ResourceGroupSettingsUsbDeleteById::new(self)
    }
    ///Sends a `GET` request to `/resource_groups`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.resource_groups_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_groups_get(&self) -> builder::ResourceGroupsGet {
        builder::ResourceGroupsGet::new(self)
    }
    ///Sends a `POST` request to `/resource_groups`
    ///
    ///Arguments:
    /// - `body`: resource_groups body object
    ///```ignore
    /// let response = client.resource_groups_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_groups_post(&self) -> builder::ResourceGroupsPost {
        builder::ResourceGroupsPost::new(self)
    }
    ///Sends a `GET` request to `/resource_groups/{uuid}`
    ///
    ///Arguments:
    /// - `uuid`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.resource_groups_get_by_uuid()
    ///    .uuid(uuid)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_groups_get_by_uuid(&self) -> builder::ResourceGroupsGetByUuid {
        builder::ResourceGroupsGetByUuid::new(self)
    }
    ///Sends a `PUT` request to `/resource_groups/{uuid}`
    ///
    ///Arguments:
    /// - `uuid`: resource id
    /// - `body`: resource_groups body object
    ///```ignore
    /// let response = client.resource_groups_put_by_uuid()
    ///    .uuid(uuid)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_groups_put_by_uuid(&self) -> builder::ResourceGroupsPutByUuid {
        builder::ResourceGroupsPutByUuid::new(self)
    }
    ///Sends a `DELETE` request to `/resource_groups/{uuid}`
    ///
    ///Arguments:
    /// - `uuid`: resource id
    ///```ignore
    /// let response = client.resource_groups_delete_by_uuid()
    ///    .uuid(uuid)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_groups_delete_by_uuid(&self) -> builder::ResourceGroupsDeleteByUuid {
        builder::ResourceGroupsDeleteByUuid::new(self)
    }
    ///Sends a `GET` request to `/resource_rules`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.resource_rules_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_rules_get(&self) -> builder::ResourceRulesGet {
        builder::ResourceRulesGet::new(self)
    }
    ///Sends a `POST` request to `/resource_rules`
    ///
    ///Arguments:
    /// - `body`: resource_rules body object
    ///```ignore
    /// let response = client.resource_rules_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_rules_post(&self) -> builder::ResourceRulesPost {
        builder::ResourceRulesPost::new(self)
    }
    ///Sends a `GET` request to `/resource_rules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.resource_rules_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_rules_get_by_id(&self) -> builder::ResourceRulesGetById {
        builder::ResourceRulesGetById::new(self)
    }
    ///Sends a `PUT` request to `/resource_rules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: resource_rules body object
    ///```ignore
    /// let response = client.resource_rules_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_rules_put_by_id(&self) -> builder::ResourceRulesPutById {
        builder::ResourceRulesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/resource_rules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.resource_rules_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn resource_rules_delete_by_id(&self) -> builder::ResourceRulesDeleteById {
        builder::ResourceRulesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/root_certificates`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.root_certificates_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn root_certificates_get(&self) -> builder::RootCertificatesGet {
        builder::RootCertificatesGet::new(self)
    }
    ///Sends a `POST` request to `/root_certificates`
    ///
    ///Arguments:
    /// - `body`: root_certificates body object
    ///```ignore
    /// let response = client.root_certificates_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn root_certificates_post(&self) -> builder::RootCertificatesPost {
        builder::RootCertificatesPost::new(self)
    }
    ///Sends a `GET` request to `/root_certificates/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.root_certificates_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn root_certificates_get_by_id(&self) -> builder::RootCertificatesGetById {
        builder::RootCertificatesGetById::new(self)
    }
    ///Sends a `PUT` request to `/root_certificates/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: root_certificates body object
    ///```ignore
    /// let response = client.root_certificates_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn root_certificates_put_by_id(&self) -> builder::RootCertificatesPutById {
        builder::RootCertificatesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/root_certificates/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.root_certificates_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn root_certificates_delete_by_id(&self) -> builder::RootCertificatesDeleteById {
        builder::RootCertificatesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/schedule_task_events`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schedule_task_events_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_events_get(&self) -> builder::ScheduleTaskEventsGet {
        builder::ScheduleTaskEventsGet::new(self)
    }
    ///Sends a `POST` request to `/schedule_task_events`
    ///
    ///Arguments:
    /// - `body`: schedule_task_events body object
    ///```ignore
    /// let response = client.schedule_task_events_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_events_post(&self) -> builder::ScheduleTaskEventsPost {
        builder::ScheduleTaskEventsPost::new(self)
    }
    ///Sends a `GET` request to `/schedule_task_events/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schedule_task_events_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_events_get_by_id(&self) -> builder::ScheduleTaskEventsGetById {
        builder::ScheduleTaskEventsGetById::new(self)
    }
    ///Sends a `PUT` request to `/schedule_task_events/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: schedule_task_events body object
    ///```ignore
    /// let response = client.schedule_task_events_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_events_put_by_id(&self) -> builder::ScheduleTaskEventsPutById {
        builder::ScheduleTaskEventsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/schedule_task_events/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.schedule_task_events_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_events_delete_by_id(
        &self,
    ) -> builder::ScheduleTaskEventsDeleteById {
        builder::ScheduleTaskEventsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/schedule_task_setting_schema`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schedule_task_setting_schema_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_setting_schema_get(
        &self,
    ) -> builder::ScheduleTaskSettingSchemaGet {
        builder::ScheduleTaskSettingSchemaGet::new(self)
    }
    ///Sends a `POST` request to `/schedule_task_setting_schema`
    ///
    ///Arguments:
    /// - `body`: schedule_task_setting_schema body object
    ///```ignore
    /// let response = client.schedule_task_setting_schema_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_setting_schema_post(
        &self,
    ) -> builder::ScheduleTaskSettingSchemaPost {
        builder::ScheduleTaskSettingSchemaPost::new(self)
    }
    ///Sends a `GET` request to `/schedule_task_setting_schema/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schedule_task_setting_schema_get_by_key()
    ///    .key(key)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_setting_schema_get_by_key(
        &self,
    ) -> builder::ScheduleTaskSettingSchemaGetByKey {
        builder::ScheduleTaskSettingSchemaGetByKey::new(self)
    }
    ///Sends a `PUT` request to `/schedule_task_setting_schema/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    /// - `body`: schedule_task_setting_schema body object
    ///```ignore
    /// let response = client.schedule_task_setting_schema_put_by_key()
    ///    .key(key)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_setting_schema_put_by_key(
        &self,
    ) -> builder::ScheduleTaskSettingSchemaPutByKey {
        builder::ScheduleTaskSettingSchemaPutByKey::new(self)
    }
    ///Sends a `DELETE` request to `/schedule_task_setting_schema/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    ///```ignore
    /// let response = client.schedule_task_setting_schema_delete_by_key()
    ///    .key(key)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_setting_schema_delete_by_key(
        &self,
    ) -> builder::ScheduleTaskSettingSchemaDeleteByKey {
        builder::ScheduleTaskSettingSchemaDeleteByKey::new(self)
    }
    ///Sends a `GET` request to `/schedule_task_settings`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schedule_task_settings_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_settings_get(&self) -> builder::ScheduleTaskSettingsGet {
        builder::ScheduleTaskSettingsGet::new(self)
    }
    ///Sends a `POST` request to `/schedule_task_settings`
    ///
    ///Arguments:
    /// - `body`: schedule_task_settings body object
    ///```ignore
    /// let response = client.schedule_task_settings_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_settings_post(&self) -> builder::ScheduleTaskSettingsPost {
        builder::ScheduleTaskSettingsPost::new(self)
    }
    ///Sends a `GET` request to `/schedule_task_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schedule_task_settings_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_settings_get_by_id(
        &self,
    ) -> builder::ScheduleTaskSettingsGetById {
        builder::ScheduleTaskSettingsGetById::new(self)
    }
    ///Sends a `PUT` request to `/schedule_task_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: schedule_task_settings body object
    ///```ignore
    /// let response = client.schedule_task_settings_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_settings_put_by_id(
        &self,
    ) -> builder::ScheduleTaskSettingsPutById {
        builder::ScheduleTaskSettingsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/schedule_task_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.schedule_task_settings_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_task_settings_delete_by_id(
        &self,
    ) -> builder::ScheduleTaskSettingsDeleteById {
        builder::ScheduleTaskSettingsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/schedule_tasks`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schedule_tasks_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_tasks_get(&self) -> builder::ScheduleTasksGet {
        builder::ScheduleTasksGet::new(self)
    }
    ///Sends a `POST` request to `/schedule_tasks`
    ///
    ///Arguments:
    /// - `body`: schedule_tasks body object
    ///```ignore
    /// let response = client.schedule_tasks_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_tasks_post(&self) -> builder::ScheduleTasksPost {
        builder::ScheduleTasksPost::new(self)
    }
    ///Sends a `GET` request to `/schedule_tasks/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schedule_tasks_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_tasks_get_by_id(&self) -> builder::ScheduleTasksGetById {
        builder::ScheduleTasksGetById::new(self)
    }
    ///Sends a `PUT` request to `/schedule_tasks/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: schedule_tasks body object
    ///```ignore
    /// let response = client.schedule_tasks_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_tasks_put_by_id(&self) -> builder::ScheduleTasksPutById {
        builder::ScheduleTasksPutById::new(self)
    }
    ///Sends a `DELETE` request to `/schedule_tasks/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.schedule_tasks_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedule_tasks_delete_by_id(&self) -> builder::ScheduleTasksDeleteById {
        builder::ScheduleTasksDeleteById::new(self)
    }
    ///Sends a `GET` request to `/schedules`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schedules_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedules_get(&self) -> builder::SchedulesGet {
        builder::SchedulesGet::new(self)
    }
    ///Sends a `POST` request to `/schedules`
    ///
    ///Arguments:
    /// - `body`: schedules body object
    ///```ignore
    /// let response = client.schedules_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedules_post(&self) -> builder::SchedulesPost {
        builder::SchedulesPost::new(self)
    }
    ///Sends a `GET` request to `/schedules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schedules_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedules_get_by_id(&self) -> builder::SchedulesGetById {
        builder::SchedulesGetById::new(self)
    }
    ///Sends a `PUT` request to `/schedules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: schedules body object
    ///```ignore
    /// let response = client.schedules_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedules_put_by_id(&self) -> builder::SchedulesPutById {
        builder::SchedulesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/schedules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.schedules_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schedules_delete_by_id(&self) -> builder::SchedulesDeleteById {
        builder::SchedulesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/schema_version_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schema_version_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_version_actions_get(&self) -> builder::SchemaVersionActionsGet {
        builder::SchemaVersionActionsGet::new(self)
    }
    ///Sends a `POST` request to `/schema_version_actions`
    ///
    ///Arguments:
    /// - `body`: schema_version_actions body object
    ///```ignore
    /// let response = client.schema_version_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_version_actions_post(&self) -> builder::SchemaVersionActionsPost {
        builder::SchemaVersionActionsPost::new(self)
    }
    ///Sends a `GET` request to `/schema_version_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schema_version_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_version_actions_get_by_id(
        &self,
    ) -> builder::SchemaVersionActionsGetById {
        builder::SchemaVersionActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/schema_version_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: schema_version_actions body object
    ///```ignore
    /// let response = client.schema_version_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_version_actions_put_by_id(
        &self,
    ) -> builder::SchemaVersionActionsPutById {
        builder::SchemaVersionActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/schema_version_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.schema_version_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_version_actions_delete_by_id(
        &self,
    ) -> builder::SchemaVersionActionsDeleteById {
        builder::SchemaVersionActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/schema_versions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.schema_versions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_versions_get(&self) -> builder::SchemaVersionsGet {
        builder::SchemaVersionsGet::new(self)
    }
    ///Sends a `POST` request to `/schema_versions`
    ///
    ///Arguments:
    /// - `body`: schema_versions body object
    ///```ignore
    /// let response = client.schema_versions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_versions_post(&self) -> builder::SchemaVersionsPost {
        builder::SchemaVersionsPost::new(self)
    }
    ///Sends a `GET` request to `/schema_versions/{table}`
    ///
    ///Arguments:
    /// - `table`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.schema_versions_get_by_table()
    ///    .table(table)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_versions_get_by_table(&self) -> builder::SchemaVersionsGetByTable {
        builder::SchemaVersionsGetByTable::new(self)
    }
    ///Sends a `PUT` request to `/schema_versions/{table}`
    ///
    ///Arguments:
    /// - `table`: resource id
    ///```ignore
    /// let response = client.schema_versions_put_by_table()
    ///    .table(table)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_versions_put_by_table(&self) -> builder::SchemaVersionsPutByTable {
        builder::SchemaVersionsPutByTable::new(self)
    }
    ///Sends a `DELETE` request to `/schema_versions/{table}`
    ///
    ///Arguments:
    /// - `table`: resource id
    ///```ignore
    /// let response = client.schema_versions_delete_by_table()
    ///    .table(table)
    ///    .send()
    ///    .await;
    /// ```
    pub fn schema_versions_delete_by_table(
        &self,
    ) -> builder::SchemaVersionsDeleteByTable {
        builder::SchemaVersionsDeleteByTable::new(self)
    }
    ///Sends a `GET` request to `/settings`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.settings_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_get(&self) -> builder::SettingsGet {
        builder::SettingsGet::new(self)
    }
    ///Sends a `POST` request to `/settings`
    ///
    ///Arguments:
    /// - `body`: settings body object
    ///```ignore
    /// let response = client.settings_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_post(&self) -> builder::SettingsPost {
        builder::SettingsPost::new(self)
    }
    ///Sends a `GET` request to `/settings/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.settings_get_by_key()
    ///    .key(key)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_get_by_key(&self) -> builder::SettingsGetByKey {
        builder::SettingsGetByKey::new(self)
    }
    ///Sends a `PUT` request to `/settings/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    /// - `body`: settings body object
    ///```ignore
    /// let response = client.settings_put_by_key()
    ///    .key(key)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_put_by_key(&self) -> builder::SettingsPutByKey {
        builder::SettingsPutByKey::new(self)
    }
    ///Sends a `DELETE` request to `/settings/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    ///```ignore
    /// let response = client.settings_delete_by_key()
    ///    .key(key)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_delete_by_key(&self) -> builder::SettingsDeleteByKey {
        builder::SettingsDeleteByKey::new(self)
    }
    ///Sends a `GET` request to `/settings_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.settings_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_actions_get(&self) -> builder::SettingsActionsGet {
        builder::SettingsActionsGet::new(self)
    }
    ///Sends a `POST` request to `/settings_actions`
    ///
    ///Arguments:
    /// - `body`: settings_actions body object
    ///```ignore
    /// let response = client.settings_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_actions_post(&self) -> builder::SettingsActionsPost {
        builder::SettingsActionsPost::new(self)
    }
    ///Sends a `GET` request to `/settings_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.settings_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_actions_get_by_id(&self) -> builder::SettingsActionsGetById {
        builder::SettingsActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/settings_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: settings_actions body object
    ///```ignore
    /// let response = client.settings_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_actions_put_by_id(&self) -> builder::SettingsActionsPutById {
        builder::SettingsActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/settings_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.settings_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn settings_actions_delete_by_id(&self) -> builder::SettingsActionsDeleteById {
        builder::SettingsActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/shared_object_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.shared_object_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_object_actions_get(&self) -> builder::SharedObjectActionsGet {
        builder::SharedObjectActionsGet::new(self)
    }
    ///Sends a `POST` request to `/shared_object_actions`
    ///
    ///Arguments:
    /// - `body`: shared_object_actions body object
    ///```ignore
    /// let response = client.shared_object_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_object_actions_post(&self) -> builder::SharedObjectActionsPost {
        builder::SharedObjectActionsPost::new(self)
    }
    ///Sends a `GET` request to `/shared_object_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.shared_object_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_object_actions_get_by_id(
        &self,
    ) -> builder::SharedObjectActionsGetById {
        builder::SharedObjectActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/shared_object_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: shared_object_actions body object
    ///```ignore
    /// let response = client.shared_object_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_object_actions_put_by_id(
        &self,
    ) -> builder::SharedObjectActionsPutById {
        builder::SharedObjectActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/shared_object_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.shared_object_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_object_actions_delete_by_id(
        &self,
    ) -> builder::SharedObjectActionsDeleteById {
        builder::SharedObjectActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/shared_objects`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.shared_objects_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_objects_get(&self) -> builder::SharedObjectsGet {
        builder::SharedObjectsGet::new(self)
    }
    ///Sends a `POST` request to `/shared_objects`
    ///
    ///Arguments:
    /// - `body`: shared_objects body object
    ///```ignore
    /// let response = client.shared_objects_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_objects_post(&self) -> builder::SharedObjectsPost {
        builder::SharedObjectsPost::new(self)
    }
    ///Sends a `GET` request to `/shared_objects/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.shared_objects_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_objects_get_by_id(&self) -> builder::SharedObjectsGetById {
        builder::SharedObjectsGetById::new(self)
    }
    ///Sends a `PUT` request to `/shared_objects/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: shared_objects body object
    ///```ignore
    /// let response = client.shared_objects_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_objects_put_by_id(&self) -> builder::SharedObjectsPutById {
        builder::SharedObjectsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/shared_objects/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.shared_objects_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn shared_objects_delete_by_id(&self) -> builder::SharedObjectsDeleteById {
        builder::SharedObjectsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_actions_get(&self) -> builder::SiteActionsGet {
        builder::SiteActionsGet::new(self)
    }
    ///Sends a `POST` request to `/site_actions`
    ///
    ///Arguments:
    /// - `body`: site_actions body object
    ///```ignore
    /// let response = client.site_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_actions_post(&self) -> builder::SiteActionsPost {
        builder::SiteActionsPost::new(self)
    }
    ///Sends a `GET` request to `/site_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_actions_get_by_id(&self) -> builder::SiteActionsGetById {
        builder::SiteActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_actions body object
    ///```ignore
    /// let response = client.site_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_actions_put_by_id(&self) -> builder::SiteActionsPutById {
        builder::SiteActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_actions_delete_by_id(&self) -> builder::SiteActionsDeleteById {
        builder::SiteActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_logs_get(&self) -> builder::SiteLogsGet {
        builder::SiteLogsGet::new(self)
    }
    ///Sends a `POST` request to `/site_logs`
    ///
    ///Arguments:
    /// - `body`: site_logs body object
    ///```ignore
    /// let response = client.site_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_logs_post(&self) -> builder::SiteLogsPost {
        builder::SiteLogsPost::new(self)
    }
    ///Sends a `GET` request to `/site_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_logs_get_by_id(&self) -> builder::SiteLogsGetById {
        builder::SiteLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_logs body object
    ///```ignore
    /// let response = client.site_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_logs_put_by_id(&self) -> builder::SiteLogsPutById {
        builder::SiteLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_logs_delete_by_id(&self) -> builder::SiteLogsDeleteById {
        builder::SiteLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_sync_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_sync_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_get(&self) -> builder::SiteSyncStatsGet {
        builder::SiteSyncStatsGet::new(self)
    }
    ///Sends a `POST` request to `/site_sync_stats`
    ///
    ///Arguments:
    /// - `body`: site_sync_stats body object
    ///```ignore
    /// let response = client.site_sync_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_post(&self) -> builder::SiteSyncStatsPost {
        builder::SiteSyncStatsPost::new(self)
    }
    ///Sends a `GET` request to `/site_sync_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_sync_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_get_by_id(&self) -> builder::SiteSyncStatsGetById {
        builder::SiteSyncStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_sync_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_sync_stats body object
    ///```ignore
    /// let response = client.site_sync_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_put_by_id(&self) -> builder::SiteSyncStatsPutById {
        builder::SiteSyncStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_sync_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_sync_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_delete_by_id(&self) -> builder::SiteSyncStatsDeleteById {
        builder::SiteSyncStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_sync_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_sync_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_history_long_get(
        &self,
    ) -> builder::SiteSyncStatsHistoryLongGet {
        builder::SiteSyncStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/site_sync_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: site_sync_stats_history_long body object
    ///```ignore
    /// let response = client.site_sync_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_history_long_post(
        &self,
    ) -> builder::SiteSyncStatsHistoryLongPost {
        builder::SiteSyncStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/site_sync_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_sync_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_history_long_get_by_id(
        &self,
    ) -> builder::SiteSyncStatsHistoryLongGetById {
        builder::SiteSyncStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_sync_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_sync_stats_history_long body object
    ///```ignore
    /// let response = client.site_sync_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_history_long_put_by_id(
        &self,
    ) -> builder::SiteSyncStatsHistoryLongPutById {
        builder::SiteSyncStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_sync_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_sync_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_sync_stats_history_long_delete_by_id(
        &self,
    ) -> builder::SiteSyncStatsHistoryLongDeleteById {
        builder::SiteSyncStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_incoming_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_get(&self) -> builder::SiteSyncsIncomingGet {
        builder::SiteSyncsIncomingGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_incoming`
    ///
    ///Arguments:
    /// - `body`: site_syncs_incoming body object
    ///```ignore
    /// let response = client.site_syncs_incoming_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_post(&self) -> builder::SiteSyncsIncomingPost {
        builder::SiteSyncsIncomingPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_incoming_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_get_by_id(&self) -> builder::SiteSyncsIncomingGetById {
        builder::SiteSyncsIncomingGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_incoming/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_incoming body object
    ///```ignore
    /// let response = client.site_syncs_incoming_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_put_by_id(&self) -> builder::SiteSyncsIncomingPutById {
        builder::SiteSyncsIncomingPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_incoming/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_incoming_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_delete_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingDeleteById {
        builder::SiteSyncsIncomingDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_incoming_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_actions_get(
        &self,
    ) -> builder::SiteSyncsIncomingActionsGet {
        builder::SiteSyncsIncomingActionsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_incoming_actions`
    ///
    ///Arguments:
    /// - `body`: site_syncs_incoming_actions body object
    ///```ignore
    /// let response = client.site_syncs_incoming_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_actions_post(
        &self,
    ) -> builder::SiteSyncsIncomingActionsPost {
        builder::SiteSyncsIncomingActionsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_incoming_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_actions_get_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingActionsGetById {
        builder::SiteSyncsIncomingActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_incoming_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_incoming_actions body object
    ///```ignore
    /// let response = client.site_syncs_incoming_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_actions_put_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingActionsPutById {
        builder::SiteSyncsIncomingActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_incoming_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_incoming_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_actions_delete_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingActionsDeleteById {
        builder::SiteSyncsIncomingActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_incoming_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_logs_get(&self) -> builder::SiteSyncsIncomingLogsGet {
        builder::SiteSyncsIncomingLogsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_incoming_logs`
    ///
    ///Arguments:
    /// - `body`: site_syncs_incoming_logs body object
    ///```ignore
    /// let response = client.site_syncs_incoming_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_logs_post(&self) -> builder::SiteSyncsIncomingLogsPost {
        builder::SiteSyncsIncomingLogsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_incoming_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_logs_get_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingLogsGetById {
        builder::SiteSyncsIncomingLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_incoming_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_incoming_logs body object
    ///```ignore
    /// let response = client.site_syncs_incoming_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_logs_put_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingLogsPutById {
        builder::SiteSyncsIncomingLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_incoming_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_incoming_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_logs_delete_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingLogsDeleteById {
        builder::SiteSyncsIncomingLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_verified`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_get(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedGet {
        builder::SiteSyncsIncomingVerifiedGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_incoming_verified`
    ///
    ///Arguments:
    /// - `body`: site_syncs_incoming_verified body object
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_post(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedPost {
        builder::SiteSyncsIncomingVerifiedPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_verified/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_get_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedGetById {
        builder::SiteSyncsIncomingVerifiedGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_incoming_verified/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_incoming_verified body object
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_put_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedPutById {
        builder::SiteSyncsIncomingVerifiedPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_incoming_verified/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_delete_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedDeleteById {
        builder::SiteSyncsIncomingVerifiedDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_verified_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_actions_get(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedActionsGet {
        builder::SiteSyncsIncomingVerifiedActionsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_incoming_verified_actions`
    ///
    ///Arguments:
    /// - `body`: site_syncs_incoming_verified_actions body object
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_actions_post(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedActionsPost {
        builder::SiteSyncsIncomingVerifiedActionsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_incoming_verified_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_actions_get_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedActionsGetById {
        builder::SiteSyncsIncomingVerifiedActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_incoming_verified_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_incoming_verified_actions body object
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_actions_put_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedActionsPutById {
        builder::SiteSyncsIncomingVerifiedActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_incoming_verified_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_incoming_verified_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_incoming_verified_actions_delete_by_id(
        &self,
    ) -> builder::SiteSyncsIncomingVerifiedActionsDeleteById {
        builder::SiteSyncsIncomingVerifiedActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_get(&self) -> builder::SiteSyncsOutgoingGet {
        builder::SiteSyncsOutgoingGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_post(&self) -> builder::SiteSyncsOutgoingPost {
        builder::SiteSyncsOutgoingPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_get_by_id(&self) -> builder::SiteSyncsOutgoingGetById {
        builder::SiteSyncsOutgoingGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_put_by_id(&self) -> builder::SiteSyncsOutgoingPutById {
        builder::SiteSyncsOutgoingPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_outgoing/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingDeleteById {
        builder::SiteSyncsOutgoingDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_actions_get(
        &self,
    ) -> builder::SiteSyncsOutgoingActionsGet {
        builder::SiteSyncsOutgoingActionsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing_actions`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing_actions body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_actions_post(
        &self,
    ) -> builder::SiteSyncsOutgoingActionsPost {
        builder::SiteSyncsOutgoingActionsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_actions_get_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingActionsGetById {
        builder::SiteSyncsOutgoingActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing_actions body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_actions_put_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingActionsPutById {
        builder::SiteSyncsOutgoingActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_outgoing_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_actions_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingActionsDeleteById {
        builder::SiteSyncsOutgoingActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_logs_get(&self) -> builder::SiteSyncsOutgoingLogsGet {
        builder::SiteSyncsOutgoingLogsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing_logs`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing_logs body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_logs_post(&self) -> builder::SiteSyncsOutgoingLogsPost {
        builder::SiteSyncsOutgoingLogsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_logs_get_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingLogsGetById {
        builder::SiteSyncsOutgoingLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing_logs body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_logs_put_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingLogsPutById {
        builder::SiteSyncsOutgoingLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_outgoing_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_logs_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingLogsDeleteById {
        builder::SiteSyncsOutgoingLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_profile_periods`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_profile_periods_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_profile_periods_get(
        &self,
    ) -> builder::SiteSyncsOutgoingProfilePeriodsGet {
        builder::SiteSyncsOutgoingProfilePeriodsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing_profile_periods`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing_profile_periods body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_profile_periods_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_profile_periods_post(
        &self,
    ) -> builder::SiteSyncsOutgoingProfilePeriodsPost {
        builder::SiteSyncsOutgoingProfilePeriodsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_profile_periods/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_profile_periods_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_profile_periods_get_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingProfilePeriodsGetById {
        builder::SiteSyncsOutgoingProfilePeriodsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing_profile_periods/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing_profile_periods body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_profile_periods_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_profile_periods_put_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingProfilePeriodsPutById {
        builder::SiteSyncsOutgoingProfilePeriodsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_outgoing_profile_periods/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_profile_periods_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_profile_periods_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingProfilePeriodsDeleteById {
        builder::SiteSyncsOutgoingProfilePeriodsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_queue`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_queue_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_queue_get(&self) -> builder::SiteSyncsOutgoingQueueGet {
        builder::SiteSyncsOutgoingQueueGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing_queue`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing_queue body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_queue_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_queue_post(&self) -> builder::SiteSyncsOutgoingQueuePost {
        builder::SiteSyncsOutgoingQueuePost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_queue/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_queue_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_queue_get_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingQueueGetById {
        builder::SiteSyncsOutgoingQueueGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing_queue/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing_queue body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_queue_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_queue_put_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingQueuePutById {
        builder::SiteSyncsOutgoingQueuePutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_outgoing_queue/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_queue_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_queue_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingQueueDeleteById {
        builder::SiteSyncsOutgoingQueueDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_remote_snap_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snap_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snap_actions_get(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapActionsGet {
        builder::SiteSyncsOutgoingRemoteSnapActionsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing_remote_snap_actions`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing_remote_snap_actions body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snap_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snap_actions_post(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapActionsPost {
        builder::SiteSyncsOutgoingRemoteSnapActionsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_remote_snap_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snap_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snap_actions_get_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapActionsGetById {
        builder::SiteSyncsOutgoingRemoteSnapActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing_remote_snap_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing_remote_snap_actions body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snap_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snap_actions_put_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapActionsPutById {
        builder::SiteSyncsOutgoingRemoteSnapActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to
    /// `/site_syncs_outgoing_remote_snap_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snap_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snap_actions_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapActionsDeleteById {
        builder::SiteSyncsOutgoingRemoteSnapActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_remote_snaps`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snaps_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snaps_get(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapsGet {
        builder::SiteSyncsOutgoingRemoteSnapsGet::new(self)
    }
    ///Sends a `POST` request to `/site_syncs_outgoing_remote_snaps`
    ///
    ///Arguments:
    /// - `body`: site_syncs_outgoing_remote_snaps body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snaps_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snaps_post(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapsPost {
        builder::SiteSyncsOutgoingRemoteSnapsPost::new(self)
    }
    ///Sends a `GET` request to `/site_syncs_outgoing_remote_snaps/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snaps_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snaps_get_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapsGetById {
        builder::SiteSyncsOutgoingRemoteSnapsGetById::new(self)
    }
    ///Sends a `PUT` request to `/site_syncs_outgoing_remote_snaps/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: site_syncs_outgoing_remote_snaps body object
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snaps_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snaps_put_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapsPutById {
        builder::SiteSyncsOutgoingRemoteSnapsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/site_syncs_outgoing_remote_snaps/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.site_syncs_outgoing_remote_snaps_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn site_syncs_outgoing_remote_snaps_delete_by_id(
        &self,
    ) -> builder::SiteSyncsOutgoingRemoteSnapsDeleteById {
        builder::SiteSyncsOutgoingRemoteSnapsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vms`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vms_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vms_get(&self) -> builder::VmsGet {
        builder::VmsGet::new(self)
    }
    ///Sends a `POST` request to `/vms`
    ///
    ///Arguments:
    /// - `body`: vms body object
    ///```ignore
    /// let response = client.vms_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vms_post(&self) -> builder::VmsPost {
        builder::VmsPost::new(self)
    }
    ///Sends a `GET` request to `/vms/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vms_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vms_get_by_id(&self) -> builder::VmsGetById {
        builder::VmsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vms/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vms body object
    ///```ignore
    /// let response = client.vms_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vms_put_by_id(&self) -> builder::VmsPutById {
        builder::VmsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vms/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vms_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vms_delete_by_id(&self) -> builder::VmsDeleteById {
        builder::VmsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/smtp_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.smtp_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_actions_get(&self) -> builder::SmtpActionsGet {
        builder::SmtpActionsGet::new(self)
    }
    ///Sends a `POST` request to `/smtp_actions`
    ///
    ///Arguments:
    /// - `body`: smtp_actions body object
    ///```ignore
    /// let response = client.smtp_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_actions_post(&self) -> builder::SmtpActionsPost {
        builder::SmtpActionsPost::new(self)
    }
    ///Sends a `GET` request to `/smtp_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.smtp_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_actions_get_by_id(&self) -> builder::SmtpActionsGetById {
        builder::SmtpActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/smtp_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: smtp_actions body object
    ///```ignore
    /// let response = client.smtp_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_actions_put_by_id(&self) -> builder::SmtpActionsPutById {
        builder::SmtpActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/smtp_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.smtp_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_actions_delete_by_id(&self) -> builder::SmtpActionsDeleteById {
        builder::SmtpActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/smtp_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.smtp_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_logs_get(&self) -> builder::SmtpLogsGet {
        builder::SmtpLogsGet::new(self)
    }
    ///Sends a `POST` request to `/smtp_logs`
    ///
    ///Arguments:
    /// - `body`: smtp_logs body object
    ///```ignore
    /// let response = client.smtp_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_logs_post(&self) -> builder::SmtpLogsPost {
        builder::SmtpLogsPost::new(self)
    }
    ///Sends a `GET` request to `/smtp_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.smtp_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_logs_get_by_id(&self) -> builder::SmtpLogsGetById {
        builder::SmtpLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/smtp_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: smtp_logs body object
    ///```ignore
    /// let response = client.smtp_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_logs_put_by_id(&self) -> builder::SmtpLogsPutById {
        builder::SmtpLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/smtp_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.smtp_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_logs_delete_by_id(&self) -> builder::SmtpLogsDeleteById {
        builder::SmtpLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/smtp_outbox`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.smtp_outbox_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_outbox_get(&self) -> builder::SmtpOutboxGet {
        builder::SmtpOutboxGet::new(self)
    }
    ///Sends a `POST` request to `/smtp_outbox`
    ///
    ///Arguments:
    /// - `body`: smtp_outbox body object
    ///```ignore
    /// let response = client.smtp_outbox_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_outbox_post(&self) -> builder::SmtpOutboxPost {
        builder::SmtpOutboxPost::new(self)
    }
    ///Sends a `GET` request to `/smtp_outbox/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.smtp_outbox_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_outbox_get_by_id(&self) -> builder::SmtpOutboxGetById {
        builder::SmtpOutboxGetById::new(self)
    }
    ///Sends a `PUT` request to `/smtp_outbox/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: smtp_outbox body object
    ///```ignore
    /// let response = client.smtp_outbox_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_outbox_put_by_id(&self) -> builder::SmtpOutboxPutById {
        builder::SmtpOutboxPutById::new(self)
    }
    ///Sends a `DELETE` request to `/smtp_outbox/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.smtp_outbox_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_outbox_delete_by_id(&self) -> builder::SmtpOutboxDeleteById {
        builder::SmtpOutboxDeleteById::new(self)
    }
    ///Sends a `GET` request to `/smtp_queue`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.smtp_queue_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_queue_get(&self) -> builder::SmtpQueueGet {
        builder::SmtpQueueGet::new(self)
    }
    ///Sends a `POST` request to `/smtp_queue`
    ///
    ///Arguments:
    /// - `body`: smtp_queue body object
    ///```ignore
    /// let response = client.smtp_queue_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_queue_post(&self) -> builder::SmtpQueuePost {
        builder::SmtpQueuePost::new(self)
    }
    ///Sends a `GET` request to `/smtp_queue/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.smtp_queue_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_queue_get_by_id(&self) -> builder::SmtpQueueGetById {
        builder::SmtpQueueGetById::new(self)
    }
    ///Sends a `PUT` request to `/smtp_queue/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: smtp_queue body object
    ///```ignore
    /// let response = client.smtp_queue_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_queue_put_by_id(&self) -> builder::SmtpQueuePutById {
        builder::SmtpQueuePutById::new(self)
    }
    ///Sends a `DELETE` request to `/smtp_queue/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.smtp_queue_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_queue_delete_by_id(&self) -> builder::SmtpQueueDeleteById {
        builder::SmtpQueueDeleteById::new(self)
    }
    ///Sends a `GET` request to `/smtp_settings`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.smtp_settings_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_settings_get(&self) -> builder::SmtpSettingsGet {
        builder::SmtpSettingsGet::new(self)
    }
    ///Sends a `POST` request to `/smtp_settings`
    ///
    ///Arguments:
    /// - `body`: smtp_settings body object
    ///```ignore
    /// let response = client.smtp_settings_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_settings_post(&self) -> builder::SmtpSettingsPost {
        builder::SmtpSettingsPost::new(self)
    }
    ///Sends a `GET` request to `/smtp_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.smtp_settings_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_settings_get_by_id(&self) -> builder::SmtpSettingsGetById {
        builder::SmtpSettingsGetById::new(self)
    }
    ///Sends a `PUT` request to `/smtp_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: smtp_settings body object
    ///```ignore
    /// let response = client.smtp_settings_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_settings_put_by_id(&self) -> builder::SmtpSettingsPutById {
        builder::SmtpSettingsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/smtp_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.smtp_settings_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn smtp_settings_delete_by_id(&self) -> builder::SmtpSettingsDeleteById {
        builder::SmtpSettingsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/snapshot_profile_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.snapshot_profile_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_actions_get(&self) -> builder::SnapshotProfileActionsGet {
        builder::SnapshotProfileActionsGet::new(self)
    }
    ///Sends a `POST` request to `/snapshot_profile_actions`
    ///
    ///Arguments:
    /// - `body`: snapshot_profile_actions body object
    ///```ignore
    /// let response = client.snapshot_profile_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_actions_post(&self) -> builder::SnapshotProfileActionsPost {
        builder::SnapshotProfileActionsPost::new(self)
    }
    ///Sends a `GET` request to `/snapshot_profile_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.snapshot_profile_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_actions_get_by_id(
        &self,
    ) -> builder::SnapshotProfileActionsGetById {
        builder::SnapshotProfileActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/snapshot_profile_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: snapshot_profile_actions body object
    ///```ignore
    /// let response = client.snapshot_profile_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_actions_put_by_id(
        &self,
    ) -> builder::SnapshotProfileActionsPutById {
        builder::SnapshotProfileActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/snapshot_profile_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.snapshot_profile_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_actions_delete_by_id(
        &self,
    ) -> builder::SnapshotProfileActionsDeleteById {
        builder::SnapshotProfileActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/snapshot_profile_periods`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.snapshot_profile_periods_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_periods_get(&self) -> builder::SnapshotProfilePeriodsGet {
        builder::SnapshotProfilePeriodsGet::new(self)
    }
    ///Sends a `POST` request to `/snapshot_profile_periods`
    ///
    ///Arguments:
    /// - `body`: snapshot_profile_periods body object
    ///```ignore
    /// let response = client.snapshot_profile_periods_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_periods_post(&self) -> builder::SnapshotProfilePeriodsPost {
        builder::SnapshotProfilePeriodsPost::new(self)
    }
    ///Sends a `GET` request to `/snapshot_profile_periods/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.snapshot_profile_periods_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_periods_get_by_id(
        &self,
    ) -> builder::SnapshotProfilePeriodsGetById {
        builder::SnapshotProfilePeriodsGetById::new(self)
    }
    ///Sends a `PUT` request to `/snapshot_profile_periods/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: snapshot_profile_periods body object
    ///```ignore
    /// let response = client.snapshot_profile_periods_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_periods_put_by_id(
        &self,
    ) -> builder::SnapshotProfilePeriodsPutById {
        builder::SnapshotProfilePeriodsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/snapshot_profile_periods/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.snapshot_profile_periods_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profile_periods_delete_by_id(
        &self,
    ) -> builder::SnapshotProfilePeriodsDeleteById {
        builder::SnapshotProfilePeriodsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/snapshot_profiles`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.snapshot_profiles_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profiles_get(&self) -> builder::SnapshotProfilesGet {
        builder::SnapshotProfilesGet::new(self)
    }
    ///Sends a `POST` request to `/snapshot_profiles`
    ///
    ///Arguments:
    /// - `body`: snapshot_profiles body object
    ///```ignore
    /// let response = client.snapshot_profiles_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profiles_post(&self) -> builder::SnapshotProfilesPost {
        builder::SnapshotProfilesPost::new(self)
    }
    ///Sends a `GET` request to `/snapshot_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.snapshot_profiles_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profiles_get_by_id(&self) -> builder::SnapshotProfilesGetById {
        builder::SnapshotProfilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/snapshot_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: snapshot_profiles body object
    ///```ignore
    /// let response = client.snapshot_profiles_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profiles_put_by_id(&self) -> builder::SnapshotProfilesPutById {
        builder::SnapshotProfilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/snapshot_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.snapshot_profiles_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn snapshot_profiles_delete_by_id(&self) -> builder::SnapshotProfilesDeleteById {
        builder::SnapshotProfilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/storage_tier_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.storage_tier_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_get(&self) -> builder::StorageTierStatsGet {
        builder::StorageTierStatsGet::new(self)
    }
    ///Sends a `POST` request to `/storage_tier_stats`
    ///
    ///Arguments:
    /// - `body`: storage_tier_stats body object
    ///```ignore
    /// let response = client.storage_tier_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_post(&self) -> builder::StorageTierStatsPost {
        builder::StorageTierStatsPost::new(self)
    }
    ///Sends a `GET` request to `/storage_tier_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.storage_tier_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_get_by_id(&self) -> builder::StorageTierStatsGetById {
        builder::StorageTierStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/storage_tier_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: storage_tier_stats body object
    ///```ignore
    /// let response = client.storage_tier_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_put_by_id(&self) -> builder::StorageTierStatsPutById {
        builder::StorageTierStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/storage_tier_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.storage_tier_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_delete_by_id(
        &self,
    ) -> builder::StorageTierStatsDeleteById {
        builder::StorageTierStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/storage_tier_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.storage_tier_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_long_get(
        &self,
    ) -> builder::StorageTierStatsHistoryLongGet {
        builder::StorageTierStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/storage_tier_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: storage_tier_stats_history_long body object
    ///```ignore
    /// let response = client.storage_tier_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_long_post(
        &self,
    ) -> builder::StorageTierStatsHistoryLongPost {
        builder::StorageTierStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/storage_tier_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.storage_tier_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_long_get_by_id(
        &self,
    ) -> builder::StorageTierStatsHistoryLongGetById {
        builder::StorageTierStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/storage_tier_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: storage_tier_stats_history_long body object
    ///```ignore
    /// let response = client.storage_tier_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_long_put_by_id(
        &self,
    ) -> builder::StorageTierStatsHistoryLongPutById {
        builder::StorageTierStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/storage_tier_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.storage_tier_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_long_delete_by_id(
        &self,
    ) -> builder::StorageTierStatsHistoryLongDeleteById {
        builder::StorageTierStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/storage_tier_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.storage_tier_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_short_get(
        &self,
    ) -> builder::StorageTierStatsHistoryShortGet {
        builder::StorageTierStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/storage_tier_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: storage_tier_stats_history_short body object
    ///```ignore
    /// let response = client.storage_tier_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_short_post(
        &self,
    ) -> builder::StorageTierStatsHistoryShortPost {
        builder::StorageTierStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/storage_tier_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.storage_tier_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_short_get_by_id(
        &self,
    ) -> builder::StorageTierStatsHistoryShortGetById {
        builder::StorageTierStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/storage_tier_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: storage_tier_stats_history_short body object
    ///```ignore
    /// let response = client.storage_tier_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_short_put_by_id(
        &self,
    ) -> builder::StorageTierStatsHistoryShortPutById {
        builder::StorageTierStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/storage_tier_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.storage_tier_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tier_stats_history_short_delete_by_id(
        &self,
    ) -> builder::StorageTierStatsHistoryShortDeleteById {
        builder::StorageTierStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/storage_tiers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.storage_tiers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tiers_get(&self) -> builder::StorageTiersGet {
        builder::StorageTiersGet::new(self)
    }
    ///Sends a `POST` request to `/storage_tiers`
    ///
    ///Arguments:
    /// - `body`: storage_tiers body object
    ///```ignore
    /// let response = client.storage_tiers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tiers_post(&self) -> builder::StorageTiersPost {
        builder::StorageTiersPost::new(self)
    }
    ///Sends a `GET` request to `/storage_tiers/{tier}`
    ///
    ///Arguments:
    /// - `tier`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.storage_tiers_get_by_tier()
    ///    .tier(tier)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tiers_get_by_tier(&self) -> builder::StorageTiersGetByTier {
        builder::StorageTiersGetByTier::new(self)
    }
    ///Sends a `PUT` request to `/storage_tiers/{tier}`
    ///
    ///Arguments:
    /// - `tier`: resource id
    /// - `body`: storage_tiers body object
    ///```ignore
    /// let response = client.storage_tiers_put_by_tier()
    ///    .tier(tier)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tiers_put_by_tier(&self) -> builder::StorageTiersPutByTier {
        builder::StorageTiersPutByTier::new(self)
    }
    ///Sends a `DELETE` request to `/storage_tiers/{tier}`
    ///
    ///Arguments:
    /// - `tier`: resource id
    ///```ignore
    /// let response = client.storage_tiers_delete_by_tier()
    ///    .tier(tier)
    ///    .send()
    ///    .await;
    /// ```
    pub fn storage_tiers_delete_by_tier(&self) -> builder::StorageTiersDeleteByTier {
        builder::StorageTiersDeleteByTier::new(self)
    }
    ///Sends a `GET` request to `/subscription_profiles`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.subscription_profiles_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscription_profiles_get(&self) -> builder::SubscriptionProfilesGet {
        builder::SubscriptionProfilesGet::new(self)
    }
    ///Sends a `POST` request to `/subscription_profiles`
    ///
    ///Arguments:
    /// - `body`: subscription_profiles body object
    ///```ignore
    /// let response = client.subscription_profiles_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscription_profiles_post(&self) -> builder::SubscriptionProfilesPost {
        builder::SubscriptionProfilesPost::new(self)
    }
    ///Sends a `GET` request to `/subscription_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.subscription_profiles_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscription_profiles_get_by_id(
        &self,
    ) -> builder::SubscriptionProfilesGetById {
        builder::SubscriptionProfilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/subscription_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: subscription_profiles body object
    ///```ignore
    /// let response = client.subscription_profiles_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscription_profiles_put_by_id(
        &self,
    ) -> builder::SubscriptionProfilesPutById {
        builder::SubscriptionProfilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/subscription_profiles/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.subscription_profiles_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscription_profiles_delete_by_id(
        &self,
    ) -> builder::SubscriptionProfilesDeleteById {
        builder::SubscriptionProfilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/subscriptions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.subscriptions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscriptions_get(&self) -> builder::SubscriptionsGet {
        builder::SubscriptionsGet::new(self)
    }
    ///Sends a `POST` request to `/subscriptions`
    ///
    ///Arguments:
    /// - `body`: subscriptions body object
    ///```ignore
    /// let response = client.subscriptions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscriptions_post(&self) -> builder::SubscriptionsPost {
        builder::SubscriptionsPost::new(self)
    }
    ///Sends a `GET` request to `/subscriptions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.subscriptions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscriptions_get_by_id(&self) -> builder::SubscriptionsGetById {
        builder::SubscriptionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/subscriptions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: subscriptions body object
    ///```ignore
    /// let response = client.subscriptions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscriptions_put_by_id(&self) -> builder::SubscriptionsPutById {
        builder::SubscriptionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/subscriptions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.subscriptions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn subscriptions_delete_by_id(&self) -> builder::SubscriptionsDeleteById {
        builder::SubscriptionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/swagger_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.swagger_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn swagger_actions_get(&self) -> builder::SwaggerActionsGet {
        builder::SwaggerActionsGet::new(self)
    }
    ///Sends a `POST` request to `/swagger_actions`
    ///
    ///Arguments:
    /// - `body`: swagger_actions body object
    ///```ignore
    /// let response = client.swagger_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn swagger_actions_post(&self) -> builder::SwaggerActionsPost {
        builder::SwaggerActionsPost::new(self)
    }
    ///Sends a `GET` request to `/swagger_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.swagger_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn swagger_actions_get_by_id(&self) -> builder::SwaggerActionsGetById {
        builder::SwaggerActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/swagger_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: swagger_actions body object
    ///```ignore
    /// let response = client.swagger_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn swagger_actions_put_by_id(&self) -> builder::SwaggerActionsPutById {
        builder::SwaggerActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/swagger_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.swagger_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn swagger_actions_delete_by_id(&self) -> builder::SwaggerActionsDeleteById {
        builder::SwaggerActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/system`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.system_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_get(&self) -> builder::SystemGet {
        builder::SystemGet::new(self)
    }
    ///Sends a `POST` request to `/system`
    ///
    ///Arguments:
    /// - `body`: system body object
    ///```ignore
    /// let response = client.system_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_post(&self) -> builder::SystemPost {
        builder::SystemPost::new(self)
    }
    ///Sends a `GET` request to `/system/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.system_get_by_key()
    ///    .key(key)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_get_by_key(&self) -> builder::SystemGetByKey {
        builder::SystemGetByKey::new(self)
    }
    ///Sends a `PUT` request to `/system/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    /// - `body`: system body object
    ///```ignore
    /// let response = client.system_put_by_key()
    ///    .key(key)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_put_by_key(&self) -> builder::SystemPutByKey {
        builder::SystemPutByKey::new(self)
    }
    ///Sends a `DELETE` request to `/system/{key}`
    ///
    ///Arguments:
    /// - `key`: resource id
    ///```ignore
    /// let response = client.system_delete_by_key()
    ///    .key(key)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_delete_by_key(&self) -> builder::SystemDeleteByKey {
        builder::SystemDeleteByKey::new(self)
    }
    ///Sends a `GET` request to `/system_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.system_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_actions_get(&self) -> builder::SystemActionsGet {
        builder::SystemActionsGet::new(self)
    }
    ///Sends a `POST` request to `/system_actions`
    ///
    ///Arguments:
    /// - `body`: system_actions body object
    ///```ignore
    /// let response = client.system_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_actions_post(&self) -> builder::SystemActionsPost {
        builder::SystemActionsPost::new(self)
    }
    ///Sends a `GET` request to `/system_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.system_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_actions_get_by_id(&self) -> builder::SystemActionsGetById {
        builder::SystemActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/system_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: system_actions body object
    ///```ignore
    /// let response = client.system_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_actions_put_by_id(&self) -> builder::SystemActionsPutById {
        builder::SystemActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/system_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.system_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_actions_delete_by_id(&self) -> builder::SystemActionsDeleteById {
        builder::SystemActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/system_diagnostic_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.system_diagnostic_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostic_actions_get(&self) -> builder::SystemDiagnosticActionsGet {
        builder::SystemDiagnosticActionsGet::new(self)
    }
    ///Sends a `POST` request to `/system_diagnostic_actions`
    ///
    ///Arguments:
    /// - `body`: system_diagnostic_actions body object
    ///```ignore
    /// let response = client.system_diagnostic_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostic_actions_post(
        &self,
    ) -> builder::SystemDiagnosticActionsPost {
        builder::SystemDiagnosticActionsPost::new(self)
    }
    ///Sends a `GET` request to `/system_diagnostic_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.system_diagnostic_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostic_actions_get_by_id(
        &self,
    ) -> builder::SystemDiagnosticActionsGetById {
        builder::SystemDiagnosticActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/system_diagnostic_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: system_diagnostic_actions body object
    ///```ignore
    /// let response = client.system_diagnostic_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostic_actions_put_by_id(
        &self,
    ) -> builder::SystemDiagnosticActionsPutById {
        builder::SystemDiagnosticActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/system_diagnostic_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.system_diagnostic_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostic_actions_delete_by_id(
        &self,
    ) -> builder::SystemDiagnosticActionsDeleteById {
        builder::SystemDiagnosticActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/system_diagnostics`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.system_diagnostics_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostics_get(&self) -> builder::SystemDiagnosticsGet {
        builder::SystemDiagnosticsGet::new(self)
    }
    ///Sends a `POST` request to `/system_diagnostics`
    ///
    ///Arguments:
    /// - `body`: system_diagnostics body object
    ///```ignore
    /// let response = client.system_diagnostics_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostics_post(&self) -> builder::SystemDiagnosticsPost {
        builder::SystemDiagnosticsPost::new(self)
    }
    ///Sends a `GET` request to `/system_diagnostics/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.system_diagnostics_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostics_get_by_id(&self) -> builder::SystemDiagnosticsGetById {
        builder::SystemDiagnosticsGetById::new(self)
    }
    ///Sends a `PUT` request to `/system_diagnostics/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: system_diagnostics body object
    ///```ignore
    /// let response = client.system_diagnostics_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostics_put_by_id(&self) -> builder::SystemDiagnosticsPutById {
        builder::SystemDiagnosticsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/system_diagnostics/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.system_diagnostics_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn system_diagnostics_delete_by_id(
        &self,
    ) -> builder::SystemDiagnosticsDeleteById {
        builder::SystemDiagnosticsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_actions_get(&self) -> builder::TenantActionsGet {
        builder::TenantActionsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_actions`
    ///
    ///Arguments:
    /// - `body`: tenant_actions body object
    ///```ignore
    /// let response = client.tenant_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_actions_post(&self) -> builder::TenantActionsPost {
        builder::TenantActionsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_actions_get_by_id(&self) -> builder::TenantActionsGetById {
        builder::TenantActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_actions body object
    ///```ignore
    /// let response = client.tenant_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_actions_put_by_id(&self) -> builder::TenantActionsPutById {
        builder::TenantActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_actions_delete_by_id(&self) -> builder::TenantActionsDeleteById {
        builder::TenantActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_logs_get(&self) -> builder::TenantLogsGet {
        builder::TenantLogsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_logs`
    ///
    ///Arguments:
    /// - `body`: tenant_logs body object
    ///```ignore
    /// let response = client.tenant_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_logs_post(&self) -> builder::TenantLogsPost {
        builder::TenantLogsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_logs_get_by_id(&self) -> builder::TenantLogsGetById {
        builder::TenantLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_logs body object
    ///```ignore
    /// let response = client.tenant_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_logs_put_by_id(&self) -> builder::TenantLogsPutById {
        builder::TenantLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_logs_delete_by_id(&self) -> builder::TenantLogsDeleteById {
        builder::TenantLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_node_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_node_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_actions_get(&self) -> builder::TenantNodeActionsGet {
        builder::TenantNodeActionsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_node_actions`
    ///
    ///Arguments:
    /// - `body`: tenant_node_actions body object
    ///```ignore
    /// let response = client.tenant_node_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_actions_post(&self) -> builder::TenantNodeActionsPost {
        builder::TenantNodeActionsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_node_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_node_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_actions_get_by_id(&self) -> builder::TenantNodeActionsGetById {
        builder::TenantNodeActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_node_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_node_actions body object
    ///```ignore
    /// let response = client.tenant_node_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_actions_put_by_id(&self) -> builder::TenantNodeActionsPutById {
        builder::TenantNodeActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_node_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_node_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_actions_delete_by_id(
        &self,
    ) -> builder::TenantNodeActionsDeleteById {
        builder::TenantNodeActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_node_queries`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_node_queries_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_queries_get(&self) -> builder::TenantNodeQueriesGet {
        builder::TenantNodeQueriesGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_node_queries`
    ///
    ///Arguments:
    /// - `body`: tenant_node_queries body object
    ///```ignore
    /// let response = client.tenant_node_queries_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_queries_post(&self) -> builder::TenantNodeQueriesPost {
        builder::TenantNodeQueriesPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_node_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_node_queries_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_queries_get_by_id(&self) -> builder::TenantNodeQueriesGetById {
        builder::TenantNodeQueriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_node_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_node_queries body object
    ///```ignore
    /// let response = client.tenant_node_queries_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_queries_put_by_id(&self) -> builder::TenantNodeQueriesPutById {
        builder::TenantNodeQueriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_node_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_node_queries_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_node_queries_delete_by_id(
        &self,
    ) -> builder::TenantNodeQueriesDeleteById {
        builder::TenantNodeQueriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_nodes`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_nodes_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_nodes_get(&self) -> builder::TenantNodesGet {
        builder::TenantNodesGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_nodes`
    ///
    ///Arguments:
    /// - `body`: tenant_nodes body object
    ///```ignore
    /// let response = client.tenant_nodes_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_nodes_post(&self) -> builder::TenantNodesPost {
        builder::TenantNodesPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_nodes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_nodes_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_nodes_get_by_id(&self) -> builder::TenantNodesGetById {
        builder::TenantNodesGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_nodes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_nodes body object
    ///```ignore
    /// let response = client.tenant_nodes_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_nodes_put_by_id(&self) -> builder::TenantNodesPutById {
        builder::TenantNodesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_nodes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_nodes_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_nodes_delete_by_id(&self) -> builder::TenantNodesDeleteById {
        builder::TenantNodesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipe_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_recipe_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_actions_get(&self) -> builder::TenantRecipeActionsGet {
        builder::TenantRecipeActionsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_recipe_actions`
    ///
    ///Arguments:
    /// - `body`: tenant_recipe_actions body object
    ///```ignore
    /// let response = client.tenant_recipe_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_actions_post(&self) -> builder::TenantRecipeActionsPost {
        builder::TenantRecipeActionsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipe_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_recipe_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_actions_get_by_id(
        &self,
    ) -> builder::TenantRecipeActionsGetById {
        builder::TenantRecipeActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_recipe_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_recipe_actions body object
    ///```ignore
    /// let response = client.tenant_recipe_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_actions_put_by_id(
        &self,
    ) -> builder::TenantRecipeActionsPutById {
        builder::TenantRecipeActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_recipe_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_recipe_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_actions_delete_by_id(
        &self,
    ) -> builder::TenantRecipeActionsDeleteById {
        builder::TenantRecipeActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipe_instances`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_recipe_instances_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_instances_get(&self) -> builder::TenantRecipeInstancesGet {
        builder::TenantRecipeInstancesGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_recipe_instances`
    ///
    ///Arguments:
    /// - `body`: tenant_recipe_instances body object
    ///```ignore
    /// let response = client.tenant_recipe_instances_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_instances_post(&self) -> builder::TenantRecipeInstancesPost {
        builder::TenantRecipeInstancesPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipe_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_recipe_instances_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_instances_get_by_id(
        &self,
    ) -> builder::TenantRecipeInstancesGetById {
        builder::TenantRecipeInstancesGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_recipe_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_recipe_instances body object
    ///```ignore
    /// let response = client.tenant_recipe_instances_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_instances_put_by_id(
        &self,
    ) -> builder::TenantRecipeInstancesPutById {
        builder::TenantRecipeInstancesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_recipe_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_recipe_instances_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_instances_delete_by_id(
        &self,
    ) -> builder::TenantRecipeInstancesDeleteById {
        builder::TenantRecipeInstancesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipe_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_recipe_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_logs_get(&self) -> builder::TenantRecipeLogsGet {
        builder::TenantRecipeLogsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_recipe_logs`
    ///
    ///Arguments:
    /// - `body`: tenant_recipe_logs body object
    ///```ignore
    /// let response = client.tenant_recipe_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_logs_post(&self) -> builder::TenantRecipeLogsPost {
        builder::TenantRecipeLogsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipe_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_recipe_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_logs_get_by_id(&self) -> builder::TenantRecipeLogsGetById {
        builder::TenantRecipeLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_recipe_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_recipe_logs body object
    ///```ignore
    /// let response = client.tenant_recipe_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_logs_put_by_id(&self) -> builder::TenantRecipeLogsPutById {
        builder::TenantRecipeLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_recipe_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_recipe_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipe_logs_delete_by_id(
        &self,
    ) -> builder::TenantRecipeLogsDeleteById {
        builder::TenantRecipeLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipes`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_recipes_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipes_get(&self) -> builder::TenantRecipesGet {
        builder::TenantRecipesGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_recipes`
    ///
    ///Arguments:
    /// - `body`: tenant_recipes body object
    ///```ignore
    /// let response = client.tenant_recipes_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipes_post(&self) -> builder::TenantRecipesPost {
        builder::TenantRecipesPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_recipes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_recipes_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipes_get_by_id(&self) -> builder::TenantRecipesGetById {
        builder::TenantRecipesGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_recipes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_recipes body object
    ///```ignore
    /// let response = client.tenant_recipes_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipes_put_by_id(&self) -> builder::TenantRecipesPutById {
        builder::TenantRecipesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_recipes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_recipes_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_recipes_delete_by_id(&self) -> builder::TenantRecipesDeleteById {
        builder::TenantRecipesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_snapshot_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_snapshot_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshot_actions_get(&self) -> builder::TenantSnapshotActionsGet {
        builder::TenantSnapshotActionsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_snapshot_actions`
    ///
    ///Arguments:
    /// - `body`: tenant_snapshot_actions body object
    ///```ignore
    /// let response = client.tenant_snapshot_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshot_actions_post(&self) -> builder::TenantSnapshotActionsPost {
        builder::TenantSnapshotActionsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_snapshot_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_snapshot_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshot_actions_get_by_id(
        &self,
    ) -> builder::TenantSnapshotActionsGetById {
        builder::TenantSnapshotActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_snapshot_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_snapshot_actions body object
    ///```ignore
    /// let response = client.tenant_snapshot_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshot_actions_put_by_id(
        &self,
    ) -> builder::TenantSnapshotActionsPutById {
        builder::TenantSnapshotActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_snapshot_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_snapshot_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshot_actions_delete_by_id(
        &self,
    ) -> builder::TenantSnapshotActionsDeleteById {
        builder::TenantSnapshotActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_snapshots`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_snapshots_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshots_get(&self) -> builder::TenantSnapshotsGet {
        builder::TenantSnapshotsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_snapshots`
    ///
    ///Arguments:
    /// - `body`: tenant_snapshots body object
    ///```ignore
    /// let response = client.tenant_snapshots_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshots_post(&self) -> builder::TenantSnapshotsPost {
        builder::TenantSnapshotsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_snapshots_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshots_get_by_id(&self) -> builder::TenantSnapshotsGetById {
        builder::TenantSnapshotsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_snapshots body object
    ///```ignore
    /// let response = client.tenant_snapshots_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshots_put_by_id(&self) -> builder::TenantSnapshotsPutById {
        builder::TenantSnapshotsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_snapshots_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_snapshots_delete_by_id(&self) -> builder::TenantSnapshotsDeleteById {
        builder::TenantSnapshotsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_get(&self) -> builder::TenantStatsGet {
        builder::TenantStatsGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_stats`
    ///
    ///Arguments:
    /// - `body`: tenant_stats body object
    ///```ignore
    /// let response = client.tenant_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_post(&self) -> builder::TenantStatsPost {
        builder::TenantStatsPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_get_by_id(&self) -> builder::TenantStatsGetById {
        builder::TenantStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_stats body object
    ///```ignore
    /// let response = client.tenant_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_put_by_id(&self) -> builder::TenantStatsPutById {
        builder::TenantStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_delete_by_id(&self) -> builder::TenantStatsDeleteById {
        builder::TenantStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_long_get(&self) -> builder::TenantStatsHistoryLongGet {
        builder::TenantStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: tenant_stats_history_long body object
    ///```ignore
    /// let response = client.tenant_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_long_post(&self) -> builder::TenantStatsHistoryLongPost {
        builder::TenantStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_long_get_by_id(
        &self,
    ) -> builder::TenantStatsHistoryLongGetById {
        builder::TenantStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_stats_history_long body object
    ///```ignore
    /// let response = client.tenant_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_long_put_by_id(
        &self,
    ) -> builder::TenantStatsHistoryLongPutById {
        builder::TenantStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_long_delete_by_id(
        &self,
    ) -> builder::TenantStatsHistoryLongDeleteById {
        builder::TenantStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_short_get(&self) -> builder::TenantStatsHistoryShortGet {
        builder::TenantStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: tenant_stats_history_short body object
    ///```ignore
    /// let response = client.tenant_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_short_post(
        &self,
    ) -> builder::TenantStatsHistoryShortPost {
        builder::TenantStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_short_get_by_id(
        &self,
    ) -> builder::TenantStatsHistoryShortGetById {
        builder::TenantStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_stats_history_short body object
    ///```ignore
    /// let response = client.tenant_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_short_put_by_id(
        &self,
    ) -> builder::TenantStatsHistoryShortPutById {
        builder::TenantStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_stats_history_short_delete_by_id(
        &self,
    ) -> builder::TenantStatsHistoryShortDeleteById {
        builder::TenantStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_status_get(&self) -> builder::TenantStatusGet {
        builder::TenantStatusGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_status`
    ///
    ///Arguments:
    /// - `body`: tenant_status body object
    ///```ignore
    /// let response = client.tenant_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_status_post(&self) -> builder::TenantStatusPost {
        builder::TenantStatusPost::new(self)
    }
    ///Sends a `GET` request to `/tenant_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_status_get_by_id(&self) -> builder::TenantStatusGetById {
        builder::TenantStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_status body object
    ///```ignore
    /// let response = client.tenant_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_status_put_by_id(&self) -> builder::TenantStatusPutById {
        builder::TenantStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_status_delete_by_id(&self) -> builder::TenantStatusDeleteById {
        builder::TenantStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenant_storage`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenant_storage_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_storage_get(&self) -> builder::TenantStorageGet {
        builder::TenantStorageGet::new(self)
    }
    ///Sends a `POST` request to `/tenant_storage`
    ///
    ///Arguments:
    /// - `body`: tenant_storage body object
    ///```ignore
    /// let response = client.tenant_storage_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_storage_post(&self) -> builder::TenantStoragePost {
        builder::TenantStoragePost::new(self)
    }
    ///Sends a `GET` request to `/tenant_storage/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenant_storage_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_storage_get_by_id(&self) -> builder::TenantStorageGetById {
        builder::TenantStorageGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenant_storage/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenant_storage body object
    ///```ignore
    /// let response = client.tenant_storage_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_storage_put_by_id(&self) -> builder::TenantStoragePutById {
        builder::TenantStoragePutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenant_storage/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenant_storage_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenant_storage_delete_by_id(&self) -> builder::TenantStorageDeleteById {
        builder::TenantStorageDeleteById::new(self)
    }
    ///Sends a `GET` request to `/tenants`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.tenants_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenants_get(&self) -> builder::TenantsGet {
        builder::TenantsGet::new(self)
    }
    ///Sends a `POST` request to `/tenants`
    ///
    ///Arguments:
    /// - `body`: tenants body object
    ///```ignore
    /// let response = client.tenants_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenants_post(&self) -> builder::TenantsPost {
        builder::TenantsPost::new(self)
    }
    ///Sends a `GET` request to `/tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.tenants_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenants_get_by_id(&self) -> builder::TenantsGetById {
        builder::TenantsGetById::new(self)
    }
    ///Sends a `PUT` request to `/tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: tenants body object
    ///```ignore
    /// let response = client.tenants_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenants_put_by_id(&self) -> builder::TenantsPutById {
        builder::TenantsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.tenants_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn tenants_delete_by_id(&self) -> builder::TenantsDeleteById {
        builder::TenantsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/ui_branding`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.ui_branding_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ui_branding_get(&self) -> builder::UiBrandingGet {
        builder::UiBrandingGet::new(self)
    }
    ///Sends a `POST` request to `/ui_branding`
    ///
    ///Arguments:
    /// - `body`: ui_branding body object
    ///```ignore
    /// let response = client.ui_branding_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ui_branding_post(&self) -> builder::UiBrandingPost {
        builder::UiBrandingPost::new(self)
    }
    ///Sends a `GET` request to `/ui_branding/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.ui_branding_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ui_branding_get_by_id(&self) -> builder::UiBrandingGetById {
        builder::UiBrandingGetById::new(self)
    }
    ///Sends a `PUT` request to `/ui_branding/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: ui_branding body object
    ///```ignore
    /// let response = client.ui_branding_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ui_branding_put_by_id(&self) -> builder::UiBrandingPutById {
        builder::UiBrandingPutById::new(self)
    }
    ///Sends a `DELETE` request to `/ui_branding/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.ui_branding_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ui_branding_delete_by_id(&self) -> builder::UiBrandingDeleteById {
        builder::UiBrandingDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_actions_get(&self) -> builder::UpdateActionsGet {
        builder::UpdateActionsGet::new(self)
    }
    ///Sends a `POST` request to `/update_actions`
    ///
    ///Arguments:
    /// - `body`: update_actions body object
    ///```ignore
    /// let response = client.update_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_actions_post(&self) -> builder::UpdateActionsPost {
        builder::UpdateActionsPost::new(self)
    }
    ///Sends a `GET` request to `/update_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_actions_get_by_id(&self) -> builder::UpdateActionsGetById {
        builder::UpdateActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_actions body object
    ///```ignore
    /// let response = client.update_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_actions_put_by_id(&self) -> builder::UpdateActionsPutById {
        builder::UpdateActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_actions_delete_by_id(&self) -> builder::UpdateActionsDeleteById {
        builder::UpdateActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_branches`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_branches_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_branches_get(&self) -> builder::UpdateBranchesGet {
        builder::UpdateBranchesGet::new(self)
    }
    ///Sends a `POST` request to `/update_branches`
    ///
    ///Arguments:
    /// - `body`: update_branches body object
    ///```ignore
    /// let response = client.update_branches_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_branches_post(&self) -> builder::UpdateBranchesPost {
        builder::UpdateBranchesPost::new(self)
    }
    ///Sends a `GET` request to `/update_branches/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_branches_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_branches_get_by_id(&self) -> builder::UpdateBranchesGetById {
        builder::UpdateBranchesGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_branches/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_branches body object
    ///```ignore
    /// let response = client.update_branches_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_branches_put_by_id(&self) -> builder::UpdateBranchesPutById {
        builder::UpdateBranchesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_branches/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_branches_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_branches_delete_by_id(&self) -> builder::UpdateBranchesDeleteById {
        builder::UpdateBranchesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_file_finish`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_file_finish_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_finish_get(&self) -> builder::UpdateFileFinishGet {
        builder::UpdateFileFinishGet::new(self)
    }
    ///Sends a `POST` request to `/update_file_finish`
    ///
    ///Arguments:
    /// - `body`: update_file_finish body object
    ///```ignore
    /// let response = client.update_file_finish_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_finish_post(&self) -> builder::UpdateFileFinishPost {
        builder::UpdateFileFinishPost::new(self)
    }
    ///Sends a `GET` request to `/update_file_finish/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_file_finish_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_finish_get_by_id(&self) -> builder::UpdateFileFinishGetById {
        builder::UpdateFileFinishGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_file_finish/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_file_finish body object
    ///```ignore
    /// let response = client.update_file_finish_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_finish_put_by_id(&self) -> builder::UpdateFileFinishPutById {
        builder::UpdateFileFinishPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_file_finish/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_file_finish_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_finish_delete_by_id(
        &self,
    ) -> builder::UpdateFileFinishDeleteById {
        builder::UpdateFileFinishDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_file_verify`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_file_verify_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_verify_get(&self) -> builder::UpdateFileVerifyGet {
        builder::UpdateFileVerifyGet::new(self)
    }
    ///Sends a `POST` request to `/update_file_verify`
    ///
    ///Arguments:
    /// - `body`: update_file_verify body object
    ///```ignore
    /// let response = client.update_file_verify_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_verify_post(&self) -> builder::UpdateFileVerifyPost {
        builder::UpdateFileVerifyPost::new(self)
    }
    ///Sends a `GET` request to `/update_file_verify/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_file_verify_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_verify_get_by_id(&self) -> builder::UpdateFileVerifyGetById {
        builder::UpdateFileVerifyGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_file_verify/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_file_verify body object
    ///```ignore
    /// let response = client.update_file_verify_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_verify_put_by_id(&self) -> builder::UpdateFileVerifyPutById {
        builder::UpdateFileVerifyPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_file_verify/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_file_verify_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_file_verify_delete_by_id(
        &self,
    ) -> builder::UpdateFileVerifyDeleteById {
        builder::UpdateFileVerifyDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_files`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_files_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_files_get(&self) -> builder::UpdateFilesGet {
        builder::UpdateFilesGet::new(self)
    }
    ///Sends a `POST` request to `/update_files`
    ///
    ///Arguments:
    /// - `body`: update_files body object
    ///```ignore
    /// let response = client.update_files_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_files_post(&self) -> builder::UpdateFilesPost {
        builder::UpdateFilesPost::new(self)
    }
    ///Sends a `GET` request to `/update_files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_files_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_files_get_by_id(&self) -> builder::UpdateFilesGetById {
        builder::UpdateFilesGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_files body object
    ///```ignore
    /// let response = client.update_files_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_files_put_by_id(&self) -> builder::UpdateFilesPutById {
        builder::UpdateFilesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_files/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_files_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_files_delete_by_id(&self) -> builder::UpdateFilesDeleteById {
        builder::UpdateFilesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_logs_get(&self) -> builder::UpdateLogsGet {
        builder::UpdateLogsGet::new(self)
    }
    ///Sends a `POST` request to `/update_logs`
    ///
    ///Arguments:
    /// - `body`: update_logs body object
    ///```ignore
    /// let response = client.update_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_logs_post(&self) -> builder::UpdateLogsPost {
        builder::UpdateLogsPost::new(self)
    }
    ///Sends a `GET` request to `/update_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_logs_get_by_id(&self) -> builder::UpdateLogsGetById {
        builder::UpdateLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_logs body object
    ///```ignore
    /// let response = client.update_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_logs_put_by_id(&self) -> builder::UpdateLogsPutById {
        builder::UpdateLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_logs_delete_by_id(&self) -> builder::UpdateLogsDeleteById {
        builder::UpdateLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_packages`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_packages_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_packages_get(&self) -> builder::UpdatePackagesGet {
        builder::UpdatePackagesGet::new(self)
    }
    ///Sends a `POST` request to `/update_packages`
    ///
    ///Arguments:
    /// - `body`: update_packages body object
    ///```ignore
    /// let response = client.update_packages_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_packages_post(&self) -> builder::UpdatePackagesPost {
        builder::UpdatePackagesPost::new(self)
    }
    ///Sends a `GET` request to `/update_packages/{name}`
    ///
    ///Arguments:
    /// - `name`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_packages_get_by_name()
    ///    .name(name)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_packages_get_by_name(&self) -> builder::UpdatePackagesGetByName {
        builder::UpdatePackagesGetByName::new(self)
    }
    ///Sends a `PUT` request to `/update_packages/{name}`
    ///
    ///Arguments:
    /// - `name`: resource id
    /// - `body`: update_packages body object
    ///```ignore
    /// let response = client.update_packages_put_by_name()
    ///    .name(name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_packages_put_by_name(&self) -> builder::UpdatePackagesPutByName {
        builder::UpdatePackagesPutByName::new(self)
    }
    ///Sends a `DELETE` request to `/update_packages/{name}`
    ///
    ///Arguments:
    /// - `name`: resource id
    ///```ignore
    /// let response = client.update_packages_delete_by_name()
    ///    .name(name)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_packages_delete_by_name(&self) -> builder::UpdatePackagesDeleteByName {
        builder::UpdatePackagesDeleteByName::new(self)
    }
    ///Sends a `GET` request to `/update_settings`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_settings_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_settings_get(&self) -> builder::UpdateSettingsGet {
        builder::UpdateSettingsGet::new(self)
    }
    ///Sends a `POST` request to `/update_settings`
    ///
    ///Arguments:
    /// - `body`: update_settings body object
    ///```ignore
    /// let response = client.update_settings_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_settings_post(&self) -> builder::UpdateSettingsPost {
        builder::UpdateSettingsPost::new(self)
    }
    ///Sends a `GET` request to `/update_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_settings_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_settings_get_by_id(&self) -> builder::UpdateSettingsGetById {
        builder::UpdateSettingsGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_settings body object
    ///```ignore
    /// let response = client.update_settings_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_settings_put_by_id(&self) -> builder::UpdateSettingsPutById {
        builder::UpdateSettingsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_settings_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_settings_delete_by_id(&self) -> builder::UpdateSettingsDeleteById {
        builder::UpdateSettingsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_source_packages`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_source_packages_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_packages_get(&self) -> builder::UpdateSourcePackagesGet {
        builder::UpdateSourcePackagesGet::new(self)
    }
    ///Sends a `POST` request to `/update_source_packages`
    ///
    ///Arguments:
    /// - `body`: update_source_packages body object
    ///```ignore
    /// let response = client.update_source_packages_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_packages_post(&self) -> builder::UpdateSourcePackagesPost {
        builder::UpdateSourcePackagesPost::new(self)
    }
    ///Sends a `GET` request to `/update_source_packages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_source_packages_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_packages_get_by_id(
        &self,
    ) -> builder::UpdateSourcePackagesGetById {
        builder::UpdateSourcePackagesGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_source_packages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_source_packages body object
    ///```ignore
    /// let response = client.update_source_packages_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_packages_put_by_id(
        &self,
    ) -> builder::UpdateSourcePackagesPutById {
        builder::UpdateSourcePackagesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_source_packages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_source_packages_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_packages_delete_by_id(
        &self,
    ) -> builder::UpdateSourcePackagesDeleteById {
        builder::UpdateSourcePackagesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_source_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_source_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_status_get(&self) -> builder::UpdateSourceStatusGet {
        builder::UpdateSourceStatusGet::new(self)
    }
    ///Sends a `POST` request to `/update_source_status`
    ///
    ///Arguments:
    /// - `body`: update_source_status body object
    ///```ignore
    /// let response = client.update_source_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_status_post(&self) -> builder::UpdateSourceStatusPost {
        builder::UpdateSourceStatusPost::new(self)
    }
    ///Sends a `GET` request to `/update_source_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_source_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_status_get_by_id(&self) -> builder::UpdateSourceStatusGetById {
        builder::UpdateSourceStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_source_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_source_status body object
    ///```ignore
    /// let response = client.update_source_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_status_put_by_id(&self) -> builder::UpdateSourceStatusPutById {
        builder::UpdateSourceStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_source_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_source_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_source_status_delete_by_id(
        &self,
    ) -> builder::UpdateSourceStatusDeleteById {
        builder::UpdateSourceStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/update_sources`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.update_sources_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_sources_get(&self) -> builder::UpdateSourcesGet {
        builder::UpdateSourcesGet::new(self)
    }
    ///Sends a `POST` request to `/update_sources`
    ///
    ///Arguments:
    /// - `body`: update_sources body object
    ///```ignore
    /// let response = client.update_sources_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_sources_post(&self) -> builder::UpdateSourcesPost {
        builder::UpdateSourcesPost::new(self)
    }
    ///Sends a `GET` request to `/update_sources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.update_sources_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_sources_get_by_id(&self) -> builder::UpdateSourcesGetById {
        builder::UpdateSourcesGetById::new(self)
    }
    ///Sends a `PUT` request to `/update_sources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: update_sources body object
    ///```ignore
    /// let response = client.update_sources_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_sources_put_by_id(&self) -> builder::UpdateSourcesPutById {
        builder::UpdateSourcesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/update_sources/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.update_sources_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_sources_delete_by_id(&self) -> builder::UpdateSourcesDeleteById {
        builder::UpdateSourcesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/user`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.user_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_get(&self) -> builder::UserGet {
        builder::UserGet::new(self)
    }
    ///Sends a `POST` request to `/user`
    ///
    ///Arguments:
    /// - `body`: user body object
    ///```ignore
    /// let response = client.user_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_post(&self) -> builder::UserPost {
        builder::UserPost::new(self)
    }
    ///Sends a `GET` request to `/user/{request}`
    ///
    ///Arguments:
    /// - `request`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.user_get_by_request()
    ///    .request(request)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_get_by_request(&self) -> builder::UserGetByRequest {
        builder::UserGetByRequest::new(self)
    }
    ///Sends a `PUT` request to `/user/{request}`
    ///
    ///Arguments:
    /// - `request`: resource id
    /// - `body`: user body object
    ///```ignore
    /// let response = client.user_put_by_request()
    ///    .request(request)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_put_by_request(&self) -> builder::UserPutByRequest {
        builder::UserPutByRequest::new(self)
    }
    ///Sends a `DELETE` request to `/user/{request}`
    ///
    ///Arguments:
    /// - `request`: resource id
    ///```ignore
    /// let response = client.user_delete_by_request()
    ///    .request(request)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_delete_by_request(&self) -> builder::UserDeleteByRequest {
        builder::UserDeleteByRequest::new(self)
    }
    ///Sends a `GET` request to `/user_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.user_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_actions_get(&self) -> builder::UserActionsGet {
        builder::UserActionsGet::new(self)
    }
    ///Sends a `POST` request to `/user_actions`
    ///
    ///Arguments:
    /// - `body`: user_actions body object
    ///```ignore
    /// let response = client.user_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_actions_post(&self) -> builder::UserActionsPost {
        builder::UserActionsPost::new(self)
    }
    ///Sends a `GET` request to `/user_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.user_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_actions_get_by_id(&self) -> builder::UserActionsGetById {
        builder::UserActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/user_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: user_actions body object
    ///```ignore
    /// let response = client.user_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_actions_put_by_id(&self) -> builder::UserActionsPutById {
        builder::UserActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/user_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.user_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_actions_delete_by_id(&self) -> builder::UserActionsDeleteById {
        builder::UserActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/user_devices`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.user_devices_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_devices_get(&self) -> builder::UserDevicesGet {
        builder::UserDevicesGet::new(self)
    }
    ///Sends a `POST` request to `/user_devices`
    ///
    ///Arguments:
    /// - `body`: user_devices body object
    ///```ignore
    /// let response = client.user_devices_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_devices_post(&self) -> builder::UserDevicesPost {
        builder::UserDevicesPost::new(self)
    }
    ///Sends a `GET` request to `/user_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.user_devices_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_devices_get_by_id(&self) -> builder::UserDevicesGetById {
        builder::UserDevicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/user_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: user_devices body object
    ///```ignore
    /// let response = client.user_devices_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_devices_put_by_id(&self) -> builder::UserDevicesPutById {
        builder::UserDevicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/user_devices/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.user_devices_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_devices_delete_by_id(&self) -> builder::UserDevicesDeleteById {
        builder::UserDevicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/user_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.user_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_logs_get(&self) -> builder::UserLogsGet {
        builder::UserLogsGet::new(self)
    }
    ///Sends a `POST` request to `/user_logs`
    ///
    ///Arguments:
    /// - `body`: user_logs body object
    ///```ignore
    /// let response = client.user_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_logs_post(&self) -> builder::UserLogsPost {
        builder::UserLogsPost::new(self)
    }
    ///Sends a `GET` request to `/user_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.user_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_logs_get_by_id(&self) -> builder::UserLogsGetById {
        builder::UserLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/user_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: user_logs body object
    ///```ignore
    /// let response = client.user_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_logs_put_by_id(&self) -> builder::UserLogsPutById {
        builder::UserLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/user_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.user_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_logs_delete_by_id(&self) -> builder::UserLogsDeleteById {
        builder::UserLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/user_messages`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.user_messages_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_messages_get(&self) -> builder::UserMessagesGet {
        builder::UserMessagesGet::new(self)
    }
    ///Sends a `POST` request to `/user_messages`
    ///
    ///Arguments:
    /// - `body`: user_messages body object
    ///```ignore
    /// let response = client.user_messages_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_messages_post(&self) -> builder::UserMessagesPost {
        builder::UserMessagesPost::new(self)
    }
    ///Sends a `GET` request to `/user_messages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.user_messages_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_messages_get_by_id(&self) -> builder::UserMessagesGetById {
        builder::UserMessagesGetById::new(self)
    }
    ///Sends a `PUT` request to `/user_messages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: user_messages body object
    ///```ignore
    /// let response = client.user_messages_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_messages_put_by_id(&self) -> builder::UserMessagesPutById {
        builder::UserMessagesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/user_messages/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.user_messages_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_messages_delete_by_id(&self) -> builder::UserMessagesDeleteById {
        builder::UserMessagesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/user_settings`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.user_settings_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_settings_get(&self) -> builder::UserSettingsGet {
        builder::UserSettingsGet::new(self)
    }
    ///Sends a `POST` request to `/user_settings`
    ///
    ///Arguments:
    /// - `body`: user_settings body object
    ///```ignore
    /// let response = client.user_settings_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_settings_post(&self) -> builder::UserSettingsPost {
        builder::UserSettingsPost::new(self)
    }
    ///Sends a `GET` request to `/user_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.user_settings_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_settings_get_by_id(&self) -> builder::UserSettingsGetById {
        builder::UserSettingsGetById::new(self)
    }
    ///Sends a `PUT` request to `/user_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: user_settings body object
    ///```ignore
    /// let response = client.user_settings_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_settings_put_by_id(&self) -> builder::UserSettingsPutById {
        builder::UserSettingsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/user_settings/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.user_settings_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_settings_delete_by_id(&self) -> builder::UserSettingsDeleteById {
        builder::UserSettingsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/users`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.users_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn users_get(&self) -> builder::UsersGet {
        builder::UsersGet::new(self)
    }
    ///Sends a `POST` request to `/users`
    ///
    ///Arguments:
    /// - `body`: users body object
    ///```ignore
    /// let response = client.users_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn users_post(&self) -> builder::UsersPost {
        builder::UsersPost::new(self)
    }
    ///Sends a `GET` request to `/users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.users_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn users_get_by_id(&self) -> builder::UsersGetById {
        builder::UsersGetById::new(self)
    }
    ///Sends a `PUT` request to `/users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: users body object
    ///```ignore
    /// let response = client.users_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn users_put_by_id(&self) -> builder::UsersPutById {
        builder::UsersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.users_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn users_delete_by_id(&self) -> builder::UsersDeleteById {
        builder::UsersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/v3_vm_import`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.v3_vm_import_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn v3_vm_import_get(&self) -> builder::V3VmImportGet {
        builder::V3VmImportGet::new(self)
    }
    ///Sends a `POST` request to `/v3_vm_import`
    ///
    ///Arguments:
    /// - `body`: v3_vm_import body object
    ///```ignore
    /// let response = client.v3_vm_import_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn v3_vm_import_post(&self) -> builder::V3VmImportPost {
        builder::V3VmImportPost::new(self)
    }
    ///Sends a `GET` request to `/v3_vm_import/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.v3_vm_import_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn v3_vm_import_get_by_id(&self) -> builder::V3VmImportGetById {
        builder::V3VmImportGetById::new(self)
    }
    ///Sends a `PUT` request to `/v3_vm_import/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: v3_vm_import body object
    ///```ignore
    /// let response = client.v3_vm_import_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn v3_vm_import_put_by_id(&self) -> builder::V3VmImportPutById {
        builder::V3VmImportPutById::new(self)
    }
    ///Sends a `DELETE` request to `/v3_vm_import/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.v3_vm_import_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn v3_vm_import_delete_by_id(&self) -> builder::V3VmImportDeleteById {
        builder::V3VmImportDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_actions_get(&self) -> builder::VmActionsGet {
        builder::VmActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_actions`
    ///
    ///Arguments:
    /// - `body`: vm_actions body object
    ///```ignore
    /// let response = client.vm_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_actions_post(&self) -> builder::VmActionsPost {
        builder::VmActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_actions_get_by_id(&self) -> builder::VmActionsGetById {
        builder::VmActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_actions body object
    ///```ignore
    /// let response = client.vm_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_actions_put_by_id(&self) -> builder::VmActionsPutById {
        builder::VmActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_actions_delete_by_id(&self) -> builder::VmActionsDeleteById {
        builder::VmActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_console_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_console_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_console_actions_get(&self) -> builder::VmConsoleActionsGet {
        builder::VmConsoleActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_console_actions`
    ///
    ///Arguments:
    /// - `body`: vm_console_actions body object
    ///```ignore
    /// let response = client.vm_console_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_console_actions_post(&self) -> builder::VmConsoleActionsPost {
        builder::VmConsoleActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_console_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_console_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_console_actions_get_by_id(&self) -> builder::VmConsoleActionsGetById {
        builder::VmConsoleActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_console_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_console_actions body object
    ///```ignore
    /// let response = client.vm_console_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_console_actions_put_by_id(&self) -> builder::VmConsoleActionsPutById {
        builder::VmConsoleActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_console_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_console_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_console_actions_delete_by_id(
        &self,
    ) -> builder::VmConsoleActionsDeleteById {
        builder::VmConsoleActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_favorites`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_favorites_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_favorites_get(&self) -> builder::VmFavoritesGet {
        builder::VmFavoritesGet::new(self)
    }
    ///Sends a `POST` request to `/vm_favorites`
    ///
    ///Arguments:
    /// - `body`: vm_favorites body object
    ///```ignore
    /// let response = client.vm_favorites_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_favorites_post(&self) -> builder::VmFavoritesPost {
        builder::VmFavoritesPost::new(self)
    }
    ///Sends a `GET` request to `/vm_favorites/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_favorites_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_favorites_get_by_id(&self) -> builder::VmFavoritesGetById {
        builder::VmFavoritesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_favorites/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_favorites_put_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_favorites_put_by_id(&self) -> builder::VmFavoritesPutById {
        builder::VmFavoritesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_favorites/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_favorites_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_favorites_delete_by_id(&self) -> builder::VmFavoritesDeleteById {
        builder::VmFavoritesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_import_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_import_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_import_logs_get(&self) -> builder::VmImportLogsGet {
        builder::VmImportLogsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_import_logs`
    ///
    ///Arguments:
    /// - `body`: vm_import_logs body object
    ///```ignore
    /// let response = client.vm_import_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_import_logs_post(&self) -> builder::VmImportLogsPost {
        builder::VmImportLogsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_import_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_import_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_import_logs_get_by_id(&self) -> builder::VmImportLogsGetById {
        builder::VmImportLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_import_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_import_logs body object
    ///```ignore
    /// let response = client.vm_import_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_import_logs_put_by_id(&self) -> builder::VmImportLogsPutById {
        builder::VmImportLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_import_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_import_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_import_logs_delete_by_id(&self) -> builder::VmImportLogsDeleteById {
        builder::VmImportLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_imports`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_imports_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_imports_get(&self) -> builder::VmImportsGet {
        builder::VmImportsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_imports`
    ///
    ///Arguments:
    /// - `body`: vm_imports body object
    ///```ignore
    /// let response = client.vm_imports_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_imports_post(&self) -> builder::VmImportsPost {
        builder::VmImportsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_imports/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_imports_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_imports_get_by_id(&self) -> builder::VmImportsGetById {
        builder::VmImportsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_imports/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_imports body object
    ///```ignore
    /// let response = client.vm_imports_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_imports_put_by_id(&self) -> builder::VmImportsPutById {
        builder::VmImportsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_imports/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_imports_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_imports_delete_by_id(&self) -> builder::VmImportsDeleteById {
        builder::VmImportsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_paste_configs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_paste_configs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_paste_configs_get(&self) -> builder::VmPasteConfigsGet {
        builder::VmPasteConfigsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_paste_configs`
    ///
    ///Arguments:
    /// - `body`: vm_paste_configs body object
    ///```ignore
    /// let response = client.vm_paste_configs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_paste_configs_post(&self) -> builder::VmPasteConfigsPost {
        builder::VmPasteConfigsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_paste_configs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_paste_configs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_paste_configs_get_by_id(&self) -> builder::VmPasteConfigsGetById {
        builder::VmPasteConfigsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_paste_configs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_paste_configs body object
    ///```ignore
    /// let response = client.vm_paste_configs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_paste_configs_put_by_id(&self) -> builder::VmPasteConfigsPutById {
        builder::VmPasteConfigsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_paste_configs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_paste_configs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_paste_configs_delete_by_id(&self) -> builder::VmPasteConfigsDeleteById {
        builder::VmPasteConfigsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_recipe_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_recipe_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_actions_get(&self) -> builder::VmRecipeActionsGet {
        builder::VmRecipeActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_recipe_actions`
    ///
    ///Arguments:
    /// - `body`: vm_recipe_actions body object
    ///```ignore
    /// let response = client.vm_recipe_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_actions_post(&self) -> builder::VmRecipeActionsPost {
        builder::VmRecipeActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_recipe_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_recipe_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_actions_get_by_id(&self) -> builder::VmRecipeActionsGetById {
        builder::VmRecipeActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_recipe_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_recipe_actions body object
    ///```ignore
    /// let response = client.vm_recipe_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_actions_put_by_id(&self) -> builder::VmRecipeActionsPutById {
        builder::VmRecipeActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_recipe_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_recipe_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_actions_delete_by_id(&self) -> builder::VmRecipeActionsDeleteById {
        builder::VmRecipeActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_recipe_instances`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_recipe_instances_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_instances_get(&self) -> builder::VmRecipeInstancesGet {
        builder::VmRecipeInstancesGet::new(self)
    }
    ///Sends a `POST` request to `/vm_recipe_instances`
    ///
    ///Arguments:
    /// - `body`: vm_recipe_instances body object
    ///```ignore
    /// let response = client.vm_recipe_instances_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_instances_post(&self) -> builder::VmRecipeInstancesPost {
        builder::VmRecipeInstancesPost::new(self)
    }
    ///Sends a `GET` request to `/vm_recipe_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_recipe_instances_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_instances_get_by_id(&self) -> builder::VmRecipeInstancesGetById {
        builder::VmRecipeInstancesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_recipe_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_recipe_instances body object
    ///```ignore
    /// let response = client.vm_recipe_instances_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_instances_put_by_id(&self) -> builder::VmRecipeInstancesPutById {
        builder::VmRecipeInstancesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_recipe_instances/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_recipe_instances_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_instances_delete_by_id(
        &self,
    ) -> builder::VmRecipeInstancesDeleteById {
        builder::VmRecipeInstancesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_recipe_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_recipe_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_logs_get(&self) -> builder::VmRecipeLogsGet {
        builder::VmRecipeLogsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_recipe_logs`
    ///
    ///Arguments:
    /// - `body`: vm_recipe_logs body object
    ///```ignore
    /// let response = client.vm_recipe_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_logs_post(&self) -> builder::VmRecipeLogsPost {
        builder::VmRecipeLogsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_recipe_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_recipe_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_logs_get_by_id(&self) -> builder::VmRecipeLogsGetById {
        builder::VmRecipeLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_recipe_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_recipe_logs body object
    ///```ignore
    /// let response = client.vm_recipe_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_logs_put_by_id(&self) -> builder::VmRecipeLogsPutById {
        builder::VmRecipeLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_recipe_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_recipe_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipe_logs_delete_by_id(&self) -> builder::VmRecipeLogsDeleteById {
        builder::VmRecipeLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_recipes`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_recipes_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipes_get(&self) -> builder::VmRecipesGet {
        builder::VmRecipesGet::new(self)
    }
    ///Sends a `POST` request to `/vm_recipes`
    ///
    ///Arguments:
    /// - `body`: vm_recipes body object
    ///```ignore
    /// let response = client.vm_recipes_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipes_post(&self) -> builder::VmRecipesPost {
        builder::VmRecipesPost::new(self)
    }
    ///Sends a `GET` request to `/vm_recipes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_recipes_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipes_get_by_id(&self) -> builder::VmRecipesGetById {
        builder::VmRecipesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_recipes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_recipes body object
    ///```ignore
    /// let response = client.vm_recipes_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipes_put_by_id(&self) -> builder::VmRecipesPutById {
        builder::VmRecipesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_recipes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_recipes_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_recipes_delete_by_id(&self) -> builder::VmRecipesDeleteById {
        builder::VmRecipesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_antivirus`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_antivirus_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_antivirus_get(&self) -> builder::VmServiceAntivirusGet {
        builder::VmServiceAntivirusGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_antivirus`
    ///
    ///Arguments:
    /// - `body`: vm_service_antivirus body object
    ///```ignore
    /// let response = client.vm_service_antivirus_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_antivirus_post(&self) -> builder::VmServiceAntivirusPost {
        builder::VmServiceAntivirusPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_antivirus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_antivirus_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_antivirus_get_by_id(&self) -> builder::VmServiceAntivirusGetById {
        builder::VmServiceAntivirusGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_antivirus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_antivirus body object
    ///```ignore
    /// let response = client.vm_service_antivirus_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_antivirus_put_by_id(&self) -> builder::VmServiceAntivirusPutById {
        builder::VmServiceAntivirusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_antivirus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_antivirus_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_antivirus_delete_by_id(
        &self,
    ) -> builder::VmServiceAntivirusDeleteById {
        builder::VmServiceAntivirusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_cifs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_cifs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_cifs_get(&self) -> builder::VmServiceCifsGet {
        builder::VmServiceCifsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_cifs`
    ///
    ///Arguments:
    /// - `body`: vm_service_cifs body object
    ///```ignore
    /// let response = client.vm_service_cifs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_cifs_post(&self) -> builder::VmServiceCifsPost {
        builder::VmServiceCifsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_cifs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_cifs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_cifs_get_by_id(&self) -> builder::VmServiceCifsGetById {
        builder::VmServiceCifsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_cifs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_cifs body object
    ///```ignore
    /// let response = client.vm_service_cifs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_cifs_put_by_id(&self) -> builder::VmServiceCifsPutById {
        builder::VmServiceCifsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_cifs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_cifs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_cifs_delete_by_id(&self) -> builder::VmServiceCifsDeleteById {
        builder::VmServiceCifsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_nfs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_nfs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_nfs_get(&self) -> builder::VmServiceNfsGet {
        builder::VmServiceNfsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_nfs`
    ///
    ///Arguments:
    /// - `body`: vm_service_nfs body object
    ///```ignore
    /// let response = client.vm_service_nfs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_nfs_post(&self) -> builder::VmServiceNfsPost {
        builder::VmServiceNfsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_nfs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_nfs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_nfs_get_by_id(&self) -> builder::VmServiceNfsGetById {
        builder::VmServiceNfsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_nfs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_nfs body object
    ///```ignore
    /// let response = client.vm_service_nfs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_nfs_put_by_id(&self) -> builder::VmServiceNfsPutById {
        builder::VmServiceNfsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_nfs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_nfs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_nfs_delete_by_id(&self) -> builder::VmServiceNfsDeleteById {
        builder::VmServiceNfsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_queries`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_queries_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_queries_get(&self) -> builder::VmServiceQueriesGet {
        builder::VmServiceQueriesGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_queries`
    ///
    ///Arguments:
    /// - `body`: vm_service_queries body object
    ///```ignore
    /// let response = client.vm_service_queries_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_queries_post(&self) -> builder::VmServiceQueriesPost {
        builder::VmServiceQueriesPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_queries_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_queries_get_by_id(&self) -> builder::VmServiceQueriesGetById {
        builder::VmServiceQueriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_queries body object
    ///```ignore
    /// let response = client.vm_service_queries_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_queries_put_by_id(&self) -> builder::VmServiceQueriesPutById {
        builder::VmServiceQueriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_queries_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_queries_delete_by_id(
        &self,
    ) -> builder::VmServiceQueriesDeleteById {
        builder::VmServiceQueriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_user_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_user_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_logs_get(&self) -> builder::VmServiceUserLogsGet {
        builder::VmServiceUserLogsGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_user_logs`
    ///
    ///Arguments:
    /// - `body`: vm_service_user_logs body object
    ///```ignore
    /// let response = client.vm_service_user_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_logs_post(&self) -> builder::VmServiceUserLogsPost {
        builder::VmServiceUserLogsPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_user_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_user_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_logs_get_by_id(&self) -> builder::VmServiceUserLogsGetById {
        builder::VmServiceUserLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_user_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_user_logs body object
    ///```ignore
    /// let response = client.vm_service_user_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_logs_put_by_id(&self) -> builder::VmServiceUserLogsPutById {
        builder::VmServiceUserLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_user_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_user_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_logs_delete_by_id(
        &self,
    ) -> builder::VmServiceUserLogsDeleteById {
        builder::VmServiceUserLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_user_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_user_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_status_get(&self) -> builder::VmServiceUserStatusGet {
        builder::VmServiceUserStatusGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_user_status`
    ///
    ///Arguments:
    /// - `body`: vm_service_user_status body object
    ///```ignore
    /// let response = client.vm_service_user_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_status_post(&self) -> builder::VmServiceUserStatusPost {
        builder::VmServiceUserStatusPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_user_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_user_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_status_get_by_id(
        &self,
    ) -> builder::VmServiceUserStatusGetById {
        builder::VmServiceUserStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_user_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_user_status body object
    ///```ignore
    /// let response = client.vm_service_user_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_status_put_by_id(
        &self,
    ) -> builder::VmServiceUserStatusPutById {
        builder::VmServiceUserStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_user_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_user_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_user_status_delete_by_id(
        &self,
    ) -> builder::VmServiceUserStatusDeleteById {
        builder::VmServiceUserStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_service_users`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_service_users_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_users_get(&self) -> builder::VmServiceUsersGet {
        builder::VmServiceUsersGet::new(self)
    }
    ///Sends a `POST` request to `/vm_service_users`
    ///
    ///Arguments:
    /// - `body`: vm_service_users body object
    ///```ignore
    /// let response = client.vm_service_users_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_users_post(&self) -> builder::VmServiceUsersPost {
        builder::VmServiceUsersPost::new(self)
    }
    ///Sends a `GET` request to `/vm_service_users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_service_users_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_users_get_by_id(&self) -> builder::VmServiceUsersGetById {
        builder::VmServiceUsersGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_service_users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_service_users body object
    ///```ignore
    /// let response = client.vm_service_users_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_users_put_by_id(&self) -> builder::VmServiceUsersPutById {
        builder::VmServiceUsersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_service_users/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_service_users_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_service_users_delete_by_id(&self) -> builder::VmServiceUsersDeleteById {
        builder::VmServiceUsersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vm_services`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vm_services_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_services_get(&self) -> builder::VmServicesGet {
        builder::VmServicesGet::new(self)
    }
    ///Sends a `POST` request to `/vm_services`
    ///
    ///Arguments:
    /// - `body`: vm_services body object
    ///```ignore
    /// let response = client.vm_services_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_services_post(&self) -> builder::VmServicesPost {
        builder::VmServicesPost::new(self)
    }
    ///Sends a `GET` request to `/vm_services/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vm_services_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_services_get_by_id(&self) -> builder::VmServicesGetById {
        builder::VmServicesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vm_services/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vm_services body object
    ///```ignore
    /// let response = client.vm_services_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_services_put_by_id(&self) -> builder::VmServicesPutById {
        builder::VmServicesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vm_services/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vm_services_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vm_services_delete_by_id(&self) -> builder::VmServicesDeleteById {
        builder::VmServicesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vsan_queries`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vsan_queries_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vsan_queries_get(&self) -> builder::VsanQueriesGet {
        builder::VsanQueriesGet::new(self)
    }
    ///Sends a `POST` request to `/vsan_queries`
    ///
    ///Arguments:
    /// - `body`: vsan_queries body object
    ///```ignore
    /// let response = client.vsan_queries_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vsan_queries_post(&self) -> builder::VsanQueriesPost {
        builder::VsanQueriesPost::new(self)
    }
    ///Sends a `GET` request to `/vsan_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vsan_queries_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vsan_queries_get_by_id(&self) -> builder::VsanQueriesGetById {
        builder::VsanQueriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vsan_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vsan_queries body object
    ///```ignore
    /// let response = client.vsan_queries_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vsan_queries_put_by_id(&self) -> builder::VsanQueriesPutById {
        builder::VsanQueriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vsan_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vsan_queries_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vsan_queries_delete_by_id(&self) -> builder::VsanQueriesDeleteById {
        builder::VsanQueriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_container_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_actions_get(&self) -> builder::VmwareContainerActionsGet {
        builder::VmwareContainerActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_container_actions`
    ///
    ///Arguments:
    /// - `body`: vmware_container_actions body object
    ///```ignore
    /// let response = client.vmware_container_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_actions_post(&self) -> builder::VmwareContainerActionsPost {
        builder::VmwareContainerActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_container_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_actions_get_by_id(
        &self,
    ) -> builder::VmwareContainerActionsGetById {
        builder::VmwareContainerActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_container_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_container_actions body object
    ///```ignore
    /// let response = client.vmware_container_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_actions_put_by_id(
        &self,
    ) -> builder::VmwareContainerActionsPutById {
        builder::VmwareContainerActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_container_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_container_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_actions_delete_by_id(
        &self,
    ) -> builder::VmwareContainerActionsDeleteById {
        builder::VmwareContainerActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_api`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_container_api_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_api_get(&self) -> builder::VmwareContainerApiGet {
        builder::VmwareContainerApiGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_container_api`
    ///
    ///Arguments:
    /// - `body`: vmware_container_api body object
    ///```ignore
    /// let response = client.vmware_container_api_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_api_post(&self) -> builder::VmwareContainerApiPost {
        builder::VmwareContainerApiPost::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_api/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_container_api_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_api_get_by_id(&self) -> builder::VmwareContainerApiGetById {
        builder::VmwareContainerApiGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_container_api/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_container_api body object
    ///```ignore
    /// let response = client.vmware_container_api_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_api_put_by_id(&self) -> builder::VmwareContainerApiPutById {
        builder::VmwareContainerApiPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_container_api/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_container_api_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_api_delete_by_id(
        &self,
    ) -> builder::VmwareContainerApiDeleteById {
        builder::VmwareContainerApiDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_backup_job_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_container_backup_job_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_backup_job_actions_get(
        &self,
    ) -> builder::VmwareContainerBackupJobActionsGet {
        builder::VmwareContainerBackupJobActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_container_backup_job_actions`
    ///
    ///Arguments:
    /// - `body`: vmware_container_backup_job_actions body object
    ///```ignore
    /// let response = client.vmware_container_backup_job_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_backup_job_actions_post(
        &self,
    ) -> builder::VmwareContainerBackupJobActionsPost {
        builder::VmwareContainerBackupJobActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_backup_job_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_container_backup_job_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_backup_job_actions_get_by_id(
        &self,
    ) -> builder::VmwareContainerBackupJobActionsGetById {
        builder::VmwareContainerBackupJobActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_container_backup_job_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_container_backup_job_actions body object
    ///```ignore
    /// let response = client.vmware_container_backup_job_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_backup_job_actions_put_by_id(
        &self,
    ) -> builder::VmwareContainerBackupJobActionsPutById {
        builder::VmwareContainerBackupJobActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_container_backup_job_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_container_backup_job_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_backup_job_actions_delete_by_id(
        &self,
    ) -> builder::VmwareContainerBackupJobActionsDeleteById {
        builder::VmwareContainerBackupJobActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_queries`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_container_queries_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_queries_get(&self) -> builder::VmwareContainerQueriesGet {
        builder::VmwareContainerQueriesGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_container_queries`
    ///
    ///Arguments:
    /// - `body`: vmware_container_queries body object
    ///```ignore
    /// let response = client.vmware_container_queries_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_queries_post(&self) -> builder::VmwareContainerQueriesPost {
        builder::VmwareContainerQueriesPost::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_container_queries_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_queries_get_by_id(
        &self,
    ) -> builder::VmwareContainerQueriesGetById {
        builder::VmwareContainerQueriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_container_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_container_queries body object
    ///```ignore
    /// let response = client.vmware_container_queries_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_queries_put_by_id(
        &self,
    ) -> builder::VmwareContainerQueriesPutById {
        builder::VmwareContainerQueriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_container_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_container_queries_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_queries_delete_by_id(
        &self,
    ) -> builder::VmwareContainerQueriesDeleteById {
        builder::VmwareContainerQueriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_restore_job_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_container_restore_job_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_restore_job_actions_get(
        &self,
    ) -> builder::VmwareContainerRestoreJobActionsGet {
        builder::VmwareContainerRestoreJobActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_container_restore_job_actions`
    ///
    ///Arguments:
    /// - `body`: vmware_container_restore_job_actions body object
    ///```ignore
    /// let response = client.vmware_container_restore_job_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_restore_job_actions_post(
        &self,
    ) -> builder::VmwareContainerRestoreJobActionsPost {
        builder::VmwareContainerRestoreJobActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_restore_job_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_container_restore_job_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_restore_job_actions_get_by_id(
        &self,
    ) -> builder::VmwareContainerRestoreJobActionsGetById {
        builder::VmwareContainerRestoreJobActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_container_restore_job_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_container_restore_job_actions body object
    ///```ignore
    /// let response = client.vmware_container_restore_job_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_restore_job_actions_put_by_id(
        &self,
    ) -> builder::VmwareContainerRestoreJobActionsPutById {
        builder::VmwareContainerRestoreJobActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_container_restore_job_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_container_restore_job_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_restore_job_actions_delete_by_id(
        &self,
    ) -> builder::VmwareContainerRestoreJobActionsDeleteById {
        builder::VmwareContainerRestoreJobActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_storage`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_container_storage_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_storage_get(&self) -> builder::VmwareContainerStorageGet {
        builder::VmwareContainerStorageGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_container_storage`
    ///
    ///Arguments:
    /// - `body`: vmware_container_storage body object
    ///```ignore
    /// let response = client.vmware_container_storage_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_storage_post(&self) -> builder::VmwareContainerStoragePost {
        builder::VmwareContainerStoragePost::new(self)
    }
    ///Sends a `GET` request to `/vmware_container_storage/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_container_storage_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_storage_get_by_id(
        &self,
    ) -> builder::VmwareContainerStorageGetById {
        builder::VmwareContainerStorageGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_container_storage/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_container_storage body object
    ///```ignore
    /// let response = client.vmware_container_storage_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_storage_put_by_id(
        &self,
    ) -> builder::VmwareContainerStoragePutById {
        builder::VmwareContainerStoragePutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_container_storage/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_container_storage_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_container_storage_delete_by_id(
        &self,
    ) -> builder::VmwareContainerStorageDeleteById {
        builder::VmwareContainerStorageDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vmware_containers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vmware_containers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_containers_get(&self) -> builder::VmwareContainersGet {
        builder::VmwareContainersGet::new(self)
    }
    ///Sends a `POST` request to `/vmware_containers`
    ///
    ///Arguments:
    /// - `body`: vmware_containers body object
    ///```ignore
    /// let response = client.vmware_containers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_containers_post(&self) -> builder::VmwareContainersPost {
        builder::VmwareContainersPost::new(self)
    }
    ///Sends a `GET` request to `/vmware_containers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vmware_containers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_containers_get_by_id(&self) -> builder::VmwareContainersGetById {
        builder::VmwareContainersGetById::new(self)
    }
    ///Sends a `PUT` request to `/vmware_containers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vmware_containers body object
    ///```ignore
    /// let response = client.vmware_containers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_containers_put_by_id(&self) -> builder::VmwareContainersPutById {
        builder::VmwareContainersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vmware_containers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vmware_containers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vmware_containers_delete_by_id(&self) -> builder::VmwareContainersDeleteById {
        builder::VmwareContainersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_actions_get(&self) -> builder::VnetActionsGet {
        builder::VnetActionsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_actions`
    ///
    ///Arguments:
    /// - `body`: vnet_actions body object
    ///```ignore
    /// let response = client.vnet_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_actions_post(&self) -> builder::VnetActionsPost {
        builder::VnetActionsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_actions_get_by_id(&self) -> builder::VnetActionsGetById {
        builder::VnetActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_actions body object
    ///```ignore
    /// let response = client.vnet_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_actions_put_by_id(&self) -> builder::VnetActionsPutById {
        builder::VnetActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_actions_delete_by_id(&self) -> builder::VnetActionsDeleteById {
        builder::VnetActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_addresses`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_addresses_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_addresses_get(&self) -> builder::VnetAddressesGet {
        builder::VnetAddressesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_addresses`
    ///
    ///Arguments:
    /// - `body`: vnet_addresses body object
    ///```ignore
    /// let response = client.vnet_addresses_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_addresses_post(&self) -> builder::VnetAddressesPost {
        builder::VnetAddressesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_addresses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_addresses_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_addresses_get_by_id(&self) -> builder::VnetAddressesGetById {
        builder::VnetAddressesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_addresses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_addresses body object
    ///```ignore
    /// let response = client.vnet_addresses_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_addresses_put_by_id(&self) -> builder::VnetAddressesPutById {
        builder::VnetAddressesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_addresses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_addresses_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_addresses_delete_by_id(&self) -> builder::VnetAddressesDeleteById {
        builder::VnetAddressesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_get(&self) -> builder::VnetBgpGet {
        builder::VnetBgpGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp body object
    ///```ignore
    /// let response = client.vnet_bgp_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_post(&self) -> builder::VnetBgpPost {
        builder::VnetBgpPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_get_by_id(&self) -> builder::VnetBgpGetById {
        builder::VnetBgpGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp body object
    ///```ignore
    /// let response = client.vnet_bgp_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_put_by_id(&self) -> builder::VnetBgpPutById {
        builder::VnetBgpPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_delete_by_id(&self) -> builder::VnetBgpDeleteById {
        builder::VnetBgpDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_interface_commands`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_interface_commands_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interface_commands_get(
        &self,
    ) -> builder::VnetBgpInterfaceCommandsGet {
        builder::VnetBgpInterfaceCommandsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_interface_commands`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_interface_commands body object
    ///```ignore
    /// let response = client.vnet_bgp_interface_commands_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interface_commands_post(
        &self,
    ) -> builder::VnetBgpInterfaceCommandsPost {
        builder::VnetBgpInterfaceCommandsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_interface_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_interface_commands_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interface_commands_get_by_id(
        &self,
    ) -> builder::VnetBgpInterfaceCommandsGetById {
        builder::VnetBgpInterfaceCommandsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_interface_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_interface_commands body object
    ///```ignore
    /// let response = client.vnet_bgp_interface_commands_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interface_commands_put_by_id(
        &self,
    ) -> builder::VnetBgpInterfaceCommandsPutById {
        builder::VnetBgpInterfaceCommandsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_interface_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_interface_commands_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interface_commands_delete_by_id(
        &self,
    ) -> builder::VnetBgpInterfaceCommandsDeleteById {
        builder::VnetBgpInterfaceCommandsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_interfaces`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_interfaces_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interfaces_get(&self) -> builder::VnetBgpInterfacesGet {
        builder::VnetBgpInterfacesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_interfaces`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_interfaces body object
    ///```ignore
    /// let response = client.vnet_bgp_interfaces_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interfaces_post(&self) -> builder::VnetBgpInterfacesPost {
        builder::VnetBgpInterfacesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_interfaces/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_interfaces_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interfaces_get_by_id(&self) -> builder::VnetBgpInterfacesGetById {
        builder::VnetBgpInterfacesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_interfaces/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_interfaces body object
    ///```ignore
    /// let response = client.vnet_bgp_interfaces_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interfaces_put_by_id(&self) -> builder::VnetBgpInterfacesPutById {
        builder::VnetBgpInterfacesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_interfaces/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_interfaces_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_interfaces_delete_by_id(
        &self,
    ) -> builder::VnetBgpInterfacesDeleteById {
        builder::VnetBgpInterfacesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_ip`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_ip_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_ip_get(&self) -> builder::VnetBgpIpGet {
        builder::VnetBgpIpGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_ip`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_ip body object
    ///```ignore
    /// let response = client.vnet_bgp_ip_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_ip_post(&self) -> builder::VnetBgpIpPost {
        builder::VnetBgpIpPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_ip/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_ip_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_ip_get_by_id(&self) -> builder::VnetBgpIpGetById {
        builder::VnetBgpIpGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_ip/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_ip body object
    ///```ignore
    /// let response = client.vnet_bgp_ip_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_ip_put_by_id(&self) -> builder::VnetBgpIpPutById {
        builder::VnetBgpIpPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_ip/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_ip_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_ip_delete_by_id(&self) -> builder::VnetBgpIpDeleteById {
        builder::VnetBgpIpDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_routemap_commands`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_routemap_commands_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemap_commands_get(&self) -> builder::VnetBgpRoutemapCommandsGet {
        builder::VnetBgpRoutemapCommandsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_routemap_commands`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_routemap_commands body object
    ///```ignore
    /// let response = client.vnet_bgp_routemap_commands_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemap_commands_post(
        &self,
    ) -> builder::VnetBgpRoutemapCommandsPost {
        builder::VnetBgpRoutemapCommandsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_routemap_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_routemap_commands_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemap_commands_get_by_id(
        &self,
    ) -> builder::VnetBgpRoutemapCommandsGetById {
        builder::VnetBgpRoutemapCommandsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_routemap_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_routemap_commands body object
    ///```ignore
    /// let response = client.vnet_bgp_routemap_commands_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemap_commands_put_by_id(
        &self,
    ) -> builder::VnetBgpRoutemapCommandsPutById {
        builder::VnetBgpRoutemapCommandsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_routemap_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_routemap_commands_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemap_commands_delete_by_id(
        &self,
    ) -> builder::VnetBgpRoutemapCommandsDeleteById {
        builder::VnetBgpRoutemapCommandsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_routemaps`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_routemaps_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemaps_get(&self) -> builder::VnetBgpRoutemapsGet {
        builder::VnetBgpRoutemapsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_routemaps`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_routemaps body object
    ///```ignore
    /// let response = client.vnet_bgp_routemaps_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemaps_post(&self) -> builder::VnetBgpRoutemapsPost {
        builder::VnetBgpRoutemapsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_routemaps/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_routemaps_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemaps_get_by_id(&self) -> builder::VnetBgpRoutemapsGetById {
        builder::VnetBgpRoutemapsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_routemaps/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_routemaps body object
    ///```ignore
    /// let response = client.vnet_bgp_routemaps_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemaps_put_by_id(&self) -> builder::VnetBgpRoutemapsPutById {
        builder::VnetBgpRoutemapsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_routemaps/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_routemaps_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routemaps_delete_by_id(
        &self,
    ) -> builder::VnetBgpRoutemapsDeleteById {
        builder::VnetBgpRoutemapsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_router_commands`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_router_commands_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_router_commands_get(&self) -> builder::VnetBgpRouterCommandsGet {
        builder::VnetBgpRouterCommandsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_router_commands`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_router_commands body object
    ///```ignore
    /// let response = client.vnet_bgp_router_commands_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_router_commands_post(&self) -> builder::VnetBgpRouterCommandsPost {
        builder::VnetBgpRouterCommandsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_router_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_router_commands_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_router_commands_get_by_id(
        &self,
    ) -> builder::VnetBgpRouterCommandsGetById {
        builder::VnetBgpRouterCommandsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_router_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_router_commands body object
    ///```ignore
    /// let response = client.vnet_bgp_router_commands_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_router_commands_put_by_id(
        &self,
    ) -> builder::VnetBgpRouterCommandsPutById {
        builder::VnetBgpRouterCommandsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_router_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_router_commands_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_router_commands_delete_by_id(
        &self,
    ) -> builder::VnetBgpRouterCommandsDeleteById {
        builder::VnetBgpRouterCommandsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_routers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bgp_routers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routers_get(&self) -> builder::VnetBgpRoutersGet {
        builder::VnetBgpRoutersGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bgp_routers`
    ///
    ///Arguments:
    /// - `body`: vnet_bgp_routers body object
    ///```ignore
    /// let response = client.vnet_bgp_routers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routers_post(&self) -> builder::VnetBgpRoutersPost {
        builder::VnetBgpRoutersPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bgp_routers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bgp_routers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routers_get_by_id(&self) -> builder::VnetBgpRoutersGetById {
        builder::VnetBgpRoutersGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bgp_routers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bgp_routers body object
    ///```ignore
    /// let response = client.vnet_bgp_routers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routers_put_by_id(&self) -> builder::VnetBgpRoutersPutById {
        builder::VnetBgpRoutersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bgp_routers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bgp_routers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bgp_routers_delete_by_id(&self) -> builder::VnetBgpRoutersDeleteById {
        builder::VnetBgpRoutersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bond_interfaces`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bond_interfaces_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bond_interfaces_get(&self) -> builder::VnetBondInterfacesGet {
        builder::VnetBondInterfacesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bond_interfaces`
    ///
    ///Arguments:
    /// - `body`: vnet_bond_interfaces body object
    ///```ignore
    /// let response = client.vnet_bond_interfaces_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bond_interfaces_post(&self) -> builder::VnetBondInterfacesPost {
        builder::VnetBondInterfacesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bond_interfaces/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bond_interfaces_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bond_interfaces_get_by_id(&self) -> builder::VnetBondInterfacesGetById {
        builder::VnetBondInterfacesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bond_interfaces/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bond_interfaces_put_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bond_interfaces_put_by_id(&self) -> builder::VnetBondInterfacesPutById {
        builder::VnetBondInterfacesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bond_interfaces/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bond_interfaces_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bond_interfaces_delete_by_id(
        &self,
    ) -> builder::VnetBondInterfacesDeleteById {
        builder::VnetBondInterfacesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_bonds`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_bonds_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bonds_get(&self) -> builder::VnetBondsGet {
        builder::VnetBondsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_bonds`
    ///
    ///Arguments:
    /// - `body`: vnet_bonds body object
    ///```ignore
    /// let response = client.vnet_bonds_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bonds_post(&self) -> builder::VnetBondsPost {
        builder::VnetBondsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_bonds/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_bonds_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bonds_get_by_id(&self) -> builder::VnetBondsGetById {
        builder::VnetBondsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_bonds/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_bonds body object
    ///```ignore
    /// let response = client.vnet_bonds_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bonds_put_by_id(&self) -> builder::VnetBondsPutById {
        builder::VnetBondsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_bonds/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_bonds_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_bonds_delete_by_id(&self) -> builder::VnetBondsDeleteById {
        builder::VnetBondsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_cidrs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_cidrs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_cidrs_get(&self) -> builder::VnetCidrsGet {
        builder::VnetCidrsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_cidrs`
    ///
    ///Arguments:
    /// - `body`: vnet_cidrs body object
    ///```ignore
    /// let response = client.vnet_cidrs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_cidrs_post(&self) -> builder::VnetCidrsPost {
        builder::VnetCidrsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_cidrs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_cidrs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_cidrs_get_by_id(&self) -> builder::VnetCidrsGetById {
        builder::VnetCidrsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_cidrs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_cidrs body object
    ///```ignore
    /// let response = client.vnet_cidrs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_cidrs_put_by_id(&self) -> builder::VnetCidrsPutById {
        builder::VnetCidrsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_cidrs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_cidrs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_cidrs_delete_by_id(&self) -> builder::VnetCidrsDeleteById {
        builder::VnetCidrsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_dns_views`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_dns_views_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_views_get(&self) -> builder::VnetDnsViewsGet {
        builder::VnetDnsViewsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_dns_views`
    ///
    ///Arguments:
    /// - `body`: vnet_dns_views body object
    ///```ignore
    /// let response = client.vnet_dns_views_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_views_post(&self) -> builder::VnetDnsViewsPost {
        builder::VnetDnsViewsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_dns_views/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_dns_views_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_views_get_by_id(&self) -> builder::VnetDnsViewsGetById {
        builder::VnetDnsViewsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_dns_views/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_dns_views body object
    ///```ignore
    /// let response = client.vnet_dns_views_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_views_put_by_id(&self) -> builder::VnetDnsViewsPutById {
        builder::VnetDnsViewsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_dns_views/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_dns_views_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_views_delete_by_id(&self) -> builder::VnetDnsViewsDeleteById {
        builder::VnetDnsViewsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_dns_zone_records`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_dns_zone_records_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zone_records_get(&self) -> builder::VnetDnsZoneRecordsGet {
        builder::VnetDnsZoneRecordsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_dns_zone_records`
    ///
    ///Arguments:
    /// - `body`: vnet_dns_zone_records body object
    ///```ignore
    /// let response = client.vnet_dns_zone_records_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zone_records_post(&self) -> builder::VnetDnsZoneRecordsPost {
        builder::VnetDnsZoneRecordsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_dns_zone_records/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_dns_zone_records_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zone_records_get_by_id(&self) -> builder::VnetDnsZoneRecordsGetById {
        builder::VnetDnsZoneRecordsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_dns_zone_records/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_dns_zone_records body object
    ///```ignore
    /// let response = client.vnet_dns_zone_records_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zone_records_put_by_id(&self) -> builder::VnetDnsZoneRecordsPutById {
        builder::VnetDnsZoneRecordsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_dns_zone_records/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_dns_zone_records_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zone_records_delete_by_id(
        &self,
    ) -> builder::VnetDnsZoneRecordsDeleteById {
        builder::VnetDnsZoneRecordsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_dns_zones`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_dns_zones_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zones_get(&self) -> builder::VnetDnsZonesGet {
        builder::VnetDnsZonesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_dns_zones`
    ///
    ///Arguments:
    /// - `body`: vnet_dns_zones body object
    ///```ignore
    /// let response = client.vnet_dns_zones_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zones_post(&self) -> builder::VnetDnsZonesPost {
        builder::VnetDnsZonesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_dns_zones/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_dns_zones_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zones_get_by_id(&self) -> builder::VnetDnsZonesGetById {
        builder::VnetDnsZonesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_dns_zones/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_dns_zones body object
    ///```ignore
    /// let response = client.vnet_dns_zones_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zones_put_by_id(&self) -> builder::VnetDnsZonesPutById {
        builder::VnetDnsZonesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_dns_zones/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_dns_zones_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_dns_zones_delete_by_id(&self) -> builder::VnetDnsZonesDeleteById {
        builder::VnetDnsZonesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_eigrp_router_commands`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_eigrp_router_commands_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_router_commands_get(&self) -> builder::VnetEigrpRouterCommandsGet {
        builder::VnetEigrpRouterCommandsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_eigrp_router_commands`
    ///
    ///Arguments:
    /// - `body`: vnet_eigrp_router_commands body object
    ///```ignore
    /// let response = client.vnet_eigrp_router_commands_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_router_commands_post(
        &self,
    ) -> builder::VnetEigrpRouterCommandsPost {
        builder::VnetEigrpRouterCommandsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_eigrp_router_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_eigrp_router_commands_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_router_commands_get_by_id(
        &self,
    ) -> builder::VnetEigrpRouterCommandsGetById {
        builder::VnetEigrpRouterCommandsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_eigrp_router_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_eigrp_router_commands body object
    ///```ignore
    /// let response = client.vnet_eigrp_router_commands_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_router_commands_put_by_id(
        &self,
    ) -> builder::VnetEigrpRouterCommandsPutById {
        builder::VnetEigrpRouterCommandsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_eigrp_router_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_eigrp_router_commands_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_router_commands_delete_by_id(
        &self,
    ) -> builder::VnetEigrpRouterCommandsDeleteById {
        builder::VnetEigrpRouterCommandsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_eigrp_routers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_eigrp_routers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_routers_get(&self) -> builder::VnetEigrpRoutersGet {
        builder::VnetEigrpRoutersGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_eigrp_routers`
    ///
    ///Arguments:
    /// - `body`: vnet_eigrp_routers body object
    ///```ignore
    /// let response = client.vnet_eigrp_routers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_routers_post(&self) -> builder::VnetEigrpRoutersPost {
        builder::VnetEigrpRoutersPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_eigrp_routers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_eigrp_routers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_routers_get_by_id(&self) -> builder::VnetEigrpRoutersGetById {
        builder::VnetEigrpRoutersGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_eigrp_routers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_eigrp_routers body object
    ///```ignore
    /// let response = client.vnet_eigrp_routers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_routers_put_by_id(&self) -> builder::VnetEigrpRoutersPutById {
        builder::VnetEigrpRoutersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_eigrp_routers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_eigrp_routers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_eigrp_routers_delete_by_id(
        &self,
    ) -> builder::VnetEigrpRoutersDeleteById {
        builder::VnetEigrpRoutersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_hosts`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_hosts_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_hosts_get(&self) -> builder::VnetHostsGet {
        builder::VnetHostsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_hosts`
    ///
    ///Arguments:
    /// - `body`: vnet_hosts body object
    ///```ignore
    /// let response = client.vnet_hosts_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_hosts_post(&self) -> builder::VnetHostsPost {
        builder::VnetHostsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_hosts/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_hosts_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_hosts_get_by_id(&self) -> builder::VnetHostsGetById {
        builder::VnetHostsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_hosts/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_hosts body object
    ///```ignore
    /// let response = client.vnet_hosts_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_hosts_put_by_id(&self) -> builder::VnetHostsPutById {
        builder::VnetHostsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_hosts/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_hosts_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_hosts_delete_by_id(&self) -> builder::VnetHostsDeleteById {
        builder::VnetHostsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsec_connections`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_ipsec_connections_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_connections_get(&self) -> builder::VnetIpsecConnectionsGet {
        builder::VnetIpsecConnectionsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_ipsec_connections`
    ///
    ///Arguments:
    /// - `body`: vnet_ipsec_connections body object
    ///```ignore
    /// let response = client.vnet_ipsec_connections_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_connections_post(&self) -> builder::VnetIpsecConnectionsPost {
        builder::VnetIpsecConnectionsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsec_connections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_ipsec_connections_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_connections_get_by_id(
        &self,
    ) -> builder::VnetIpsecConnectionsGetById {
        builder::VnetIpsecConnectionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_ipsec_connections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_ipsec_connections body object
    ///```ignore
    /// let response = client.vnet_ipsec_connections_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_connections_put_by_id(
        &self,
    ) -> builder::VnetIpsecConnectionsPutById {
        builder::VnetIpsecConnectionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_ipsec_connections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_ipsec_connections_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_connections_delete_by_id(
        &self,
    ) -> builder::VnetIpsecConnectionsDeleteById {
        builder::VnetIpsecConnectionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsec_phase1s`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_ipsec_phase1s_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase1s_get(&self) -> builder::VnetIpsecPhase1sGet {
        builder::VnetIpsecPhase1sGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_ipsec_phase1s`
    ///
    ///Arguments:
    /// - `body`: vnet_ipsec_phase1s body object
    ///```ignore
    /// let response = client.vnet_ipsec_phase1s_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase1s_post(&self) -> builder::VnetIpsecPhase1sPost {
        builder::VnetIpsecPhase1sPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsec_phase1s/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_ipsec_phase1s_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase1s_get_by_id(&self) -> builder::VnetIpsecPhase1sGetById {
        builder::VnetIpsecPhase1sGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_ipsec_phase1s/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_ipsec_phase1s body object
    ///```ignore
    /// let response = client.vnet_ipsec_phase1s_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase1s_put_by_id(&self) -> builder::VnetIpsecPhase1sPutById {
        builder::VnetIpsecPhase1sPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_ipsec_phase1s/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_ipsec_phase1s_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase1s_delete_by_id(
        &self,
    ) -> builder::VnetIpsecPhase1sDeleteById {
        builder::VnetIpsecPhase1sDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsec_phase2s`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_ipsec_phase2s_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase2s_get(&self) -> builder::VnetIpsecPhase2sGet {
        builder::VnetIpsecPhase2sGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_ipsec_phase2s`
    ///
    ///Arguments:
    /// - `body`: vnet_ipsec_phase2s body object
    ///```ignore
    /// let response = client.vnet_ipsec_phase2s_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase2s_post(&self) -> builder::VnetIpsecPhase2sPost {
        builder::VnetIpsecPhase2sPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsec_phase2s/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_ipsec_phase2s_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase2s_get_by_id(&self) -> builder::VnetIpsecPhase2sGetById {
        builder::VnetIpsecPhase2sGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_ipsec_phase2s/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_ipsec_phase2s body object
    ///```ignore
    /// let response = client.vnet_ipsec_phase2s_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase2s_put_by_id(&self) -> builder::VnetIpsecPhase2sPutById {
        builder::VnetIpsecPhase2sPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_ipsec_phase2s/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_ipsec_phase2s_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsec_phase2s_delete_by_id(
        &self,
    ) -> builder::VnetIpsecPhase2sDeleteById {
        builder::VnetIpsecPhase2sDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsecs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_ipsecs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsecs_get(&self) -> builder::VnetIpsecsGet {
        builder::VnetIpsecsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_ipsecs`
    ///
    ///Arguments:
    /// - `body`: vnet_ipsecs body object
    ///```ignore
    /// let response = client.vnet_ipsecs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsecs_post(&self) -> builder::VnetIpsecsPost {
        builder::VnetIpsecsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_ipsecs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_ipsecs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsecs_get_by_id(&self) -> builder::VnetIpsecsGetById {
        builder::VnetIpsecsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_ipsecs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_ipsecs body object
    ///```ignore
    /// let response = client.vnet_ipsecs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsecs_put_by_id(&self) -> builder::VnetIpsecsPutById {
        builder::VnetIpsecsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_ipsecs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_ipsecs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ipsecs_delete_by_id(&self) -> builder::VnetIpsecsDeleteById {
        builder::VnetIpsecsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_monitor_stats_history_long`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_long_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_long_get(
        &self,
    ) -> builder::VnetMonitorStatsHistoryLongGet {
        builder::VnetMonitorStatsHistoryLongGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_monitor_stats_history_long`
    ///
    ///Arguments:
    /// - `body`: vnet_monitor_stats_history_long body object
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_long_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_long_post(
        &self,
    ) -> builder::VnetMonitorStatsHistoryLongPost {
        builder::VnetMonitorStatsHistoryLongPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_monitor_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_long_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_long_get_by_id(
        &self,
    ) -> builder::VnetMonitorStatsHistoryLongGetById {
        builder::VnetMonitorStatsHistoryLongGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_monitor_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_monitor_stats_history_long body object
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_long_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_long_put_by_id(
        &self,
    ) -> builder::VnetMonitorStatsHistoryLongPutById {
        builder::VnetMonitorStatsHistoryLongPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_monitor_stats_history_long/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_long_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_long_delete_by_id(
        &self,
    ) -> builder::VnetMonitorStatsHistoryLongDeleteById {
        builder::VnetMonitorStatsHistoryLongDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_monitor_stats_history_short`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_short_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_short_get(
        &self,
    ) -> builder::VnetMonitorStatsHistoryShortGet {
        builder::VnetMonitorStatsHistoryShortGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_monitor_stats_history_short`
    ///
    ///Arguments:
    /// - `body`: vnet_monitor_stats_history_short body object
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_short_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_short_post(
        &self,
    ) -> builder::VnetMonitorStatsHistoryShortPost {
        builder::VnetMonitorStatsHistoryShortPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_monitor_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_short_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_short_get_by_id(
        &self,
    ) -> builder::VnetMonitorStatsHistoryShortGetById {
        builder::VnetMonitorStatsHistoryShortGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_monitor_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_monitor_stats_history_short body object
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_short_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_short_put_by_id(
        &self,
    ) -> builder::VnetMonitorStatsHistoryShortPutById {
        builder::VnetMonitorStatsHistoryShortPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_monitor_stats_history_short/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_monitor_stats_history_short_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_monitor_stats_history_short_delete_by_id(
        &self,
    ) -> builder::VnetMonitorStatsHistoryShortDeleteById {
        builder::VnetMonitorStatsHistoryShortDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_ospf_commands`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_ospf_commands_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ospf_commands_get(&self) -> builder::VnetOspfCommandsGet {
        builder::VnetOspfCommandsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_ospf_commands`
    ///
    ///Arguments:
    /// - `body`: vnet_ospf_commands body object
    ///```ignore
    /// let response = client.vnet_ospf_commands_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ospf_commands_post(&self) -> builder::VnetOspfCommandsPost {
        builder::VnetOspfCommandsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_ospf_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_ospf_commands_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ospf_commands_get_by_id(&self) -> builder::VnetOspfCommandsGetById {
        builder::VnetOspfCommandsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_ospf_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_ospf_commands body object
    ///```ignore
    /// let response = client.vnet_ospf_commands_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ospf_commands_put_by_id(&self) -> builder::VnetOspfCommandsPutById {
        builder::VnetOspfCommandsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_ospf_commands/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_ospf_commands_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_ospf_commands_delete_by_id(
        &self,
    ) -> builder::VnetOspfCommandsDeleteById {
        builder::VnetOspfCommandsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_proxy`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_proxy_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_get(&self) -> builder::VnetProxyGet {
        builder::VnetProxyGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_proxy`
    ///
    ///Arguments:
    /// - `body`: vnet_proxy body object
    ///```ignore
    /// let response = client.vnet_proxy_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_post(&self) -> builder::VnetProxyPost {
        builder::VnetProxyPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_proxy/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_proxy_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_get_by_id(&self) -> builder::VnetProxyGetById {
        builder::VnetProxyGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_proxy/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_proxy body object
    ///```ignore
    /// let response = client.vnet_proxy_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_put_by_id(&self) -> builder::VnetProxyPutById {
        builder::VnetProxyPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_proxy/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_proxy_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_delete_by_id(&self) -> builder::VnetProxyDeleteById {
        builder::VnetProxyDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_proxy_tenants`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_proxy_tenants_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_tenants_get(&self) -> builder::VnetProxyTenantsGet {
        builder::VnetProxyTenantsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_proxy_tenants`
    ///
    ///Arguments:
    /// - `body`: vnet_proxy_tenants body object
    ///```ignore
    /// let response = client.vnet_proxy_tenants_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_tenants_post(&self) -> builder::VnetProxyTenantsPost {
        builder::VnetProxyTenantsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_proxy_tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_proxy_tenants_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_tenants_get_by_id(&self) -> builder::VnetProxyTenantsGetById {
        builder::VnetProxyTenantsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_proxy_tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_proxy_tenants body object
    ///```ignore
    /// let response = client.vnet_proxy_tenants_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_tenants_put_by_id(&self) -> builder::VnetProxyTenantsPutById {
        builder::VnetProxyTenantsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_proxy_tenants/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_proxy_tenants_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_proxy_tenants_delete_by_id(
        &self,
    ) -> builder::VnetProxyTenantsDeleteById {
        builder::VnetProxyTenantsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_queries`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_queries_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_queries_get(&self) -> builder::VnetQueriesGet {
        builder::VnetQueriesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_queries`
    ///
    ///Arguments:
    /// - `body`: vnet_queries body object
    ///```ignore
    /// let response = client.vnet_queries_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_queries_post(&self) -> builder::VnetQueriesPost {
        builder::VnetQueriesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_queries_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_queries_get_by_id(&self) -> builder::VnetQueriesGetById {
        builder::VnetQueriesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_queries body object
    ///```ignore
    /// let response = client.vnet_queries_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_queries_put_by_id(&self) -> builder::VnetQueriesPutById {
        builder::VnetQueriesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_queries/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_queries_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_queries_delete_by_id(&self) -> builder::VnetQueriesDeleteById {
        builder::VnetQueriesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_rule_aliases`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_rule_aliases_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_aliases_get(&self) -> builder::VnetRuleAliasesGet {
        builder::VnetRuleAliasesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_rule_aliases`
    ///
    ///Arguments:
    /// - `body`: vnet_rule_aliases body object
    ///```ignore
    /// let response = client.vnet_rule_aliases_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_aliases_post(&self) -> builder::VnetRuleAliasesPost {
        builder::VnetRuleAliasesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_rule_aliases/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_rule_aliases_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_aliases_get_by_id(&self) -> builder::VnetRuleAliasesGetById {
        builder::VnetRuleAliasesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_rule_aliases/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_rule_aliases body object
    ///```ignore
    /// let response = client.vnet_rule_aliases_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_aliases_put_by_id(&self) -> builder::VnetRuleAliasesPutById {
        builder::VnetRuleAliasesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_rule_aliases/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_rule_aliases_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_aliases_delete_by_id(&self) -> builder::VnetRuleAliasesDeleteById {
        builder::VnetRuleAliasesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_rule_references`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_rule_references_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_references_get(&self) -> builder::VnetRuleReferencesGet {
        builder::VnetRuleReferencesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_rule_references`
    ///
    ///Arguments:
    /// - `body`: vnet_rule_references body object
    ///```ignore
    /// let response = client.vnet_rule_references_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_references_post(&self) -> builder::VnetRuleReferencesPost {
        builder::VnetRuleReferencesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_rule_references/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_rule_references_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_references_get_by_id(&self) -> builder::VnetRuleReferencesGetById {
        builder::VnetRuleReferencesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_rule_references/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_rule_references_put_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_references_put_by_id(&self) -> builder::VnetRuleReferencesPutById {
        builder::VnetRuleReferencesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_rule_references/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_rule_references_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rule_references_delete_by_id(
        &self,
    ) -> builder::VnetRuleReferencesDeleteById {
        builder::VnetRuleReferencesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_rules`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_rules_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rules_get(&self) -> builder::VnetRulesGet {
        builder::VnetRulesGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_rules`
    ///
    ///Arguments:
    /// - `body`: vnet_rules body object
    ///```ignore
    /// let response = client.vnet_rules_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rules_post(&self) -> builder::VnetRulesPost {
        builder::VnetRulesPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_rules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_rules_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rules_get_by_id(&self) -> builder::VnetRulesGetById {
        builder::VnetRulesGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_rules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_rules body object
    ///```ignore
    /// let response = client.vnet_rules_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rules_put_by_id(&self) -> builder::VnetRulesPutById {
        builder::VnetRulesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_rules/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_rules_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_rules_delete_by_id(&self) -> builder::VnetRulesDeleteById {
        builder::VnetRulesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_wireguard_peer_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_wireguard_peer_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peer_status_get(&self) -> builder::VnetWireguardPeerStatusGet {
        builder::VnetWireguardPeerStatusGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_wireguard_peer_status`
    ///
    ///Arguments:
    /// - `body`: vnet_wireguard_peer_status body object
    ///```ignore
    /// let response = client.vnet_wireguard_peer_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peer_status_post(
        &self,
    ) -> builder::VnetWireguardPeerStatusPost {
        builder::VnetWireguardPeerStatusPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_wireguard_peer_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_wireguard_peer_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peer_status_get_by_id(
        &self,
    ) -> builder::VnetWireguardPeerStatusGetById {
        builder::VnetWireguardPeerStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_wireguard_peer_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_wireguard_peer_status body object
    ///```ignore
    /// let response = client.vnet_wireguard_peer_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peer_status_put_by_id(
        &self,
    ) -> builder::VnetWireguardPeerStatusPutById {
        builder::VnetWireguardPeerStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_wireguard_peer_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_wireguard_peer_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peer_status_delete_by_id(
        &self,
    ) -> builder::VnetWireguardPeerStatusDeleteById {
        builder::VnetWireguardPeerStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_wireguard_peers`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_wireguard_peers_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peers_get(&self) -> builder::VnetWireguardPeersGet {
        builder::VnetWireguardPeersGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_wireguard_peers`
    ///
    ///Arguments:
    /// - `body`: vnet_wireguard_peers body object
    ///```ignore
    /// let response = client.vnet_wireguard_peers_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peers_post(&self) -> builder::VnetWireguardPeersPost {
        builder::VnetWireguardPeersPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_wireguard_peers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_wireguard_peers_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peers_get_by_id(&self) -> builder::VnetWireguardPeersGetById {
        builder::VnetWireguardPeersGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_wireguard_peers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_wireguard_peers body object
    ///```ignore
    /// let response = client.vnet_wireguard_peers_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peers_put_by_id(&self) -> builder::VnetWireguardPeersPutById {
        builder::VnetWireguardPeersPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_wireguard_peers/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_wireguard_peers_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguard_peers_delete_by_id(
        &self,
    ) -> builder::VnetWireguardPeersDeleteById {
        builder::VnetWireguardPeersDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_wireguards`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_wireguards_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguards_get(&self) -> builder::VnetWireguardsGet {
        builder::VnetWireguardsGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_wireguards`
    ///
    ///Arguments:
    /// - `body`: vnet_wireguards body object
    ///```ignore
    /// let response = client.vnet_wireguards_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguards_post(&self) -> builder::VnetWireguardsPost {
        builder::VnetWireguardsPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_wireguards/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_wireguards_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguards_get_by_id(&self) -> builder::VnetWireguardsGetById {
        builder::VnetWireguardsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_wireguards/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_wireguards body object
    ///```ignore
    /// let response = client.vnet_wireguards_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguards_put_by_id(&self) -> builder::VnetWireguardsPutById {
        builder::VnetWireguardsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_wireguards/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_wireguards_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wireguards_delete_by_id(&self) -> builder::VnetWireguardsDeleteById {
        builder::VnetWireguardsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnet_wires`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnet_wires_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wires_get(&self) -> builder::VnetWiresGet {
        builder::VnetWiresGet::new(self)
    }
    ///Sends a `POST` request to `/vnet_wires`
    ///
    ///Arguments:
    /// - `body`: vnet_wires body object
    ///```ignore
    /// let response = client.vnet_wires_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wires_post(&self) -> builder::VnetWiresPost {
        builder::VnetWiresPost::new(self)
    }
    ///Sends a `GET` request to `/vnet_wires/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnet_wires_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wires_get_by_id(&self) -> builder::VnetWiresGetById {
        builder::VnetWiresGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnet_wires/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnet_wires body object
    ///```ignore
    /// let response = client.vnet_wires_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wires_put_by_id(&self) -> builder::VnetWiresPutById {
        builder::VnetWiresPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnet_wires/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnet_wires_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnet_wires_delete_by_id(&self) -> builder::VnetWiresDeleteById {
        builder::VnetWiresDeleteById::new(self)
    }
    ///Sends a `GET` request to `/vnets`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.vnets_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnets_get(&self) -> builder::VnetsGet {
        builder::VnetsGet::new(self)
    }
    ///Sends a `POST` request to `/vnets`
    ///
    ///Arguments:
    /// - `body`: vnets body object
    ///```ignore
    /// let response = client.vnets_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnets_post(&self) -> builder::VnetsPost {
        builder::VnetsPost::new(self)
    }
    ///Sends a `GET` request to `/vnets/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.vnets_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnets_get_by_id(&self) -> builder::VnetsGetById {
        builder::VnetsGetById::new(self)
    }
    ///Sends a `PUT` request to `/vnets/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: vnets body object
    ///```ignore
    /// let response = client.vnets_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnets_put_by_id(&self) -> builder::VnetsPutById {
        builder::VnetsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/vnets/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.vnets_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn vnets_delete_by_id(&self) -> builder::VnetsDeleteById {
        builder::VnetsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_actions_get(&self) -> builder::VolumeActionsGet {
        builder::VolumeActionsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_actions`
    ///
    ///Arguments:
    /// - `body`: volume_actions body object
    ///```ignore
    /// let response = client.volume_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_actions_post(&self) -> builder::VolumeActionsPost {
        builder::VolumeActionsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_actions_get_by_id(&self) -> builder::VolumeActionsGetById {
        builder::VolumeActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_actions body object
    ///```ignore
    /// let response = client.volume_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_actions_put_by_id(&self) -> builder::VolumeActionsPutById {
        builder::VolumeActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_actions_delete_by_id(&self) -> builder::VolumeActionsDeleteById {
        builder::VolumeActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_antivirus_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_get(&self) -> builder::VolumeAntivirusGet {
        builder::VolumeAntivirusGet::new(self)
    }
    ///Sends a `POST` request to `/volume_antivirus`
    ///
    ///Arguments:
    /// - `body`: volume_antivirus body object
    ///```ignore
    /// let response = client.volume_antivirus_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_post(&self) -> builder::VolumeAntivirusPost {
        builder::VolumeAntivirusPost::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_antivirus_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_get_by_id(&self) -> builder::VolumeAntivirusGetById {
        builder::VolumeAntivirusGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_antivirus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_antivirus body object
    ///```ignore
    /// let response = client.volume_antivirus_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_put_by_id(&self) -> builder::VolumeAntivirusPutById {
        builder::VolumeAntivirusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_antivirus/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_antivirus_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_delete_by_id(&self) -> builder::VolumeAntivirusDeleteById {
        builder::VolumeAntivirusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_antivirus_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_actions_get(&self) -> builder::VolumeAntivirusActionsGet {
        builder::VolumeAntivirusActionsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_antivirus_actions`
    ///
    ///Arguments:
    /// - `body`: volume_antivirus_actions body object
    ///```ignore
    /// let response = client.volume_antivirus_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_actions_post(&self) -> builder::VolumeAntivirusActionsPost {
        builder::VolumeAntivirusActionsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_antivirus_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_actions_get_by_id(
        &self,
    ) -> builder::VolumeAntivirusActionsGetById {
        builder::VolumeAntivirusActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_antivirus_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_antivirus_actions body object
    ///```ignore
    /// let response = client.volume_antivirus_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_actions_put_by_id(
        &self,
    ) -> builder::VolumeAntivirusActionsPutById {
        builder::VolumeAntivirusActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_antivirus_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_antivirus_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_actions_delete_by_id(
        &self,
    ) -> builder::VolumeAntivirusActionsDeleteById {
        builder::VolumeAntivirusActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_infections`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_antivirus_infections_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_infections_get(
        &self,
    ) -> builder::VolumeAntivirusInfectionsGet {
        builder::VolumeAntivirusInfectionsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_antivirus_infections`
    ///
    ///Arguments:
    /// - `body`: volume_antivirus_infections body object
    ///```ignore
    /// let response = client.volume_antivirus_infections_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_infections_post(
        &self,
    ) -> builder::VolumeAntivirusInfectionsPost {
        builder::VolumeAntivirusInfectionsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_infections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_antivirus_infections_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_infections_get_by_id(
        &self,
    ) -> builder::VolumeAntivirusInfectionsGetById {
        builder::VolumeAntivirusInfectionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_antivirus_infections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_antivirus_infections body object
    ///```ignore
    /// let response = client.volume_antivirus_infections_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_infections_put_by_id(
        &self,
    ) -> builder::VolumeAntivirusInfectionsPutById {
        builder::VolumeAntivirusInfectionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_antivirus_infections/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_antivirus_infections_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_infections_delete_by_id(
        &self,
    ) -> builder::VolumeAntivirusInfectionsDeleteById {
        builder::VolumeAntivirusInfectionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_antivirus_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_logs_get(&self) -> builder::VolumeAntivirusLogsGet {
        builder::VolumeAntivirusLogsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_antivirus_logs`
    ///
    ///Arguments:
    /// - `body`: volume_antivirus_logs body object
    ///```ignore
    /// let response = client.volume_antivirus_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_logs_post(&self) -> builder::VolumeAntivirusLogsPost {
        builder::VolumeAntivirusLogsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_antivirus_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_logs_get_by_id(
        &self,
    ) -> builder::VolumeAntivirusLogsGetById {
        builder::VolumeAntivirusLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_antivirus_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_antivirus_logs body object
    ///```ignore
    /// let response = client.volume_antivirus_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_logs_put_by_id(
        &self,
    ) -> builder::VolumeAntivirusLogsPutById {
        builder::VolumeAntivirusLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_antivirus_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_antivirus_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_logs_delete_by_id(
        &self,
    ) -> builder::VolumeAntivirusLogsDeleteById {
        builder::VolumeAntivirusLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_antivirus_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_stats_get(&self) -> builder::VolumeAntivirusStatsGet {
        builder::VolumeAntivirusStatsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_antivirus_stats`
    ///
    ///Arguments:
    /// - `body`: volume_antivirus_stats body object
    ///```ignore
    /// let response = client.volume_antivirus_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_stats_post(&self) -> builder::VolumeAntivirusStatsPost {
        builder::VolumeAntivirusStatsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_antivirus_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_stats_get_by_id(
        &self,
    ) -> builder::VolumeAntivirusStatsGetById {
        builder::VolumeAntivirusStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_antivirus_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_antivirus_stats body object
    ///```ignore
    /// let response = client.volume_antivirus_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_stats_put_by_id(
        &self,
    ) -> builder::VolumeAntivirusStatsPutById {
        builder::VolumeAntivirusStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_antivirus_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_antivirus_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_stats_delete_by_id(
        &self,
    ) -> builder::VolumeAntivirusStatsDeleteById {
        builder::VolumeAntivirusStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_antivirus_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_status_get(&self) -> builder::VolumeAntivirusStatusGet {
        builder::VolumeAntivirusStatusGet::new(self)
    }
    ///Sends a `POST` request to `/volume_antivirus_status`
    ///
    ///Arguments:
    /// - `body`: volume_antivirus_status body object
    ///```ignore
    /// let response = client.volume_antivirus_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_status_post(&self) -> builder::VolumeAntivirusStatusPost {
        builder::VolumeAntivirusStatusPost::new(self)
    }
    ///Sends a `GET` request to `/volume_antivirus_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_antivirus_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_status_get_by_id(
        &self,
    ) -> builder::VolumeAntivirusStatusGetById {
        builder::VolumeAntivirusStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_antivirus_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_antivirus_status body object
    ///```ignore
    /// let response = client.volume_antivirus_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_status_put_by_id(
        &self,
    ) -> builder::VolumeAntivirusStatusPutById {
        builder::VolumeAntivirusStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_antivirus_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_antivirus_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_antivirus_status_delete_by_id(
        &self,
    ) -> builder::VolumeAntivirusStatusDeleteById {
        builder::VolumeAntivirusStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_browser`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_browser_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_browser_get(&self) -> builder::VolumeBrowserGet {
        builder::VolumeBrowserGet::new(self)
    }
    ///Sends a `POST` request to `/volume_browser`
    ///
    ///Arguments:
    /// - `body`: volume_browser body object
    ///```ignore
    /// let response = client.volume_browser_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_browser_post(&self) -> builder::VolumeBrowserPost {
        builder::VolumeBrowserPost::new(self)
    }
    ///Sends a `GET` request to `/volume_browser/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_browser_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_browser_get_by_id(&self) -> builder::VolumeBrowserGetById {
        builder::VolumeBrowserGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_browser/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_browser body object
    ///```ignore
    /// let response = client.volume_browser_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_browser_put_by_id(&self) -> builder::VolumeBrowserPutById {
        builder::VolumeBrowserPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_browser/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_browser_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_browser_delete_by_id(&self) -> builder::VolumeBrowserDeleteById {
        builder::VolumeBrowserDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_cifs_shares`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_cifs_shares_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_cifs_shares_get(&self) -> builder::VolumeCifsSharesGet {
        builder::VolumeCifsSharesGet::new(self)
    }
    ///Sends a `POST` request to `/volume_cifs_shares`
    ///
    ///Arguments:
    /// - `body`: volume_cifs_shares body object
    ///```ignore
    /// let response = client.volume_cifs_shares_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_cifs_shares_post(&self) -> builder::VolumeCifsSharesPost {
        builder::VolumeCifsSharesPost::new(self)
    }
    ///Sends a `GET` request to `/volume_cifs_shares/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_cifs_shares_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_cifs_shares_get_by_id(&self) -> builder::VolumeCifsSharesGetById {
        builder::VolumeCifsSharesGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_cifs_shares/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_cifs_shares body object
    ///```ignore
    /// let response = client.volume_cifs_shares_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_cifs_shares_put_by_id(&self) -> builder::VolumeCifsSharesPutById {
        builder::VolumeCifsSharesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_cifs_shares/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_cifs_shares_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_cifs_shares_delete_by_id(
        &self,
    ) -> builder::VolumeCifsSharesDeleteById {
        builder::VolumeCifsSharesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_logs_get(&self) -> builder::VolumeLogsGet {
        builder::VolumeLogsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_logs`
    ///
    ///Arguments:
    /// - `body`: volume_logs body object
    ///```ignore
    /// let response = client.volume_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_logs_post(&self) -> builder::VolumeLogsPost {
        builder::VolumeLogsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_logs_get_by_id(&self) -> builder::VolumeLogsGetById {
        builder::VolumeLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_logs body object
    ///```ignore
    /// let response = client.volume_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_logs_put_by_id(&self) -> builder::VolumeLogsPutById {
        builder::VolumeLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_logs_delete_by_id(&self) -> builder::VolumeLogsDeleteById {
        builder::VolumeLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_nfs_shares`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_nfs_shares_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_nfs_shares_get(&self) -> builder::VolumeNfsSharesGet {
        builder::VolumeNfsSharesGet::new(self)
    }
    ///Sends a `POST` request to `/volume_nfs_shares`
    ///
    ///Arguments:
    /// - `body`: volume_nfs_shares body object
    ///```ignore
    /// let response = client.volume_nfs_shares_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_nfs_shares_post(&self) -> builder::VolumeNfsSharesPost {
        builder::VolumeNfsSharesPost::new(self)
    }
    ///Sends a `GET` request to `/volume_nfs_shares/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_nfs_shares_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_nfs_shares_get_by_id(&self) -> builder::VolumeNfsSharesGetById {
        builder::VolumeNfsSharesGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_nfs_shares/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_nfs_shares body object
    ///```ignore
    /// let response = client.volume_nfs_shares_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_nfs_shares_put_by_id(&self) -> builder::VolumeNfsSharesPutById {
        builder::VolumeNfsSharesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_nfs_shares/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_nfs_shares_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_nfs_shares_delete_by_id(&self) -> builder::VolumeNfsSharesDeleteById {
        builder::VolumeNfsSharesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_share_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_share_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_logs_get(&self) -> builder::VolumeShareLogsGet {
        builder::VolumeShareLogsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_share_logs`
    ///
    ///Arguments:
    /// - `body`: volume_share_logs body object
    ///```ignore
    /// let response = client.volume_share_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_logs_post(&self) -> builder::VolumeShareLogsPost {
        builder::VolumeShareLogsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_share_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_share_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_logs_get_by_id(&self) -> builder::VolumeShareLogsGetById {
        builder::VolumeShareLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_share_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_share_logs body object
    ///```ignore
    /// let response = client.volume_share_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_logs_put_by_id(&self) -> builder::VolumeShareLogsPutById {
        builder::VolumeShareLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_share_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_share_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_logs_delete_by_id(&self) -> builder::VolumeShareLogsDeleteById {
        builder::VolumeShareLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_share_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_share_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_status_get(&self) -> builder::VolumeShareStatusGet {
        builder::VolumeShareStatusGet::new(self)
    }
    ///Sends a `POST` request to `/volume_share_status`
    ///
    ///Arguments:
    /// - `body`: volume_share_status body object
    ///```ignore
    /// let response = client.volume_share_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_status_post(&self) -> builder::VolumeShareStatusPost {
        builder::VolumeShareStatusPost::new(self)
    }
    ///Sends a `GET` request to `/volume_share_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_share_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_status_get_by_id(&self) -> builder::VolumeShareStatusGetById {
        builder::VolumeShareStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_share_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_share_status body object
    ///```ignore
    /// let response = client.volume_share_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_status_put_by_id(&self) -> builder::VolumeShareStatusPutById {
        builder::VolumeShareStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_share_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_share_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_share_status_delete_by_id(
        &self,
    ) -> builder::VolumeShareStatusDeleteById {
        builder::VolumeShareStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_snapshots`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_snapshots_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_snapshots_get(&self) -> builder::VolumeSnapshotsGet {
        builder::VolumeSnapshotsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_snapshots`
    ///
    ///Arguments:
    /// - `body`: volume_snapshots body object
    ///```ignore
    /// let response = client.volume_snapshots_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_snapshots_post(&self) -> builder::VolumeSnapshotsPost {
        builder::VolumeSnapshotsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_snapshots_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_snapshots_get_by_id(&self) -> builder::VolumeSnapshotsGetById {
        builder::VolumeSnapshotsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_snapshots body object
    ///```ignore
    /// let response = client.volume_snapshots_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_snapshots_put_by_id(&self) -> builder::VolumeSnapshotsPutById {
        builder::VolumeSnapshotsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_snapshots/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_snapshots_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_snapshots_delete_by_id(&self) -> builder::VolumeSnapshotsDeleteById {
        builder::VolumeSnapshotsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_status`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_status_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_status_get(&self) -> builder::VolumeStatusGet {
        builder::VolumeStatusGet::new(self)
    }
    ///Sends a `POST` request to `/volume_status`
    ///
    ///Arguments:
    /// - `body`: volume_status body object
    ///```ignore
    /// let response = client.volume_status_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_status_post(&self) -> builder::VolumeStatusPost {
        builder::VolumeStatusPost::new(self)
    }
    ///Sends a `GET` request to `/volume_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_status_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_status_get_by_id(&self) -> builder::VolumeStatusGetById {
        builder::VolumeStatusGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_status body object
    ///```ignore
    /// let response = client.volume_status_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_status_put_by_id(&self) -> builder::VolumeStatusPutById {
        builder::VolumeStatusPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_status/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_status_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_status_delete_by_id(&self) -> builder::VolumeStatusDeleteById {
        builder::VolumeStatusDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_sync_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_sync_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_actions_get(&self) -> builder::VolumeSyncActionsGet {
        builder::VolumeSyncActionsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_sync_actions`
    ///
    ///Arguments:
    /// - `body`: volume_sync_actions body object
    ///```ignore
    /// let response = client.volume_sync_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_actions_post(&self) -> builder::VolumeSyncActionsPost {
        builder::VolumeSyncActionsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_sync_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_sync_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_actions_get_by_id(&self) -> builder::VolumeSyncActionsGetById {
        builder::VolumeSyncActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_sync_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_sync_actions body object
    ///```ignore
    /// let response = client.volume_sync_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_actions_put_by_id(&self) -> builder::VolumeSyncActionsPutById {
        builder::VolumeSyncActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_sync_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_sync_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_actions_delete_by_id(
        &self,
    ) -> builder::VolumeSyncActionsDeleteById {
        builder::VolumeSyncActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_sync_logs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_sync_logs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_logs_get(&self) -> builder::VolumeSyncLogsGet {
        builder::VolumeSyncLogsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_sync_logs`
    ///
    ///Arguments:
    /// - `body`: volume_sync_logs body object
    ///```ignore
    /// let response = client.volume_sync_logs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_logs_post(&self) -> builder::VolumeSyncLogsPost {
        builder::VolumeSyncLogsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_sync_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_sync_logs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_logs_get_by_id(&self) -> builder::VolumeSyncLogsGetById {
        builder::VolumeSyncLogsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_sync_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_sync_logs body object
    ///```ignore
    /// let response = client.volume_sync_logs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_logs_put_by_id(&self) -> builder::VolumeSyncLogsPutById {
        builder::VolumeSyncLogsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_sync_logs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_sync_logs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_logs_delete_by_id(&self) -> builder::VolumeSyncLogsDeleteById {
        builder::VolumeSyncLogsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_sync_progresses`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_sync_progresses_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_progresses_get(&self) -> builder::VolumeSyncProgressesGet {
        builder::VolumeSyncProgressesGet::new(self)
    }
    ///Sends a `POST` request to `/volume_sync_progresses`
    ///
    ///Arguments:
    /// - `body`: volume_sync_progresses body object
    ///```ignore
    /// let response = client.volume_sync_progresses_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_progresses_post(&self) -> builder::VolumeSyncProgressesPost {
        builder::VolumeSyncProgressesPost::new(self)
    }
    ///Sends a `GET` request to `/volume_sync_progresses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_sync_progresses_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_progresses_get_by_id(
        &self,
    ) -> builder::VolumeSyncProgressesGetById {
        builder::VolumeSyncProgressesGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_sync_progresses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_sync_progresses body object
    ///```ignore
    /// let response = client.volume_sync_progresses_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_progresses_put_by_id(
        &self,
    ) -> builder::VolumeSyncProgressesPutById {
        builder::VolumeSyncProgressesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_sync_progresses/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_sync_progresses_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_sync_progresses_delete_by_id(
        &self,
    ) -> builder::VolumeSyncProgressesDeleteById {
        builder::VolumeSyncProgressesDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_syncs`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_syncs_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_syncs_get(&self) -> builder::VolumeSyncsGet {
        builder::VolumeSyncsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_syncs`
    ///
    ///Arguments:
    /// - `body`: volume_syncs body object
    ///```ignore
    /// let response = client.volume_syncs_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_syncs_post(&self) -> builder::VolumeSyncsPost {
        builder::VolumeSyncsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_syncs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_syncs_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_syncs_get_by_id(&self) -> builder::VolumeSyncsGetById {
        builder::VolumeSyncsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_syncs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_syncs body object
    ///```ignore
    /// let response = client.volume_syncs_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_syncs_put_by_id(&self) -> builder::VolumeSyncsPutById {
        builder::VolumeSyncsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_syncs/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_syncs_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_syncs_delete_by_id(&self) -> builder::VolumeSyncsDeleteById {
        builder::VolumeSyncsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_vm_export_actions`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_vm_export_actions_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_actions_get(&self) -> builder::VolumeVmExportActionsGet {
        builder::VolumeVmExportActionsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_vm_export_actions`
    ///
    ///Arguments:
    /// - `body`: volume_vm_export_actions body object
    ///```ignore
    /// let response = client.volume_vm_export_actions_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_actions_post(&self) -> builder::VolumeVmExportActionsPost {
        builder::VolumeVmExportActionsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_vm_export_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_vm_export_actions_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_actions_get_by_id(
        &self,
    ) -> builder::VolumeVmExportActionsGetById {
        builder::VolumeVmExportActionsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_vm_export_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_vm_export_actions body object
    ///```ignore
    /// let response = client.volume_vm_export_actions_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_actions_put_by_id(
        &self,
    ) -> builder::VolumeVmExportActionsPutById {
        builder::VolumeVmExportActionsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_vm_export_actions/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_vm_export_actions_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_actions_delete_by_id(
        &self,
    ) -> builder::VolumeVmExportActionsDeleteById {
        builder::VolumeVmExportActionsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_vm_export_stats`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_vm_export_stats_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_stats_get(&self) -> builder::VolumeVmExportStatsGet {
        builder::VolumeVmExportStatsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_vm_export_stats`
    ///
    ///Arguments:
    /// - `body`: volume_vm_export_stats body object
    ///```ignore
    /// let response = client.volume_vm_export_stats_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_stats_post(&self) -> builder::VolumeVmExportStatsPost {
        builder::VolumeVmExportStatsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_vm_export_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_vm_export_stats_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_stats_get_by_id(
        &self,
    ) -> builder::VolumeVmExportStatsGetById {
        builder::VolumeVmExportStatsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_vm_export_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_vm_export_stats body object
    ///```ignore
    /// let response = client.volume_vm_export_stats_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_stats_put_by_id(
        &self,
    ) -> builder::VolumeVmExportStatsPutById {
        builder::VolumeVmExportStatsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_vm_export_stats/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_vm_export_stats_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_export_stats_delete_by_id(
        &self,
    ) -> builder::VolumeVmExportStatsDeleteById {
        builder::VolumeVmExportStatsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volume_vm_exports`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volume_vm_exports_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_exports_get(&self) -> builder::VolumeVmExportsGet {
        builder::VolumeVmExportsGet::new(self)
    }
    ///Sends a `POST` request to `/volume_vm_exports`
    ///
    ///Arguments:
    /// - `body`: volume_vm_exports body object
    ///```ignore
    /// let response = client.volume_vm_exports_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_exports_post(&self) -> builder::VolumeVmExportsPost {
        builder::VolumeVmExportsPost::new(self)
    }
    ///Sends a `GET` request to `/volume_vm_exports/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volume_vm_exports_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_exports_get_by_id(&self) -> builder::VolumeVmExportsGetById {
        builder::VolumeVmExportsGetById::new(self)
    }
    ///Sends a `PUT` request to `/volume_vm_exports/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volume_vm_exports body object
    ///```ignore
    /// let response = client.volume_vm_exports_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_exports_put_by_id(&self) -> builder::VolumeVmExportsPutById {
        builder::VolumeVmExportsPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volume_vm_exports/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volume_vm_exports_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volume_vm_exports_delete_by_id(&self) -> builder::VolumeVmExportsDeleteById {
        builder::VolumeVmExportsDeleteById::new(self)
    }
    ///Sends a `GET` request to `/volumes`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    /// - `filter`: filter list for rows
    /// - `limit`: limit result count
    /// - `offset`: offset results (used for paging)
    /// - `sort`: sort rows by field (example: +id, or -id)
    ///```ignore
    /// let response = client.volumes_get()
    ///    .fields(fields)
    ///    .filter(filter)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_get(&self) -> builder::VolumesGet {
        builder::VolumesGet::new(self)
    }
    ///Sends a `POST` request to `/volumes`
    ///
    ///Arguments:
    /// - `body`: volumes body object
    ///```ignore
    /// let response = client.volumes_post()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_post(&self) -> builder::VolumesPost {
        builder::VolumesPost::new(self)
    }
    ///Sends a `GET` request to `/volumes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.volumes_get_by_id()
    ///    .id(id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_get_by_id(&self) -> builder::VolumesGetById {
        builder::VolumesGetById::new(self)
    }
    ///Sends a `PUT` request to `/volumes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    /// - `body`: volumes body object
    ///```ignore
    /// let response = client.volumes_put_by_id()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_put_by_id(&self) -> builder::VolumesPutById {
        builder::VolumesPutById::new(self)
    }
    ///Sends a `DELETE` request to `/volumes/{id}`
    ///
    ///Arguments:
    /// - `id`: resource id
    ///```ignore
    /// let response = client.volumes_delete_by_id()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn volumes_delete_by_id(&self) -> builder::VolumesDeleteById {
        builder::VolumesDeleteById::new(self)
    }
    ///List tables
    ///
    ///Get a list of database tables
    ///
    ///Sends a `GET` request to `/`
    ///
    ///Arguments:
    /// - `fields`: comma delimited list of fields/view to return (most or all
    ///   may be provided as an alias)
    ///```ignore
    /// let response = client.get()
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get(&self) -> builder::Get {
        builder::Get::new(self)
    }
}
