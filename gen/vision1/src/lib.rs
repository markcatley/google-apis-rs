// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Vision* crate version *1.0.7+20171107*, where *20171107* is the exact revision of the *vision:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.7*.
//! 
//! Everything else about the *Vision* *v1* API can be found at the
//! [official documentation site](https://cloud.google.com/vision/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/vision1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Vision.html) ... 
//! 
//! * [images](struct.Image.html)
//!  * [*annotate*](struct.ImageAnnotateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](http://byron.github.io/google-apis-rs).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.Vision.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.images().annotate(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-vision1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate hyper_rustls;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_vision1 as vision1;
//! use vision1::BatchAnnotateImagesRequest;
//! use vision1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use vision1::Vision;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = Vision::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
//! // As the method needs a request, you would usually fill it with the desired information
//! // into the respective structure. Some of the parts shown here might not be applicable !
//! // Values shown here are possibly random and not representative !
//! let mut req = BatchAnnotateImagesRequest::default();
//! 
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.images().annotate(req)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](https://docs.rs/yup-oauth2/*/yup_oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any
// unused imports in fully featured APIs. Same with unused_mut ... .
#![allow(unused_imports, unused_mut, dead_code)]

// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

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


// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// Apply machine learning models to understand and label images
    CloudVision,

    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudVision => "https://www.googleapis.com/auth/cloud-vision",
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudVision
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Vision related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_vision1 as vision1;
/// use vision1::BatchAnnotateImagesRequest;
/// use vision1::{Result, Error};
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use vision1::Vision;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Vision::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchAnnotateImagesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.images().annotate(req)
///              .doit();
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Vision<C, A> {
    client: RefCell<C>,
    auth: RefCell<A>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C, A> Hub for Vision<C, A> {}

impl<'a, C, A> Vision<C, A>
    where  C: BorrowMut<hyper::Client>, A: oauth2::GetToken {

    pub fn new(client: C, authenticator: A) -> Vision<C, A> {
        Vision {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/1.0.7".to_string(),
            _base_url: "https://vision.googleapis.com/".to_string(),
            _root_url: "https://vision.googleapis.com/".to_string(),
        }
    }

    pub fn images(&'a self) -> ImageMethods<'a, C, A> {
        ImageMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/1.0.7`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://vision.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://vision.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Request for performing Google Cloud Vision API tasks over a user-provided
/// image, with user-requested features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateImageRequest {
    /// Additional context that may accompany the image.
    #[serde(rename="imageContext")]
    pub image_context: Option<ImageContext>,
    /// The image to be processed.
    pub image: Option<Image>,
    /// Requested features.
    pub features: Option<Vec<Feature>>,
}

impl Part for AnnotateImageRequest {}


/// Users describe the type of Google Cloud Vision API tasks to perform over
/// images by using *Feature*s. Each Feature indicates a type of image
/// detection task to perform. Features encode the Cloud Vision API
/// vertical to operate on and the number of top-scoring results to return.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    /// The feature type.
    #[serde(rename="type")]
    pub type_: Option<String>,
    /// Maximum number of results of this type.
    #[serde(rename="maxResults")]
    pub max_results: Option<i32>,
}

impl Part for Feature {}


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


/// Single crop hint that is used to generate a new crop when serving an image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropHint {
    /// Confidence of this being a salient region.  Range [0, 1].
    pub confidence: Option<f32>,
    /// The bounding polygon for the crop region. The coordinates of the bounding
    /// box are in the original image's scale, as returned in `ImageParams`.
    #[serde(rename="boundingPoly")]
    pub bounding_poly: Option<BoundingPoly>,
    /// Fraction of importance of this salient region with respect to the original
    /// image.
    #[serde(rename="importanceFraction")]
    pub importance_fraction: Option<f32>,
}

impl Part for CropHint {}


/// Multiple image annotation requests are batched into a single service call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate images](struct.ImageAnnotateCall.html) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAnnotateImagesRequest {
    /// Individual image annotation requests for this batch.
    pub requests: Option<Vec<AnnotateImageRequest>>,
}

impl RequestValue for BatchAnnotateImagesRequest {}


/// A word representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Word {
    /// The bounding box for the word.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    pub bounding_box: Option<BoundingPoly>,
    /// List of symbols in the word.
    /// The order of the symbols follows the natural reading order.
    pub symbols: Option<Vec<Symbol>>,
    /// Additional information detected for the word.
    pub property: Option<TextProperty>,
}

impl Part for Word {}


/// Set of crop hints that are used to generate new crops when serving images.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropHintsAnnotation {
    /// Crop hint results.
    #[serde(rename="cropHints")]
    pub crop_hints: Option<Vec<CropHint>>,
}

impl Part for CropHintsAnnotation {}


/// Set of detected entity features.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EntityAnnotation {
    /// The accuracy of the entity detection in an image.
    /// For example, for an image in which the "Eiffel Tower" entity is detected,
    /// this field represents the confidence that there is a tower in the query
    /// image. Range [0, 1].
    pub confidence: Option<f32>,
    /// Entity textual description, expressed in its `locale` language.
    pub description: Option<String>,
    /// The language code for the locale in which the entity textual
    /// `description` is expressed.
    pub locale: Option<String>,
    /// The relevancy of the ICA (Image Content Annotation) label to the
    /// image. For example, the relevancy of "tower" is likely higher to an image
    /// containing the detected "Eiffel Tower" than to an image containing a
    /// detected distant towering building, even though the confidence that
    /// there is a tower in each image may be the same. Range [0, 1].
    pub topicality: Option<f32>,
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](https://developers.google.com/knowledge-graph/).
    pub mid: Option<String>,
    /// The location information for the detected entity. Multiple
    /// `LocationInfo` elements can be present because one location may
    /// indicate the location of the scene in the image, and another location
    /// may indicate the location of the place where the image was taken.
    /// Location information is usually present for landmarks.
    pub locations: Option<Vec<LocationInfo>>,
    /// Overall score of the result. Range [0, 1].
    pub score: Option<f32>,
    /// Image region to which this entity belongs. Not produced
    /// for `LABEL_DETECTION` features.
    #[serde(rename="boundingPoly")]
    pub bounding_poly: Option<BoundingPoly>,
    /// Some entities may have optional user-supplied `Property` (name/value)
    /// fields, such a score or string that qualifies the entity.
    pub properties: Option<Vec<Property>>,
}

impl Part for EntityAnnotation {}


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


/// Metadata for online images.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebImage {
    /// The result image URL.
    pub url: Option<String>,
    /// (Deprecated) Overall relevancy score for the image.
    pub score: Option<f32>,
}

impl Part for WebImage {}


/// Stores image properties, such as dominant colors.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageProperties {
    /// If present, dominant colors completed successfully.
    #[serde(rename="dominantColors")]
    pub dominant_colors: Option<DominantColorsAnnotation>,
}

impl Part for ImageProperties {}


/// Color information consists of RGB channels, score, and the fraction of
/// the image that the color occupies in the image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ColorInfo {
    /// RGB components of the color.
    pub color: Option<Color>,
    /// The fraction of pixels the color occupies in the image.
    /// Value in range [0, 1].
    #[serde(rename="pixelFraction")]
    pub pixel_fraction: Option<f32>,
    /// Image-specific score for this color. Value in range [0, 1].
    pub score: Option<f32>,
}

impl Part for ColorInfo {}


/// Relevant information for the image from the Internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebDetection {
    /// Deduced entities from similar images on the Internet.
    #[serde(rename="webEntities")]
    pub web_entities: Option<Vec<WebEntity>>,
    /// Web pages containing the matching images from the Internet.
    #[serde(rename="pagesWithMatchingImages")]
    pub pages_with_matching_images: Option<Vec<WebPage>>,
    /// The visually similar image results.
    #[serde(rename="visuallySimilarImages")]
    pub visually_similar_images: Option<Vec<WebImage>>,
    /// Partial matching images from the Internet.
    /// Those images are similar enough to share some key-point features. For
    /// example an original image will likely have partial matching for its crops.
    #[serde(rename="partialMatchingImages")]
    pub partial_matching_images: Option<Vec<WebImage>>,
    /// Fully matching images from the Internet.
    /// Can include resized copies of the query image.
    #[serde(rename="fullMatchingImages")]
    pub full_matching_images: Option<Vec<WebImage>>,
}

impl Part for WebDetection {}


/// A single symbol representation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    /// The bounding box for the symbol.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    pub bounding_box: Option<BoundingPoly>,
    /// The actual UTF-8 representation of the symbol.
    pub text: Option<String>,
    /// Additional information detected for the symbol.
    pub property: Option<TextProperty>,
}

impl Part for Symbol {}


/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// Y coordinate.
    pub y: Option<i32>,
    /// X coordinate.
    pub x: Option<i32>,
}

impl Part for Vertex {}


/// Additional information detected on the structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextProperty {
    /// Detected start or end of a text segment.
    #[serde(rename="detectedBreak")]
    pub detected_break: Option<DetectedBreak>,
    /// A list of detected languages together with confidence.
    #[serde(rename="detectedLanguages")]
    pub detected_languages: Option<Vec<DetectedLanguage>>,
}

impl Part for TextProperty {}


/// Rectangle determined by min and max `LatLng` pairs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLongRect {
    /// Min lat/long pair.
    #[serde(rename="minLatLng")]
    pub min_lat_lng: Option<LatLng>,
    /// Max lat/long pair.
    #[serde(rename="maxLatLng")]
    pub max_lat_lng: Option<LatLng>,
}

impl Part for LatLongRect {}


/// External image source (Google Cloud Storage image location).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageSource {
    /// NOTE: For new code `image_uri` below is preferred.
    /// Google Cloud Storage image URI, which must be in the following form:
    /// `gs://bucket_name/object_name` (for details, see
    /// [Google Cloud Storage Request
    /// URIs](https://cloud.google.com/storage/docs/reference-uris)).
    /// NOTE: Cloud Storage object versioning is not supported.
    #[serde(rename="gcsImageUri")]
    pub gcs_image_uri: Option<String>,
    /// Image URI which supports:
    /// 1) Google Cloud Storage image URI, which must be in the following form:
    /// `gs://bucket_name/object_name` (for details, see
    /// [Google Cloud Storage Request
    /// URIs](https://cloud.google.com/storage/docs/reference-uris)).
    /// NOTE: Cloud Storage object versioning is not supported.
    /// 2) Publicly accessible image HTTP/HTTPS URL.
    /// This is preferred over the legacy `gcs_image_uri` above. When both
    /// `gcs_image_uri` and `image_uri` are specified, `image_uri` takes
    /// precedence.
    #[serde(rename="imageUri")]
    pub image_uri: Option<String>,
}

impl Part for ImageSource {}


/// Set of features pertaining to the image, computed by computer vision
/// methods over safe-search verticals (for example, adult, spoof, medical,
/// violence).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SafeSearchAnnotation {
    /// Likelihood that this is a medical image.
    pub medical: Option<String>,
    /// Likelihood that this image contains violent content.
    pub violence: Option<String>,
    /// Spoof likelihood. The likelihood that an modification
    /// was made to the image's canonical version to make it appear
    /// funny or offensive.
    pub spoof: Option<String>,
    /// Represents the adult content likelihood for the image. Adult content may
    /// contain elements such as nudity, pornographic images or cartoons, or
    /// sexual activities.
    pub adult: Option<String>,
}

impl Part for SafeSearchAnnotation {}


/// TextAnnotation contains a structured representation of OCR extracted text.
/// The hierarchy of an OCR extracted text structure is like this:
///     TextAnnotation -> Page -> Block -> Paragraph -> Word -> Symbol
/// Each structural component, starting from Page, may further have their own
/// properties. Properties describe detected languages, breaks etc.. Please refer
/// to the TextAnnotation.TextProperty message definition below for more
/// detail.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TextAnnotation {
    /// UTF-8 text detected on the pages.
    pub text: Option<String>,
    /// List of pages detected by OCR.
    pub pages: Option<Vec<Page>>,
}

impl Part for TextAnnotation {}


/// Metadata for web pages.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebPage {
    /// The result web page URL.
    pub url: Option<String>,
    /// (Deprecated) Overall relevancy score for the web page.
    pub score: Option<f32>,
}

impl Part for WebPage {}


/// Detected entity location information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LocationInfo {
    /// lat/long location coordinates.
    #[serde(rename="latLng")]
    pub lat_lng: Option<LatLng>,
}

