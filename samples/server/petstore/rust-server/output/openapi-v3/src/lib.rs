#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

// Crates with macros

#[macro_use]
extern crate serde_derive;
#[cfg(any(feature = "server"))]
#[macro_use]
extern crate lazy_static;
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate url;
#[macro_use]
extern crate log;

// Crates for conversion support
#[cfg(feature = "conversion")]
#[macro_use]
extern crate frunk_derives;
#[cfg(feature = "conversion")]
#[macro_use]
extern crate frunk_enum_derive;
#[cfg(feature = "conversion")]
extern crate frunk_core;

extern crate mime;
extern crate serde;
extern crate futures;
extern crate chrono;
extern crate swagger;

#[cfg(any(feature = "client", feature = "server"))]
extern crate hyper;
#[cfg(any(feature = "client"))]
extern crate hyper_tls;
#[cfg(any(feature = "client", feature = "server"))]
extern crate openssl;
#[cfg(any(feature = "client", feature = "server"))]
extern crate native_tls;
#[cfg(feature = "server")]
extern crate percent_encoding;
#[cfg(any(feature = "client", feature = "server"))]
extern crate serde_json;
#[cfg(any(feature = "client", feature = "server"))]
extern crate serde_ignored;
#[cfg(any(feature = "client", feature = "server"))]
extern crate tokio;

#[cfg(any(feature = "client", feature = "server"))]
extern crate uuid;

extern crate serde_xml_rs;


#[cfg(any(feature = "client", feature = "server"))]


use hyper::header::HeaderValue;
use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

#[deprecated(note = "Import swagger-rs directly")]
pub use swagger::{ApiError, ContextWrapper};
#[deprecated(note = "Import futures directly")]
pub use futures::Future;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "1.0.7";


#[derive(Debug, PartialEq)]
pub enum MandatoryRequestHeaderGetResponse {
    /// Success
    Success
}

#[derive(Debug, PartialEq)]
pub enum MultigetGetResponse {
    /// JSON rsp
    JSONRsp
    (models::AnotherXmlObject)
    ,
    /// XML rsp
    XMLRsp
    (models::InlineResponse201)
    ,
    /// octet rsp
    OctetRsp
    (swagger::ByteArray)
    ,
    /// string rsp
    StringRsp
    (String)
    ,
    /// Duplicate Response long text. One.
    DuplicateResponseLongText
    (models::AnotherXmlObject)
    ,
    /// Duplicate Response long text. Two.
    DuplicateResponseLongText_2
    (models::AnotherXmlObject)
    ,
    /// Duplicate Response long text. Three.
    DuplicateResponseLongText_3
    (models::AnotherXmlObject)
}

#[derive(Debug, PartialEq)]
pub enum MultipleAuthSchemeGetResponse {
    /// Check that limiting to multiple required auth schemes works
    CheckThatLimitingToMultipleRequiredAuthSchemesWorks
}

#[derive(Debug, PartialEq)]
pub enum ParamgetGetResponse {
    /// JSON rsp
    JSONRsp
    (models::AnotherXmlObject)
}

#[derive(Debug, PartialEq)]
pub enum ReadonlyAuthSchemeGetResponse {
    /// Check that limiting to a single required auth scheme works
    CheckThatLimitingToASingleRequiredAuthSchemeWorks
}

#[derive(Debug, PartialEq)]
pub enum RequiredOctetStreamPutResponse {
    /// OK
    OK
}

#[derive(Debug, PartialEq)]
pub enum ResponsesWithHeadersGetResponse {
    /// Success
    Success
    {
        body: String,
        success_info: String,
        object_header: models::ObjectHeader,
    }
    ,
    /// Precondition Failed
    PreconditionFailed
    {
        further_info: String,
        failure_info: String,
    }
}

#[derive(Debug, PartialEq)]
pub enum UuidGetResponse {
    /// Duplicate Response long text. One.
    DuplicateResponseLongText
    (uuid::Uuid)
}

#[derive(Debug, PartialEq)]
pub enum XmlExtraPostResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
}

#[derive(Debug, PartialEq)]
pub enum XmlOtherPostResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
}

#[derive(Debug, PartialEq)]
pub enum XmlOtherPutResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
}

