
#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate yup_oauth2 as oauth2;
extern crate mime;
extern crate url;

mod cmn;

use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;
use std::time::Duration;

pub use cmn::{MultiPartReader, ToParts, MethodInfo, Result, Error, CallBuilder, Hub, ReadSeek, Part,
              ResponseResult, RequestValue, NestedType, Delegate, DefaultDelegate, MethodsBuilder,
              Resource, ErrorResponse, remove_json_null_values};


// ############
// SCHEMAS ###
// ##########
/// Representation of an album in Google Photos.
/// Albums are containers for media items. If an album has been shared by the
/// application, it contains an extra `shareInfo` property.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list albums](struct.AlbumListCall.html) (none)
/// * [add enrichment albums](struct.AlbumAddEnrichmentCall.html) (none)
/// * [get shared albums](struct.SharedAlbumGetCall.html) (response)
/// * [share albums](struct.AlbumShareCall.html) (none)
/// * [create albums](struct.AlbumCreateCall.html) (response)
/// * [get albums](struct.AlbumGetCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    /// [Output only] The number of media items in the album.
    #[serde(rename="mediaItemsCount")]
    pub media_items_count: Option<i64>,
    /// Name of the album displayed to the user in their Google Photos account.
    /// This string shouldn't be more than 500 characters.
    pub title: Option<String>,
    /// [Output only] True if you can create media items in this album.
    /// This field is based on the scopes granted and permissions of the album. If
    /// the scopes are changed or permissions of the album are changed, this field
    /// is updated.
    #[serde(rename="isWriteable")]
    pub is_writeable: Option<bool>,
    /// [Output only] Information related to shared albums.
    /// This field is only populated if the album is a shared album, the
    /// developer created the album and the user has granted the
    /// `photoslibrary.sharing` scope.
    #[serde(rename="shareInfo")]
    pub share_info: Option<ShareInfo>,
    /// [Output only] A URL to the cover photo's bytes. This shouldn't be used as
    /// is. Parameters should be appended to this URL before use. For example,
    /// `'=w2048-h1024'` sets the dimensions of
    /// the cover photo to have a width of 2048 px and height of 1024 px.
    #[serde(rename="coverPhotoBaseUrl")]
    pub cover_photo_base_url: Option<String>,
    /// [Output only] Google Photos URL for the album. The user needs to be signed
    /// in to their Google Photos account to access this link.
    #[serde(rename="productUrl")]
    pub product_url: Option<String>,
    /// [Output only] Identifier for the media item associated with the cover
    /// photo.
    #[serde(rename="coverPhotoMediaItemId")]
    pub cover_photo_media_item_id: Option<String>,
    /// [Ouput only] Identifier for the album. This is a persistent identifier that
    /// can be used between sessions to identify this album.
    pub id: Option<String>,
}

impl Resource for Album {}
impl ResponseResult for Album {}


/// List of all media items from the user's Google Photos library.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list media items](struct.MediaItemListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMediaItemsResponse {
    /// [Output only] Token to use to get the next set of media items. Its presence
    /// is the only reliable indicator of more media items being available in the
    /// next request.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// [Output only] List of media items in the user's library.
    #[serde(rename="mediaItems")]
    pub media_items: Option<Vec<MediaItem>>,
}

impl ResponseResult for ListMediaItemsResponse {}


/// An enrichment containing text.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextEnrichment {
    /// Text for this enrichment item.
    pub text: Option<String>,
}

impl Part for TextEnrichment {}


/// Response to successfully sharing an album.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [share albums](struct.AlbumShareCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShareAlbumResponse {
    /// [Output only] Information about the shared album.
    #[serde(rename="shareInfo")]
    pub share_info: Option<ShareInfo>,
}

impl ResponseResult for ShareAlbumResponse {}


/// Defines a range of dates. Both dates must be of the same format. For more
/// information, see <a href="#Date">Date</a>
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateRange {
    /// The start date (included as part of the range) in one of the formats
    /// described.
    #[serde(rename="startDate")]
    pub start_date: Option<Date>,
    /// The end date (included as part of the range). It must be specified in the
    /// same format as the start date.
    #[serde(rename="endDate")]
    pub end_date: Option<Date>,
}

impl Part for DateRange {}