impl Part for LocationInfo {}


/// Structural unit of text representing a number of words in certain order.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Paragraph {
    /// The bounding box for the paragraph.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    pub bounding_box: Option<BoundingPoly>,
    /// Additional information detected for the paragraph.
    pub property: Option<TextProperty>,
    /// List of words in this paragraph.
    pub words: Option<Vec<Word>>,
}

impl Part for Paragraph {}


/// Detected language for a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectedLanguage {
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Confidence of detected language. Range [0, 1].
    pub confidence: Option<f32>,
}

impl Part for DetectedLanguage {}


/// Parameters for crop hints annotation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CropHintsParams {
    /// Aspect ratios in floats, representing the ratio of the width to the height
    /// of the image. For example, if the desired aspect ratio is 4/3, the
    /// corresponding float value should be 1.33333.  If not specified, the
    /// best possible crop is returned. The number of provided aspect ratios is
    /// limited to a maximum of 16; any aspect ratios provided after the 16th are
    /// ignored.
    #[serde(rename="aspectRatios")]
    pub aspect_ratios: Option<Vec<f32>>,
}

impl Part for CropHintsParams {}


/// A bounding polygon for the detected image annotation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    pub vertices: Option<Vec<Vertex>>,
}

impl Part for BoundingPoly {}


