/// A builder providing access to all methods supported on *mediaItem* resources.
/// It is not used directly, but through the `PhotosLibrary` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_photoslibrary1 as photoslibrary1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use photoslibrary1::PhotosLibrary;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = PhotosLibrary::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `batch_create(...)`, `get(...)`, `list(...)` and `search(...)`
/// // to build up your call.
/// let rb = hub.media_items();
/// # }
/// ```
pub struct MediaItemMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a PhotosLibrary<C, A>,
}

impl<'a, C, A> MethodsBuilder for MediaItemMethods<'a, C, A> {}

impl<'a, C, A> MediaItemMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates one or more media items in a user's Google Photos library.
    /// 
    /// This is the second step for creating a media item. For details regarding
    /// Step 1, uploading the raw bytes to a Google Server, see
    /// <a href="/photos/library/guides/upload-media">Uploading media</a>.
    /// 
    /// This call adds the media item to the library. If an album `id` is
    /// specified, the call adds the media item to the album too. By default, the
    /// media item will be added to the end of the library or album.
    /// 
    /// If an album `id` and position are both defined, the media item is
    /// added to the album at the specified position.
    /// 
    /// If the call contains multiple media items, they're added at the specified
    /// position.
    /// If you are creating a media item in a shared album where you are not the
    /// owner, you are not allowed to position the media item. Doing so will result
    /// in a `BAD REQUEST` error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn batch_create(&self, request: BatchCreateMediaItemsRequest) -> MediaItemBatchCreateCall<'a, C, A> {
        MediaItemBatchCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the media item for the specified media item `id`.
    /// 
    /// # Arguments
    ///
    /// * `mediaItemId` - Identifier of media item to be requested.
    pub fn get(&self, media_item_id: &str) -> MediaItemGetCall<'a, C, A> {
        MediaItemGetCall {
            hub: self.hub,
            _media_item_id: media_item_id.to_string(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Searches for media items in a user's Google Photos library.
    /// If no filters are set, then all media items in the user's library are
    /// returned.
    /// If an album is set, all media items in the specified album are returned.
    /// If filters are specified, media items that match the filters from the user's
    /// library are listed.
    /// If you set both the album and the filters, the request results in an error.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn search(&self, request: SearchMediaItemsRequest) -> MediaItemSearchCall<'a, C, A> {
        MediaItemSearchCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// List all media items from a user's Google Photos library.
    pub fn list(&self) -> MediaItemListCall<'a, C, A> {
        MediaItemListCall {
            hub: self.hub,
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}