/// List of albums requested.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list albums](struct.AlbumListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAlbumsResponse {
    /// [Output only] Token to use to get the next set of albums. Populated if
    /// there are more albums to retrieve for this request.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// [Output only] List of albums shown in the Albums tab of the user's Google
    /// Photos app.
    pub albums: Option<Vec<Album>>,
}

impl ResponseResult for ListAlbumsResponse {}


/// Representation of a media item (such as a photo or video) in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create media items](struct.MediaItemBatchCreateCall.html) (none)
/// * [get media items](struct.MediaItemGetCall.html) (response)
/// * [search media items](struct.MediaItemSearchCall.html) (none)
/// * [list media items](struct.MediaItemListCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaItem {
    /// MIME type of the media item. For example, `image/jpeg`.
    #[serde(rename="mimeType")]
    pub mime_type: Option<String>,
    /// Description of the media item. This is shown to the user in the item's
    /// info section in the Google Photos app.
    pub description: Option<String>,
    /// A URL to the media item's bytes. This shouldn't be used directly to access
    /// the media item. For example, `'=w2048-h1024'` will set the dimensions of a
    /// media item of type photo to have a width of 2048 px and height of 1024 px.
    #[serde(rename="baseUrl")]
    pub base_url: Option<String>,
    /// Filename of the media item. This is shown to the user in the item's info
    /// section in the Google Photos app.
    pub filename: Option<String>,
    /// Google Photos URL for the media item. This link is available to
    /// the user only if they're signed in.
    #[serde(rename="productUrl")]
    pub product_url: Option<String>,
    /// Information about the user who created this media item.
    #[serde(rename="contributorInfo")]
    pub contributor_info: Option<ContributorInfo>,
    /// Metadata related to the media item, such as, height, width, or
    /// creation time.
    #[serde(rename="mediaMetadata")]
    pub media_metadata: Option<MediaMetadata>,
    /// Identifier for the media item. This is a persistent identifier that can be
    /// used between sessions to identify this media item.
    pub id: Option<String>,
}

impl Resource for MediaItem {}
impl ResponseResult for MediaItem {}


/// Request to add an enrichment to a specific album at a specific position.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add enrichment albums](struct.AlbumAddEnrichmentCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddEnrichmentToAlbumRequest {
    /// The position in the album where the enrichment is to be inserted.
    #[serde(rename="albumPosition")]
    pub album_position: Option<AlbumPosition>,
    /// The enrichment to be added.
    #[serde(rename="newEnrichmentItem")]
    pub new_enrichment_item: Option<NewEnrichmentItem>,
}

impl RequestValue for AddEnrichmentToAlbumRequest {}


/// Request to create one or more media items in a user's Google Photos library.
/// If an `albumid` is specified, the media items are also added to that album.
/// `albumPosition` is optional and can only be specified if an `albumId` is set.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create media items](struct.MediaItemBatchCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateMediaItemsRequest {
    /// Identifier of the album where the media items are added. The media items
    /// are also added to the user's library. This is an optional field.
    #[serde(rename="albumId")]
    pub album_id: Option<String>,
    /// List of media items to be created.
    #[serde(rename="newMediaItems")]
    pub new_media_items: Option<Vec<NewMediaItem>>,
    /// Position in the album where the media items are added. If not
    /// specified, the media items are added to the end of the album (as per
    /// the default value, that is, `LAST_IN_ALBUM`). The request fails if this
    /// field is set and the `albumId` is not specified. The request will also fail
    /// if you set the field and are not the owner of the shared album.
    #[serde(rename="albumPosition")]
    pub album_position: Option<AlbumPosition>,
}

impl RequestValue for BatchCreateMediaItemsRequest {}


/// Represents a physical location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Name of the location to be displayed.
    #[serde(rename="locationName")]
    pub location_name: Option<String>,
    /// Position of the location on the map.
    pub latlng: Option<LatLng>,
}

impl Part for Location {}


/// Specifies a position in an album.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AlbumPosition {
    /// The media item to which the position is relative to.
    /// Only used when position type is AFTER_MEDIA_ITEM.
    #[serde(rename="relativeMediaItemId")]
    pub relative_media_item_id: Option<String>,
    /// Type of position, for a media or enrichment item.
    pub position: Option<String>,
    /// The enrichment item to which the position is relative to.
    /// Only used when position type is AFTER_ENRICHMENT_ITEM.
    #[serde(rename="relativeEnrichmentItemId")]
    pub relative_enrichment_item_id: Option<String>,
}