/// A face-specific landmark (for example, a face feature).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Landmark {
    /// Face landmark position.
    pub position: Option<Position>,
    /// Face landmark type.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for Landmark {}


/// A 3D position in the image, used primarily for Face detection landmarks.
/// A valid Position must have both x and y coordinates.
/// The position coordinates are in the same scale as the original image.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    /// Y coordinate.
    pub y: Option<f32>,
    /// X coordinate.
    pub x: Option<f32>,
    /// Z coordinate (or depth).
    pub z: Option<f32>,
}

impl Part for Position {}


/// Detected start or end of a structural component.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DetectedBreak {
    /// True if break prepends the element.
    #[serde(rename="isPrefix")]
    pub is_prefix: Option<bool>,
    /// Detected break type.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl Part for DetectedBreak {}


/// A `Property` consists of a user-supplied name/value pair.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    /// Value of numeric properties.
    #[serde(rename="uint64Value")]
    pub uint64_value: Option<String>,
    /// Name of the property.
    pub name: Option<String>,
    /// Value of the property.
    pub value: Option<String>,
}

impl Part for Property {}


/// Detected page from OCR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Page {
    /// Page width in pixels.
    pub width: Option<i32>,
    /// Additional information detected on the page.
    pub property: Option<TextProperty>,
    /// List of blocks of text, images etc on this page.
    pub blocks: Option<Vec<Block>>,
    /// Page height in pixels.
    pub height: Option<i32>,
}