#[derive(Debug, PartialEq)]
pub enum XmlPostResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
}

#[derive(Debug, PartialEq)]
pub enum XmlPutResponse {
    /// OK
    OK
    ,
    /// Bad Request
    BadRequest
}


/// API
pub trait Api<C> {


    fn mandatory_request_header_get(&self, x_header: String, context: &C) -> Box<dyn Future<Item=MandatoryRequestHeaderGetResponse, Error=ApiError> + Send>;

    /// Get some stuff.
    fn multiget_get(&self, context: &C) -> Box<dyn Future<Item=MultigetGetResponse, Error=ApiError> + Send>;


    fn multiple_auth_scheme_get(&self, context: &C) -> Box<dyn Future<Item=MultipleAuthSchemeGetResponse, Error=ApiError> + Send>;

    /// Get some stuff with parameters.
    fn paramget_get(&self, uuid: Option<uuid::Uuid>, some_object: Option<models::ObjectParam>, some_list: Option<models::MyIdList>, context: &C) -> Box<dyn Future<Item=ParamgetGetResponse, Error=ApiError> + Send>;


    fn readonly_auth_scheme_get(&self, context: &C) -> Box<dyn Future<Item=ReadonlyAuthSchemeGetResponse, Error=ApiError> + Send>;


    fn required_octet_stream_put(&self, body: swagger::ByteArray, context: &C) -> Box<dyn Future<Item=RequiredOctetStreamPutResponse, Error=ApiError> + Send>;


    fn responses_with_headers_get(&self, context: &C) -> Box<dyn Future<Item=ResponsesWithHeadersGetResponse, Error=ApiError> + Send>;


    fn uuid_get(&self, context: &C) -> Box<dyn Future<Item=UuidGetResponse, Error=ApiError> + Send>;


    fn xml_extra_post(&self, duplicate_xml_object: Option<models::DuplicateXmlObject>, context: &C) -> Box<dyn Future<Item=XmlExtraPostResponse, Error=ApiError> + Send>;


    fn xml_other_post(&self, another_xml_object: Option<models::AnotherXmlObject>, context: &C) -> Box<dyn Future<Item=XmlOtherPostResponse, Error=ApiError> + Send>;


    fn xml_other_put(&self, string: Option<models::AnotherXmlArray>, context: &C) -> Box<dyn Future<Item=XmlOtherPutResponse, Error=ApiError> + Send>;

    /// Post an array
    fn xml_post(&self, string: Option<models::XmlArray>, context: &C) -> Box<dyn Future<Item=XmlPostResponse, Error=ApiError> + Send>;


    fn xml_put(&self, xml_object: Option<models::XmlObject>, context: &C) -> Box<dyn Future<Item=XmlPutResponse, Error=ApiError> + Send>;

}

/// API without a `Context`
pub trait ApiNoContext {


    fn mandatory_request_header_get(&self, x_header: String) -> Box<dyn Future<Item=MandatoryRequestHeaderGetResponse, Error=ApiError> + Send>;

    /// Get some stuff.
    fn multiget_get(&self) -> Box<dyn Future<Item=MultigetGetResponse, Error=ApiError> + Send>;


    fn multiple_auth_scheme_get(&self) -> Box<dyn Future<Item=MultipleAuthSchemeGetResponse, Error=ApiError> + Send>;

    /// Get some stuff with parameters.
    fn paramget_get(&self, uuid: Option<uuid::Uuid>, some_object: Option<models::ObjectParam>, some_list: Option<models::MyIdList>) -> Box<dyn Future<Item=ParamgetGetResponse, Error=ApiError> + Send>;


    fn readonly_auth_scheme_get(&self) -> Box<dyn Future<Item=ReadonlyAuthSchemeGetResponse, Error=ApiError> + Send>;


    fn required_octet_stream_put(&self, body: swagger::ByteArray) -> Box<dyn Future<Item=RequiredOctetStreamPutResponse, Error=ApiError> + Send>;


    fn responses_with_headers_get(&self) -> Box<dyn Future<Item=ResponsesWithHeadersGetResponse, Error=ApiError> + Send>;