impl Part for AlbumPosition {}


/// List of shared albums requested.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list shared albums](struct.SharedAlbumListCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListSharedAlbumsResponse {
    /// [Output only] Token to use to get the next set of shared albums. Populated
    /// if there are more shared albums to retrieve for this request.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// [Output only] List of shared albums.
    #[serde(rename="sharedAlbums")]
    pub shared_albums: Option<Vec<Album>>,
}

impl ResponseResult for ListSharedAlbumsResponse {}


/// An enrichment item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnrichmentItem {
    /// Identifier of the enrichment item.
    pub id: Option<String>,
}

impl Part for EnrichmentItem {}


/// The `Status` type defines a logical error model that is suitable for different
/// programming environments, including REST APIs and RPC APIs. It is used by
/// [gRPC](https://github.com/grpc). The error model is designed to be:
/// 
/// - Simple to use and understand for most users
/// - Flexible enough to meet unexpected needs
/// 
/// # Overview
/// 
/// The `Status` message contains three pieces of data: error code, error message,
/// and error details. The error code should be an enum value of
/// google.rpc.Code, but it may accept additional error codes if needed.  The
/// error message should be a developer-facing English message that helps
/// developers *understand* and *resolve* the error. If a localized user-facing
/// error message is needed, put the localized message in the error details or
/// localize it in the client. The optional error details may contain arbitrary
/// information about the error. There is a predefined set of error detail types
/// in the package `google.rpc` that can be used for common error conditions.
/// 
/// # Language mapping
/// 
/// The `Status` message is the logical representation of the error model, but it
/// is not necessarily the actual wire format. When the `Status` message is
/// exposed in different client libraries and different wire protocols, it can be
/// mapped differently. For example, it will likely be mapped to some exceptions
/// in Java, but more likely mapped to some error codes in C.
/// 
/// # Other uses
/// 
/// The error model and the `Status` message can be used in a variety of
/// environments, either with or without APIs, to provide a
/// consistent developer experience across different environments.
/// 
/// Example uses of this error model include:
/// 
/// - Partial errors. If a service needs to return partial errors to the client,
///     it may embed the `Status` in the normal response to indicate the partial
///     errors.
/// 
/// - Workflow errors. A typical workflow has multiple steps. Each step may
///     have a `Status` message for error reporting.
/// 
/// - Batch operations. If a client uses batch request and batch response, the
///     `Status` message should be used directly inside batch response, one for
///     each error sub-response.
/// 
/// - Asynchronous operations. If an API call embeds asynchronous operation
///     results in its response, the status of those operations should be
///     represented directly using the `Status` message.
/// 
/// - Logging. If some API errors are stored in logs, the message `Status` could
///     be used directly after any stripping needed for security/privacy reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// A developer-facing error message, which should be in English. Any
    /// user-facing error message should be localized and sent in the
    /// google.rpc.Status.details field, or localized by the client.
    pub message: Option<String>,
    /// The status code, which should be an enum value of google.rpc.Code.
    pub code: Option<i32>,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    pub details: Option<Vec<HashMap<String, String>>>,
}

impl Part for Status {}


/// An enrichment containing a map, showing origin and destination locations.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MapEnrichment {
    /// Origin location for this enrichment item.
    pub origin: Option<Location>,
    /// Destination location for this enrichemt item.
    pub destination: Option<Location>,
}

impl Part for MapEnrichment {}


/// An enrichment containing a single location.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationEnrichment {
    /// Location for this enrichment item.
    pub location: Option<Location>,
}

impl Part for LocationEnrichment {}


/// Options that control the sharing of an album.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SharedAlbumOptions {
    /// True if the shared album allows the owner and the collaborators (users
    /// who have joined the album) to add comments to the album. Defaults to false.
    #[serde(rename="isCommentable")]
    pub is_commentable: Option<bool>,
    /// True if the shared album allows collaborators (users who have joined
    /// the album) to add media items to it. Defaults to false.
    #[serde(rename="isCollaborative")]
    pub is_collaborative: Option<bool>,
}

impl Part for SharedAlbumOptions {}


/// Response to successfully joining the shared album on behalf of the user.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [join shared albums](struct.SharedAlbumJoinCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JoinSharedAlbumResponse {
    /// Shared album that the user has joined.
    pub album: Option<Album>,
}