impl Part for Page {}


/// Logical element on the page.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    /// The bounding box for the block.
    /// The vertices are in the order of top-left, top-right, bottom-right,
    /// bottom-left. When a rotation of the bounding box is detected the rotation
    /// is represented as around the top-left corner as defined when the text is
    /// read in the 'natural' orientation.
    /// For example:
    ///   * when the text is horizontal it might look like:
    ///      0----1
    ///      |    |
    ///      3----2
    ///   * when it's rotated 180 degrees around the top-left corner it becomes:
    ///      2----3
    ///      |    |
    ///      1----0
    ///   and the vertice order will still be (0, 1, 2, 3).
    #[serde(rename="boundingBox")]
    pub bounding_box: Option<BoundingPoly>,
    /// Detected block type (text, image etc) for this block.
    #[serde(rename="blockType")]
    pub block_type: Option<String>,
    /// Additional information detected for the block.
    pub property: Option<TextProperty>,
    /// List of paragraphs in this block (if this blocks is of type text).
    pub paragraphs: Option<Vec<Paragraph>>,
}

impl Part for Block {}


/// Image context and/or feature-specific parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ImageContext {
    /// lat/long rectangle that specifies the location of the image.
    #[serde(rename="latLongRect")]
    pub lat_long_rect: Option<LatLongRect>,
    /// List of languages to use for TEXT_DETECTION. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Text detection returns an
    /// error if one or more of the specified languages is not one of the
    /// [supported languages](/vision/docs/languages).
    #[serde(rename="languageHints")]
    pub language_hints: Option<Vec<String>>,
    /// Parameters for crop hints annotation request.
    #[serde(rename="cropHintsParams")]
    pub crop_hints_params: Option<CropHintsParams>,
}