    fn uuid_get(&self) -> Box<dyn Future<Item=UuidGetResponse, Error=ApiError> + Send>;


    fn xml_extra_post(&self, duplicate_xml_object: Option<models::DuplicateXmlObject>) -> Box<dyn Future<Item=XmlExtraPostResponse, Error=ApiError> + Send>;


    fn xml_other_post(&self, another_xml_object: Option<models::AnotherXmlObject>) -> Box<dyn Future<Item=XmlOtherPostResponse, Error=ApiError> + Send>;


    fn xml_other_put(&self, string: Option<models::AnotherXmlArray>) -> Box<dyn Future<Item=XmlOtherPutResponse, Error=ApiError> + Send>;

    /// Post an array
    fn xml_post(&self, string: Option<models::XmlArray>) -> Box<dyn Future<Item=XmlPostResponse, Error=ApiError> + Send>;


    fn xml_put(&self, xml_object: Option<models::XmlObject>) -> Box<dyn Future<Item=XmlPutResponse, Error=ApiError> + Send>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {


    fn mandatory_request_header_get(&self, x_header: String) -> Box<dyn Future<Item=MandatoryRequestHeaderGetResponse, Error=ApiError> + Send> {
        self.api().mandatory_request_header_get(x_header, &self.context())
    }

    /// Get some stuff.
    fn multiget_get(&self) -> Box<dyn Future<Item=MultigetGetResponse, Error=ApiError> + Send> {
        self.api().multiget_get(&self.context())
    }


    fn multiple_auth_scheme_get(&self) -> Box<dyn Future<Item=MultipleAuthSchemeGetResponse, Error=ApiError> + Send> {
        self.api().multiple_auth_scheme_get(&self.context())
    }

    /// Get some stuff with parameters.
    fn paramget_get(&self, uuid: Option<uuid::Uuid>, some_object: Option<models::ObjectParam>, some_list: Option<models::MyIdList>) -> Box<dyn Future<Item=ParamgetGetResponse, Error=ApiError> + Send> {
        self.api().paramget_get(uuid, some_object, some_list, &self.context())
    }


    fn readonly_auth_scheme_get(&self) -> Box<dyn Future<Item=ReadonlyAuthSchemeGetResponse, Error=ApiError> + Send> {
        self.api().readonly_auth_scheme_get(&self.context())
    }


    fn required_octet_stream_put(&self, body: swagger::ByteArray) -> Box<dyn Future<Item=RequiredOctetStreamPutResponse, Error=ApiError> + Send> {
        self.api().required_octet_stream_put(body, &self.context())
    }


    fn responses_with_headers_get(&self) -> Box<dyn Future<Item=ResponsesWithHeadersGetResponse, Error=ApiError> + Send> {
        self.api().responses_with_headers_get(&self.context())
    }


    fn uuid_get(&self) -> Box<dyn Future<Item=UuidGetResponse, Error=ApiError> + Send> {
        self.api().uuid_get(&self.context())
    }


    fn xml_extra_post(&self, duplicate_xml_object: Option<models::DuplicateXmlObject>) -> Box<dyn Future<Item=XmlExtraPostResponse, Error=ApiError> + Send> {
        self.api().xml_extra_post(duplicate_xml_object, &self.context())
    }


    fn xml_other_post(&self, another_xml_object: Option<models::AnotherXmlObject>) -> Box<dyn Future<Item=XmlOtherPostResponse, Error=ApiError> + Send> {
        self.api().xml_other_post(another_xml_object, &self.context())
    }


    fn xml_other_put(&self, string: Option<models::AnotherXmlArray>) -> Box<dyn Future<Item=XmlOtherPutResponse, Error=ApiError> + Send> {
        self.api().xml_other_put(string, &self.context())
    }

    /// Post an array
    fn xml_post(&self, string: Option<models::XmlArray>) -> Box<dyn Future<Item=XmlPostResponse, Error=ApiError> + Send> {
        self.api().xml_post(string, &self.context())
    }


    fn xml_put(&self, xml_object: Option<models::XmlObject>) -> Box<dyn Future<Item=XmlPutResponse, Error=ApiError> + Send> {
        self.api().xml_put(xml_object, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
pub mod header;