impl ResponseResult for JoinSharedAlbumResponse {}


/// Represents a whole calendar date. The day may be 0 to represent a year and month where the day isn't significant, such as a whole calendar month. The month may be 0 to represent a a day and a year where the month isn't signficant, like when you want to specify the same day in every month of a year or a specific year. The year may be 0 to represent a month and day independent of year, like an anniversary date.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Date {
    /// Year of date. Must be from 1 to 9999, or 0 if specifying a date without
    /// a year.
    pub year: Option<i32>,
    /// Day of month. Must be from 1 to 31 and valid for the year and month, or 0 if specifying a year/month where the day isn't significant.
    pub day: Option<i32>,
    /// Month of year. Must be from 1 to 12, or 0 if specifying a year without a
    /// month and day.
    pub month: Option<i32>,
}

impl Part for Date {}


/// List of media items that match the search parameters.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search media items](struct.MediaItemSearchCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchMediaItemsResponse {
    /// [Output only] Use this token to get the next set of media items. Its
    /// presence is the only reliable indicator of more media items being available
    /// in the next request.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// [Output only] List of media items that match the search parameters.
    #[serde(rename="mediaItems")]
    pub media_items: Option<Vec<MediaItem>>,
}

impl ResponseResult for SearchMediaItemsResponse {}


/// Filters that can be applied to a media item search.
/// If multiple filter options are specified, they're treated as AND with each
/// other.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filters {
    /// Filters the media items based on their creation date.
    #[serde(rename="dateFilter")]
    pub date_filter: Option<DateFilter>,
    /// Filters the media items based on their content.
    #[serde(rename="contentFilter")]
    pub content_filter: Option<ContentFilter>,
    /// If set, the results include media items that the user has archived.
    /// Defaults to false (archived media items aren't included).
    #[serde(rename="includeArchivedMedia")]
    pub include_archived_media: Option<bool>,
    /// If set, the results exclude media items that were not created by this app.
    /// Defaults to false (all media items are returned). This field is ignored if
    /// the photoslibrary.readonly.appcreateddata scope is used.
    #[serde(rename="excludeNonAppCreatedData")]
    pub exclude_non_app_created_data: Option<bool>,
    /// Filters the media items based on the type of media.
    #[serde(rename="mediaTypeFilter")]
    pub media_type_filter: Option<MediaTypeFilter>,
}

impl Part for Filters {}


/// This filter allows you to return media items based on the content type.
/// 
/// It's possible to specify a list of categories to include, and/or a list of
/// categories to exclude. Within each list, the categories are combined with an
/// OR. <p>
/// The content filter `includedContentCategories`: [c1, c2, c3] would get media
/// items that contain (c1 OR c2 OR c3). <p>
/// The content filter `excludedContentCategories`: [c1, c2, c3] would NOT get
/// media items that contain (c1 OR c2 OR c3). <p>
/// You can also include some categories while excluding others, as in this
/// example: `includedContentCategories`: [c1, c2], `excludedContentCategories`:
/// [c3, c4] <p>
/// The previous example would get media items that contain (c1 OR c2) AND NOT
/// (c3 OR c4). A category that appears in `includedContentategories` must not
/// appear in `excludedContentCategories`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContentFilter {
    /// The set of categories which are not to be included in the media item search
    /// results. The items in the set are ORed. There's a maximum of 10
    /// `excludedContentCategories` per request.
    #[serde(rename="excludedContentCategories")]
    pub excluded_content_categories: Option<Vec<String>>,
    /// The set of categories to be included in the media item search results.
    /// The items in the set are ORed. There's a maximum of 10
    /// `includedContentCategories` per request.
    #[serde(rename="includedContentCategories")]
    pub included_content_categories: Option<Vec<String>>,
}

impl Part for ContentFilter {}


/// A simple media item to be created in Google Photos via an upload token.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SimpleMediaItem {
    /// Token identifying the media bytes that have been uploaded to Google.
    #[serde(rename="uploadToken")]
    pub upload_token: Option<String>,
}

impl Part for SimpleMediaItem {}


/// This filter defines the type of media items to be returned, for example,
/// videos or photos. All the specified media types are treated as an OR when
/// used together.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaTypeFilter {
    /// The types of media items to be included. This field should be populated
    /// with only one media type. If you specify multiple media types, it results
    /// in an error.
    #[serde(rename="mediaTypes")]
    pub media_types: Option<Vec<String>>,
}