impl Part for ImageContext {}


/// Set of dominant colors and their corresponding scores.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DominantColorsAnnotation {
    /// RGB color values with their score and pixel fraction.
    pub colors: Option<Vec<ColorInfo>>,
}

impl Part for DominantColorsAnnotation {}


/// Represents a color in the RGBA color space. This representation is designed
/// for simplicity of conversion to/from color representations in various
/// languages over compactness; for example, the fields of this representation
/// can be trivially provided to the constructor of "java.awt.Color" in Java; it
/// can also be trivially provided to UIColor's "+colorWithRed:green:blue:alpha"
/// method in iOS; and, with just a little work, it can be easily formatted into
/// a CSS "rgba()" string in JavaScript, as well. Here are some examples:
/// 
/// Example (Java):
/// 
///      import com.google.type.Color;
/// 
///      // ...
///      public static java.awt.Color fromProto(Color protocolor) {
///        float alpha = protocolor.hasAlpha()
///            ? protocolor.getAlpha().getValue()
///            : 1.0;
/// 
///        return new java.awt.Color(
///            protocolor.getRed(),
///            protocolor.getGreen(),
///            protocolor.getBlue(),
///            alpha);
///      }
/// 
///      public static Color toProto(java.awt.Color color) {
///        float red = (float) color.getRed();
///        float green = (float) color.getGreen();
///        float blue = (float) color.getBlue();
///        float denominator = 255.0;
///        Color.Builder resultBuilder =
///            Color
///                .newBuilder()
///                .setRed(red / denominator)
///                .setGreen(green / denominator)
///                .setBlue(blue / denominator);
///        int alpha = color.getAlpha();
///        if (alpha != 255) {
///          result.setAlpha(
///              FloatValue
///                  .newBuilder()
///                  .setValue(((float) alpha) / denominator)
///                  .build());
///        }
///        return resultBuilder.build();
///      }
///      // ...
/// 
/// Example (iOS / Obj-C):
/// 
///      // ...
///      static UIColor* fromProto(Color* protocolor) {
///         float red = [protocolor red];
///         float green = [protocolor green];
///         float blue = [protocolor blue];
///         FloatValue* alpha_wrapper = [protocolor alpha];
///         float alpha = 1.0;
///         if (alpha_wrapper != nil) {
///           alpha = [alpha_wrapper value];
///         }
///         return [UIColor colorWithRed:red green:green blue:blue alpha:alpha];
///      }
/// 
///      static Color* toProto(UIColor* color) {
///          CGFloat red, green, blue, alpha;
///          if (![color getRed:&red green:&green blue:&blue alpha:&alpha]) {
///            return nil;
///          }
///          Color* result = [Color alloc] init];
///          [result setRed:red];
///          [result setGreen:green];
///          [result setBlue:blue];
///          if (alpha <= 0.9999) {
///            [result setAlpha:floatWrapperWithValue(alpha)];
///          }
///          [result autorelease];
///          return result;
///     }
///     // ...
/// 
///  Example (JavaScript):
/// 
///     // ...
/// 
///     var protoToCssColor = function(rgb_color) {
///        var redFrac = rgb_color.red || 0.0;
///        var greenFrac = rgb_color.green || 0.0;
///        var blueFrac = rgb_color.blue || 0.0;
///        var red = Math.floor(redFrac * 255);
///        var green = Math.floor(greenFrac * 255);
///        var blue = Math.floor(blueFrac * 255);
/// 
///        if (!('alpha' in rgb_color)) {
///           return rgbToCssColor_(red, green, blue);
///        }
/// 
///        var alphaFrac = rgb_color.alpha.value || 0.0;
///        var rgbParams = [red, green, blue].join(',');
///        return ['rgba(', rgbParams, ',', alphaFrac, ')'].join('');
///     };
/// 
///     var rgbToCssColor_ = function(red, green, blue) {
///       var rgbNumber = new Number((red << 16) | (green << 8) | blue);
///       var hexString = rgbNumber.toString(16);
///       var missingZeros = 6 - hexString.length;
///       var resultBuilder = ['#'];
///       for (var i = 0; i < missingZeros; i++) {
///          resultBuilder.push('0');
///       }
///       resultBuilder.push(hexString);
///       return resultBuilder.join('');
///     };
/// 
///     // ...
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: Option<f32>,
    /// The fraction of this color that should be applied to the pixel. That is,
    /// the final pixel color is defined by the equation:
    /// 
    ///   pixel color = alpha * (this color) + (1.0 - alpha) * (background color)
    /// 
    /// This means that a value of 1.0 corresponds to a solid color, whereas
    /// a value of 0.0 corresponds to a completely transparent color. This
    /// uses a wrapper message rather than a simple float scalar so that it is
    /// possible to distinguish between a default value and the value being unset.
    /// If omitted, this color object is to be rendered as a solid color
    /// (as if the alpha value had been explicitly given with a value of 1.0).
    pub alpha: Option<f32>,
    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: Option<f32>,
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: Option<f32>,
}

