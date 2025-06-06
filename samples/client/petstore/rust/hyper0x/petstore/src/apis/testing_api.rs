/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use futures::Future;

use crate::models;
use super::{Error, configuration};
use super::request as __internal_request;

pub struct TestingApiClient<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> TestingApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> TestingApiClient<C> {
        TestingApiClient {
            configuration,
        }
    }
}

pub trait TestingApi {
    fn tests_all_of_with_one_model_get(&self, person: models::Person) -> Pin<Box<dyn Future<Output = Result<String, Error>>>>;
    fn tests_file_response_get(&self, ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>>>>;
    fn tests_type_testing_get(&self, ) -> Pin<Box<dyn Future<Output = Result<models::TypeTesting, Error>>>>;
}

impl<C: hyper::client::connect::Connect>TestingApi for TestingApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn tests_all_of_with_one_model_get(&self, person: models::Person) -> Pin<Box<dyn Future<Output = Result<String, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tests/allOfWithOneModel".to_string())
        ;
        req = req.with_body_param(person);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn tests_file_response_get(&self, ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tests/fileResponse".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn tests_type_testing_get(&self, ) -> Pin<Box<dyn Future<Output = Result<models::TypeTesting, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/tests/typeTesting".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