impl Part for MediaTypeFilter {}


/// Information about albums that are shared. This information is only included
/// if you created the album, it is shared and you have the sharing scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShareInfo {
    /// Options that control the sharing of an album.
    #[serde(rename="sharedAlbumOptions")]
    pub shared_album_options: Option<SharedAlbumOptions>,
    /// A token that can be used by other users to join this shared album via the
    /// API.
    #[serde(rename="shareToken")]
    pub share_token: Option<String>,
    /// True if the user has joined the album. This is always true for the owner
    /// of the shared album.
    #[serde(rename="isJoined")]
    pub is_joined: Option<bool>,
    /// A link to the album that's now shared on the Google Photos website and app.
    /// Anyone with the link can access this shared album and see all of the items
    /// present in the album.
    #[serde(rename="shareableUrl")]
    pub shareable_url: Option<String>,
}

impl Part for ShareInfo {}


/// The enrichment item that's created.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add enrichment albums](struct.AlbumAddEnrichmentCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddEnrichmentToAlbumResponse {
    /// [Output only] Enrichment which was added.
    #[serde(rename="enrichmentItem")]
    pub enrichment_item: Option<EnrichmentItem>,
}

impl ResponseResult for AddEnrichmentToAlbumResponse {}


/// New media item that's created in a user's Google Photos account.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewMediaItem {
    /// A new media item that has been uploaded via the included `uploadToken`.
    #[serde(rename="simpleMediaItem")]
    pub simple_media_item: Option<SimpleMediaItem>,
    /// Description of the media item. This will be shown to the user in the item's
    /// info section in the Google Photos app.
    /// This string shouldn't be more than 1000 characters.
    pub description: Option<String>,
}

impl Part for NewMediaItem {}


/// Information about the user who added the media item. Note that this
/// information is included only if the media item is within a shared album
/// created by your app and you have the sharing scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContributorInfo {
    /// URL to the profile picture of the contributor.
    #[serde(rename="profilePictureBaseUrl")]
    pub profile_picture_base_url: Option<String>,
    /// Display name of the contributor.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
}

impl Part for ContributorInfo {}


/// Metadata for a media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// Original width (in pixels) of the media item.
    pub width: Option<String>,
    /// Time when the media item was first created (not when it was uploaded to
    /// Google Photos).
    #[serde(rename="creationTime")]
    pub creation_time: Option<String>,
    /// Metadata for a video media type.
    pub video: Option<Video>,
    /// Metadata for a photo media type.
    pub photo: Option<Photo>,
    /// Original height (in pixels) of the media item.
    pub height: Option<String>,
}

impl Part for MediaMetadata {}


/// A new enrichment item to be added to an album, used by the
/// `albums.addEnrichment` call.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewEnrichmentItem {
    /// Location to be added to the album.
    #[serde(rename="locationEnrichment")]
    pub location_enrichment: Option<LocationEnrichment>,
    /// Map to be added to the album.
    #[serde(rename="mapEnrichment")]
    pub map_enrichment: Option<MapEnrichment>,
    /// Text to be added to the album.
    #[serde(rename="textEnrichment")]
    pub text_enrichment: Option<TextEnrichment>,
}

impl Part for NewEnrichmentItem {}


/// Request to make an album shared in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [share albums](struct.AlbumShareCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShareAlbumRequest {
    /// Options to be set when converting the album to a shared album.
    #[serde(rename="sharedAlbumOptions")]
    pub shared_album_options: Option<SharedAlbumOptions>,
}

impl RequestValue for ShareAlbumRequest {}


/// Request to create an album in Google Photos.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [create albums](struct.AlbumCreateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateAlbumRequest {
    /// The album to be created.
    pub album: Option<Album>,
}

impl RequestValue for CreateAlbumRequest {}