impl Part for Color {}


/// Client image to perform Google Cloud Vision API tasks over.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate images](struct.ImageAnnotateCall.html) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    /// Image content, represented as a stream of bytes.
    /// Note: as with all `bytes` fields, protobuffers use a pure binary
    /// representation, whereas JSON representations use base64.
    pub content: Option<String>,
    /// Google Cloud Storage image location. If both `content` and `source`
    /// are provided for an image, `content` takes precedence and is
    /// used to perform the image annotation request.
    pub source: Option<ImageSource>,
}

impl Resource for Image {}


/// Entity deduced from similar images on the Internet.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebEntity {
    /// Opaque entity ID.
    #[serde(rename="entityId")]
    pub entity_id: Option<String>,
    /// Overall relevancy score for the entity.
    /// Not normalized and not comparable across different image queries.
    pub score: Option<f32>,
    /// Canonical description of the entity, in English.
    pub description: Option<String>,
}

impl Part for WebEntity {}


/// Response to a batch image annotation request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [annotate images](struct.ImageAnnotateCall.html) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchAnnotateImagesResponse {
    /// Individual responses to image annotation requests within the batch.
    pub responses: Option<Vec<AnnotateImageResponse>>,
}

impl ResponseResult for BatchAnnotateImagesResponse {}


/// A face annotation object contains the results of face detection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FaceAnnotation {
    /// Sorrow likelihood.
    #[serde(rename="sorrowLikelihood")]
    pub sorrow_likelihood: Option<String>,
    /// Under-exposed likelihood.
    #[serde(rename="underExposedLikelihood")]
    pub under_exposed_likelihood: Option<String>,
    /// Face landmarking confidence. Range [0, 1].
    #[serde(rename="landmarkingConfidence")]
    pub landmarking_confidence: Option<f32>,
    /// Detection confidence. Range [0, 1].
    #[serde(rename="detectionConfidence")]
    pub detection_confidence: Option<f32>,
    /// Joy likelihood.
    #[serde(rename="joyLikelihood")]
    pub joy_likelihood: Option<String>,
    /// Detected face landmarks.
    pub landmarks: Option<Vec<Landmark>>,
    /// Surprise likelihood.
    #[serde(rename="surpriseLikelihood")]
    pub surprise_likelihood: Option<String>,
    /// Blurred likelihood.
    #[serde(rename="blurredLikelihood")]
    pub blurred_likelihood: Option<String>,
    /// Pitch angle, which indicates the upwards/downwards angle that the face is
    /// pointing relative to the image's horizontal plane. Range [-180,180].
    #[serde(rename="tiltAngle")]
    pub tilt_angle: Option<f32>,
    /// Anger likelihood.
    #[serde(rename="angerLikelihood")]
    pub anger_likelihood: Option<String>,
    /// The bounding polygon around the face. The coordinates of the bounding box
    /// are in the original image's scale, as returned in `ImageParams`.
    /// The bounding box is computed to "frame" the face in accordance with human
    /// expectations. It is based on the landmarker results.
    /// Note that one or more x and/or y coordinates may not be generated in the
    /// `BoundingPoly` (the polygon will be unbounded) if only a partial face
    /// appears in the image to be annotated.
    #[serde(rename="boundingPoly")]
    pub bounding_poly: Option<BoundingPoly>,
    /// Roll angle, which indicates the amount of clockwise/anti-clockwise rotation
    /// of the face relative to the image vertical about the axis perpendicular to
    /// the face. Range [-180,180].
    #[serde(rename="rollAngle")]
    pub roll_angle: Option<f32>,
    /// Yaw angle, which indicates the leftward/rightward angle that the face is
    /// pointing relative to the vertical plane perpendicular to the image. Range
    /// [-180,180].
    #[serde(rename="panAngle")]
    pub pan_angle: Option<f32>,
    /// Headwear likelihood.
    #[serde(rename="headwearLikelihood")]
    pub headwear_likelihood: Option<String>,
    /// The `fd_bounding_poly` bounding polygon is tighter than the
    /// `boundingPoly`, and encloses only the skin part of the face. Typically, it
    /// is used to eliminate the face from any image analysis that detects the
    /// "amount of skin" visible in an image. It is not based on the
    /// landmarker results, only on the initial face detection, hence
    /// the <code>fd</code> (face detection) prefix.
    #[serde(rename="fdBoundingPoly")]
    pub fd_bounding_poly: Option<BoundingPoly>,
}

impl Part for FaceAnnotation {}


/// Response to an image annotation request.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnnotateImageResponse {
    /// If present, safe-search annotation has completed successfully.
    #[serde(rename="safeSearchAnnotation")]
    pub safe_search_annotation: Option<SafeSearchAnnotation>,
    /// If present, text (OCR) detection has completed successfully.
    #[serde(rename="textAnnotations")]
    pub text_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, web detection has completed successfully.
    #[serde(rename="webDetection")]
    pub web_detection: Option<WebDetection>,
    /// If present, text (OCR) detection or document (OCR) text detection has
    /// completed successfully.
    /// This annotation provides the structural hierarchy for the OCR detected
    /// text.
    #[serde(rename="fullTextAnnotation")]
    pub full_text_annotation: Option<TextAnnotation>,
    /// If present, label detection has completed successfully.
    #[serde(rename="labelAnnotations")]
    pub label_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, image properties were extracted successfully.
    #[serde(rename="imagePropertiesAnnotation")]
    pub image_properties_annotation: Option<ImageProperties>,
    /// If present, face detection has completed successfully.
    #[serde(rename="faceAnnotations")]
    pub face_annotations: Option<Vec<FaceAnnotation>>,
    /// If present, logo detection has completed successfully.
    #[serde(rename="logoAnnotations")]
    pub logo_annotations: Option<Vec<EntityAnnotation>>,
    /// If present, landmark detection has completed successfully.
    #[serde(rename="landmarkAnnotations")]
    pub landmark_annotations: Option<Vec<EntityAnnotation>>,
    /// If set, represents the error message for the operation.
    /// Note that filled-in image annotations are guaranteed to be
    /// correct, even when `error` is set.
    pub error: Option<Status>,
    /// If present, crop hints have completed successfully.
    #[serde(rename="cropHintsAnnotation")]
    pub crop_hints_annotation: Option<CropHintsAnnotation>,
}

impl Part for AnnotateImageResponse {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *image* resources.
/// It is not used directly, but through the `Vision` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_vision1 as vision1;
/// 
/// # #[test] fn egal() {
/// use std::default::Default;
/// use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// use vision1::Vision;
/// 
/// let secret: ApplicationSecret = Default::default();
/// let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
///                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
///                               <MemoryStorage as Default>::default(), None);
/// let mut hub = Vision::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `annotate(...)`
/// // to build up your call.
/// let rb = hub.images();
/// # }
/// ```
pub struct ImageMethods<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Vision<C, A>,
}