/// Request to search for media items in a user's library.
/// 
/// If the album id is specified, this call will return the list of media items
/// in the album. If neither filters nor album id are
/// specified, this call will return all media items in a user's Google Photos
/// library.
/// 
/// If filters are specified, this call will return all media items in
/// the user's library that fulfill the filter criteria.
/// 
/// Filters and album id must not both be set, as this will result in an
/// invalid request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search media items](struct.MediaItemSearchCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchMediaItemsRequest {
    /// Identifier of an album. If populated, lists all media items in
    /// specified album. Can't set in conjunction with any filters.
    #[serde(rename="albumId")]
    pub album_id: Option<String>,
    /// Maximum number of media items to return in the response. The default number
    /// of media items to return at a time is 25. The maximum
    /// `pageSize` is 100.
    #[serde(rename="pageSize")]
    pub page_size: Option<i32>,
    /// Filters to apply to the request. Can't be set in conjunction with an
    /// `albumId`.
    pub filters: Option<Filters>,
    /// A continuation token to get the next page of the results. Adding this to
    /// the request returns the rows after the `pageToken`. The `pageToken` should
    /// be the value returned in the `nextPageToken` parameter in the response to
    /// the `searchMediaItems` request.
    #[serde(rename="pageToken")]
    pub page_token: Option<String>,
}

impl RequestValue for SearchMediaItemsRequest {}


/// Request to join a shared album on behalf of the user. This uses a shareToken
/// which can be acquired via the shareAlbum or listSharedAlbums calls.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [join shared albums](struct.SharedAlbumJoinCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct JoinSharedAlbumRequest {
    /// Token to join the shared album on behalf of the user.
    #[serde(rename="shareToken")]
    pub share_token: Option<String>,
}

impl RequestValue for JoinSharedAlbumRequest {}


/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl Part for LatLng {}


/// List of media items created.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [batch create media items](struct.MediaItemBatchCreateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateMediaItemsResponse {
    /// [Output only] List of media items created.
    #[serde(rename="newMediaItemResults")]
    pub new_media_item_results: Option<Vec<NewMediaItemResult>>,
}

impl ResponseResult for BatchCreateMediaItemsResponse {}


/// This filter defines the allowed dates or date ranges for the media returned.
/// It's possible to pick a set of specific dates and a set of date ranges.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DateFilter {
    /// List of dates ranges that match the media items' creation date. A
    /// maximum of 5 dates ranges can be included per request.
    pub ranges: Option<Vec<DateRange>>,
    /// List of dates that match the media items' creation date. A maximum of
    /// 5 dates can be included per request.
    pub dates: Option<Vec<Date>>,
}

impl Part for DateFilter {}


/// Metadata that is specific to a video, for example, fps and processing status.
/// Some of these fields may be null or not included.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Video {
    /// Processing status of the video.
    pub status: Option<String>,
    /// Brand of the camera with which the video was taken.
    #[serde(rename="cameraMake")]
    pub camera_make: Option<String>,
    /// Frame rate of the video.
    pub fps: Option<f64>,
    /// Model of the camera with which the video was taken.
    #[serde(rename="cameraModel")]
    pub camera_model: Option<String>,
}

impl Part for Video {}


/// Result of creating a new media item.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NewMediaItemResult {
    /// If an error occurred during the creation of this media item, this field
    /// is  populated with information related to the error. For details regarding
    /// this field, see <a href="#Status">Status</a>.
    pub status: Option<Status>,
    /// The upload token used to create this new media item.
    #[serde(rename="uploadToken")]
    pub upload_token: Option<String>,
    /// Media item created with the upload token. It's populated if no errors
    /// occurred and the media item was created successfully.
    #[serde(rename="mediaItem")]
    pub media_item: Option<MediaItem>,
}

impl Part for NewMediaItemResult {}


/// Metadata that is specific to a photo, such as, ISO, focal length and
/// exposure time. Some of these fields may be null or not included.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    /// Exposure time of the camera aperture when the photo was taken.
    #[serde(rename="exposureTime")]
    pub exposure_time: Option<String>,
    /// Brand of the camera with which the photo was taken.
    #[serde(rename="cameraMake")]
    pub camera_make: Option<String>,
    /// Focal length of the camera lens with which the photo was taken.
    #[serde(rename="focalLength")]
    pub focal_length: Option<f32>,
    /// ISO of the camera with which the photo was taken.
    #[serde(rename="isoEquivalent")]
    pub iso_equivalent: Option<i32>,
    /// Aperture f number of the camera lens with which the photo was taken.
    #[serde(rename="apertureFNumber")]
    pub aperture_f_number: Option<f32>,
    /// Model of the camera with which the photo was taken.
    #[serde(rename="cameraModel")]
    pub camera_model: Option<String>,
}

impl Part for Photo {}