impl<'a, C, A> MethodsBuilder for ImageMethods<'a, C, A> {}

impl<'a, C, A> ImageMethods<'a, C, A> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Run image detection and annotation for a batch of images.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn annotate(&self, request: BatchAnnotateImagesRequest) -> ImageAnnotateCall<'a, C, A> {
        ImageAnnotateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _scopes: Default::default(),
            _additional_params: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Run image detection and annotation for a batch of images.
///
/// A builder for the *annotate* method supported by a *image* resource.
/// It is not used directly, but through a `ImageMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_vision1 as vision1;
/// use vision1::BatchAnnotateImagesRequest;
/// # #[test] fn egal() {
/// # use std::default::Default;
/// # use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
/// # use vision1::Vision;
/// 
/// # let secret: ApplicationSecret = Default::default();
/// # let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
/// #                               hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
/// #                               <MemoryStorage as Default>::default(), None);
/// # let mut hub = Vision::new(hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BatchAnnotateImagesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.images().annotate(req)
///              .doit();
/// # }
/// ```
pub struct ImageAnnotateCall<'a, C, A>
    where C: 'a, A: 'a {

    hub: &'a Vision<C, A>,
    _request: BatchAnnotateImagesRequest,
    _delegate: Option<&'a mut Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C, A> CallBuilder for ImageAnnotateCall<'a, C, A> {}

impl<'a, C, A> ImageAnnotateCall<'a, C, A> where C: BorrowMut<hyper::Client>, A: oauth2::GetToken {


    /// Perform the operation you have build so far.
    pub fn doit(mut self) -> Result<(hyper::client::Response, BatchAnnotateImagesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{ContentType, ContentLength, Authorization, Bearer, UserAgent, Location};
        let mut dd = DefaultDelegate;
        let mut dlg: &mut Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(MethodInfo { id: "vision.images.annotate",
                               http_method: hyper::method::Method::Post });
        let mut params: Vec<(&str, String)> = Vec::with_capacity((3 + self._additional_params.len()));
        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/images:annotate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }


        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params));
        }

        let mut json_mime_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default());
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.borrow_mut().token(self._scopes.keys()) {
                Ok(token) => token,
                Err(err) => {
                    match  dlg.token(&*err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(Error::MissingToken(err))
                        }
                    }
                }
            };
            let auth_header = Authorization(Bearer { token: token.access_token });
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let mut client = &mut *self.hub.client.borrow_mut();
                let mut req = client.borrow_mut().request(hyper::method::Method::Post, &url)
                    .header(UserAgent(self.hub._user_agent.clone()))
                    .header(auth_header.clone())
                    .header(ContentType(json_mime_type.clone()))
                    .header(ContentLength(request_size as u64))
                    .body(&mut request_value_reader);

                dlg.pre_request();
                req.send()
            };

            match req_result {
                Err(err) => {
                    if let oauth2::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        let mut json_err = String::new();
                        res.read_to_string(&mut json_err).unwrap();
                        if let oauth2::Retry::After(d) = dlg.http_failure(&res,
                                                              json::from_str(&json_err).ok(),
                                                              json::from_str(&json_err).ok()) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<ErrorResponse>(&json_err){
                            Err(_) => Err(Error::Failure(res)),
                            Ok(serr) => Err(Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let mut json_response = String::new();
                        res.read_to_string(&mut json_response).unwrap();
                        match json::from_str(&json_response) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&json_response, &err);
                                return Err(Error::JsonDecodeError(json_response, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BatchAnnotateImagesRequest) -> ImageAnnotateCall<'a, C, A> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut Delegate) -> ImageAnnotateCall<'a, C, A> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *bearer_token* (query-string) - OAuth bearer token.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *access_token* (query-string) - OAuth access token.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *callback* (query-string) - JSONP
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *alt* (query-string) - Data format for response.
    /// * *$.xgafv* (query-string) - V1 error format.
    pub fn param<T>(mut self, name: T, value: T) -> ImageAnnotateCall<'a, C, A>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ImageAnnotateCall<'a, C, A>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


