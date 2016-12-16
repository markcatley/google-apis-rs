// DO NOT EDIT !
// This file was generated automatically from 'src/mako/cli/main.rs.mako'
// DO NOT EDIT !
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

#[macro_use]
extern crate clap;
extern crate yup_oauth2 as oauth2;
extern crate yup_hyper_mock as mock;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate mime;
extern crate strsim;
extern crate google_appengine1 as api;

use std::env;
use std::io::{self, Write};
use clap::{App, SubCommand, Arg};

mod cmn;

use cmn::{InvalidOptionsError, CLIError, JsonTokenStorage, arg_from_str, writer_from_opts, parse_kv_arg,
          input_file_from_opts, input_mime_from_opts, FieldCursor, FieldError, CallType, UploadProtocol,
          calltype_from_str, remove_json_null_values, ComplexType, JsonType, JsonTypeInfo};

use std::default::Default;
use std::str::FromStr;

use oauth2::{Authenticator, DefaultAuthenticatorDelegate, FlowType};
use serde_json as json;
use clap::ArgMatches;

enum DoitError {
    IoError(String, io::Error),
    ApiError(api::Error),
}

struct Engine<'n> {
    opt: ArgMatches<'n>,
    hub: api::Appengine<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, hyper::Client>>,
    gp: Vec<&'static str>,
    gpm: Vec<(&'static str, &'static str)>,
}


impl<'n> Engine<'n> {
    fn _apps_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "default-hostname" => Some(("defaultHostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "code-bucket" => Some(("codeBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-bucket" => Some(("defaultBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-cookie-expiration" => Some(("defaultCookieExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-domain" => Some(("authDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auth-domain", "code-bucket", "default-bucket", "default-cookie-expiration", "default-hostname", "id", "location-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Application = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().create(request);
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().get(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_locations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().locations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("locations-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_locations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().locations_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_operations_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("operations-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_operations_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().operations_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                "filter" => {
                    call = call.filter(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["filter", "page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "default-hostname" => Some(("defaultHostname", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "code-bucket" => Some(("codeBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-bucket" => Some(("defaultBucket", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "default-cookie-expiration" => Some(("defaultCookieExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "location-id" => Some(("locationId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "auth-domain" => Some(("authDomain", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["auth-domain", "code-bucket", "default-bucket", "default-cookie-expiration", "default-hostname", "id", "location-id", "name"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Application = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().patch(request, opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_repair(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec![]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::RepairApplicationRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().repair(request, opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_list(opt.value_of("apps-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "split.shard-by" => Some(("split.shardBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "split.allocations" => Some(("split.allocations", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Map })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["allocations", "id", "name", "shard-by", "split"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Service = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
                },
                "migrate-traffic" => {
                    call = call.migrate_traffic(arg_from_str(value.unwrap_or("false"), err, "migrate-traffic", "boolean"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["migrate-traffic", "update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_create(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "endpoints-api-service.config-id" => Some(("endpointsApiService.configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.name" => Some(("endpointsApiService.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-url" => Some(("versionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-usage-bytes" => Some(("diskUsageBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-second" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-second" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-second" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-second" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "created-by" => Some(("createdBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.zip.files-count" => Some(("deployment.zip.filesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "deployment.zip.source-url" => Some(("deployment.zip.sourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.subnetwork-name" => Some(("network.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "config-id", "container", "cool-down-period", "cpu", "cpu-utilization", "create-time", "created-by", "default-expiration", "deployment", "disable-health-check", "disk-gb", "disk-usage-bytes", "disk-utilization", "endpoints-api-service", "env", "env-variables", "files-count", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "instance-class", "instance-tag", "instances", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "request-utilization", "resources", "restart-threshold", "runtime", "script", "security-level", "serving-status", "source-url", "subnetwork-name", "target-concurrent-requests", "target-read-bytes-per-second", "target-read-ops-per-second", "target-received-bytes-per-second", "target-received-packets-per-second", "target-request-count-per-second", "target-sent-bytes-per-second", "target-sent-packets-per-second", "target-utilization", "target-write-bytes-per-second", "target-write-ops-per-second", "threadsafe", "timeout", "unhealthy-threshold", "url", "version-url", "vm", "zip"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_versions_create(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["view"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_instances_debug(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "ssh-key" => Some(("sshKey", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["ssh-key"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::DebugInstanceRequest = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_versions_instances_debug(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_instances_delete(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_instances_delete(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_instances_get(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_instances_get(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""), opt.value_of("instances-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_instances_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_instances_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-token", "page-size"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_list(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        let mut call = self.hub.apps().services_versions_list(opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "view" => {
                    call = call.view(value.unwrap_or(""));
                },
                "page-token" => {
                    call = call.page_token(value.unwrap_or(""));
                },
                "page-size" => {
                    call = call.page_size(arg_from_str(value.unwrap_or("-0"), err, "page-size", "integer"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["page-token", "page-size", "view"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _apps_services_versions_patch(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "endpoints-api-service.config-id" => Some(("endpointsApiService.configId", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "endpoints-api-service.name" => Some(("endpointsApiService.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.idle-timeout" => Some(("basicScaling.idleTimeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "basic-scaling.max-instances" => Some(("basicScaling.maxInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "beta-settings" => Some(("betaSettings", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "vm" => Some(("vm", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "instance-class" => Some(("instanceClass", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "version-url" => Some(("versionUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.url" => Some(("apiConfig.url", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.security-level" => Some(("apiConfig.securityLevel", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.auth-fail-action" => Some(("apiConfig.authFailAction", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.login" => Some(("apiConfig.login", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "api-config.script" => Some(("apiConfig.script", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "threadsafe" => Some(("threadsafe", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "health-check.restart-threshold" => Some(("healthCheck.restartThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.check-interval" => Some(("healthCheck.checkInterval", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.unhealthy-threshold" => Some(("healthCheck.unhealthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.healthy-threshold" => Some(("healthCheck.healthyThreshold", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "health-check.host" => Some(("healthCheck.host", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.timeout" => Some(("healthCheck.timeout", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "health-check.disable-health-check" => Some(("healthCheck.disableHealthCheck", JsonTypeInfo { jtype: JsonType::Boolean, ctype: ComplexType::Pod })),
                    "default-expiration" => Some(("defaultExpiration", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "id" => Some(("id", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env" => Some(("env", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "disk-usage-bytes" => Some(("diskUsageBytes", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-pending-latency" => Some(("automaticScaling.minPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-packets-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-packets-per-second" => Some(("automaticScaling.networkUtilization.targetSentPacketsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-received-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetReceivedBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.network-utilization.target-sent-bytes-per-second" => Some(("automaticScaling.networkUtilization.targetSentBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-ops-per-second" => Some(("automaticScaling.diskUtilization.targetReadOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetWriteBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-read-bytes-per-second" => Some(("automaticScaling.diskUtilization.targetReadBytesPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.disk-utilization.target-write-ops-per-second" => Some(("automaticScaling.diskUtilization.targetWriteOpsPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-pending-latency" => Some(("automaticScaling.maxPendingLatency", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-idle-instances" => Some(("automaticScaling.maxIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-idle-instances" => Some(("automaticScaling.minIdleInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-concurrent-requests" => Some(("automaticScaling.requestUtilization.targetConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.request-utilization.target-request-count-per-second" => Some(("automaticScaling.requestUtilization.targetRequestCountPerSecond", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cool-down-period" => Some(("automaticScaling.coolDownPeriod", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-total-instances" => Some(("automaticScaling.maxTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.max-concurrent-requests" => Some(("automaticScaling.maxConcurrentRequests", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.min-total-instances" => Some(("automaticScaling.minTotalInstances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.target-utilization" => Some(("automaticScaling.cpuUtilization.targetUtilization", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "automatic-scaling.cpu-utilization.aggregation-window-length" => Some(("automaticScaling.cpuUtilization.aggregationWindowLength", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "env-variables" => Some(("envVariables", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Map })),
                    "resources.disk-gb" => Some(("resources.diskGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.cpu" => Some(("resources.cpu", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "resources.memory-gb" => Some(("resources.memoryGb", JsonTypeInfo { jtype: JsonType::Float, ctype: ComplexType::Pod })),
                    "manual-scaling.instances" => Some(("manualScaling.instances", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "inbound-services" => Some(("inboundServices", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "created-by" => Some(("createdBy", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.container.image" => Some(("deployment.container.image", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "deployment.zip.files-count" => Some(("deployment.zip.filesCount", JsonTypeInfo { jtype: JsonType::Int, ctype: ComplexType::Pod })),
                    "deployment.zip.source-url" => Some(("deployment.zip.sourceUrl", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "create-time" => Some(("createTime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.instance-tag" => Some(("network.instanceTag", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.forwarded-ports" => Some(("network.forwardedPorts", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Vec })),
                    "network.name" => Some(("network.name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "network.subnetwork-name" => Some(("network.subnetworkName", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "nobuild-files-regex" => Some(("nobuildFilesRegex", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "name" => Some(("name", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "serving-status" => Some(("servingStatus", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "runtime" => Some(("runtime", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["aggregation-window-length", "api-config", "auth-fail-action", "automatic-scaling", "basic-scaling", "beta-settings", "check-interval", "config-id", "container", "cool-down-period", "cpu", "cpu-utilization", "create-time", "created-by", "default-expiration", "deployment", "disable-health-check", "disk-gb", "disk-usage-bytes", "disk-utilization", "endpoints-api-service", "env", "env-variables", "files-count", "forwarded-ports", "health-check", "healthy-threshold", "host", "id", "idle-timeout", "image", "inbound-services", "instance-class", "instance-tag", "instances", "login", "manual-scaling", "max-concurrent-requests", "max-idle-instances", "max-instances", "max-pending-latency", "max-total-instances", "memory-gb", "min-idle-instances", "min-pending-latency", "min-total-instances", "name", "network", "network-utilization", "nobuild-files-regex", "request-utilization", "resources", "restart-threshold", "runtime", "script", "security-level", "serving-status", "source-url", "subnetwork-name", "target-concurrent-requests", "target-read-bytes-per-second", "target-read-ops-per-second", "target-received-bytes-per-second", "target-received-packets-per-second", "target-request-count-per-second", "target-sent-bytes-per-second", "target-sent-packets-per-second", "target-utilization", "target-write-bytes-per-second", "target-write-ops-per-second", "threadsafe", "timeout", "unhealthy-threshold", "url", "version-url", "vm", "zip"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::Version = json::value::from_value(object).unwrap();
        let mut call = self.hub.apps().services_versions_patch(request, opt.value_of("apps-id").unwrap_or(""), opt.value_of("services-id").unwrap_or(""), opt.value_of("versions-id").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "update-mask" => {
                    call = call.update_mask(value.unwrap_or(""));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["update-mask"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit(),
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema);
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }

    fn _doit(&self, dry_run: bool) -> Result<Result<(), DoitError>, Option<InvalidOptionsError>> {
        let mut err = InvalidOptionsError::new();
        let mut call_result: Result<(), DoitError> = Ok(());
        let mut err_opt: Option<InvalidOptionsError> = None;
        match self.opt.subcommand() {
            ("apps", Some(opt)) => {
                match opt.subcommand() {
                    ("create", Some(opt)) => {
                        call_result = self._apps_create(opt, dry_run, &mut err);
                    },
                    ("get", Some(opt)) => {
                        call_result = self._apps_get(opt, dry_run, &mut err);
                    },
                    ("locations-get", Some(opt)) => {
                        call_result = self._apps_locations_get(opt, dry_run, &mut err);
                    },
                    ("locations-list", Some(opt)) => {
                        call_result = self._apps_locations_list(opt, dry_run, &mut err);
                    },
                    ("operations-get", Some(opt)) => {
                        call_result = self._apps_operations_get(opt, dry_run, &mut err);
                    },
                    ("operations-list", Some(opt)) => {
                        call_result = self._apps_operations_list(opt, dry_run, &mut err);
                    },
                    ("patch", Some(opt)) => {
                        call_result = self._apps_patch(opt, dry_run, &mut err);
                    },
                    ("repair", Some(opt)) => {
                        call_result = self._apps_repair(opt, dry_run, &mut err);
                    },
                    ("services-delete", Some(opt)) => {
                        call_result = self._apps_services_delete(opt, dry_run, &mut err);
                    },
                    ("services-get", Some(opt)) => {
                        call_result = self._apps_services_get(opt, dry_run, &mut err);
                    },
                    ("services-list", Some(opt)) => {
                        call_result = self._apps_services_list(opt, dry_run, &mut err);
                    },
                    ("services-patch", Some(opt)) => {
                        call_result = self._apps_services_patch(opt, dry_run, &mut err);
                    },
                    ("services-versions-create", Some(opt)) => {
                        call_result = self._apps_services_versions_create(opt, dry_run, &mut err);
                    },
                    ("services-versions-delete", Some(opt)) => {
                        call_result = self._apps_services_versions_delete(opt, dry_run, &mut err);
                    },
                    ("services-versions-get", Some(opt)) => {
                        call_result = self._apps_services_versions_get(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-debug", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_debug(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-delete", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_delete(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-get", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_get(opt, dry_run, &mut err);
                    },
                    ("services-versions-instances-list", Some(opt)) => {
                        call_result = self._apps_services_versions_instances_list(opt, dry_run, &mut err);
                    },
                    ("services-versions-list", Some(opt)) => {
                        call_result = self._apps_services_versions_list(opt, dry_run, &mut err);
                    },
                    ("services-versions-patch", Some(opt)) => {
                        call_result = self._apps_services_versions_patch(opt, dry_run, &mut err);
                    },
                    _ => {
                        err.issues.push(CLIError::MissingMethodError("apps".to_string()));
                        writeln!(io::stderr(), "{}\n", opt.usage()).ok();
                    }
                }
            },
            _ => {
                err.issues.push(CLIError::MissingCommandError);
                writeln!(io::stderr(), "{}\n", self.opt.usage()).ok();
            }
        }

        if dry_run {
            if err.issues.len() > 0 {
                err_opt = Some(err);
            }
            Err(err_opt)
        } else {
            Ok(call_result)
        }
    }

    // Please note that this call will fail if any part of the opt can't be handled
    fn new(opt: ArgMatches<'n>) -> Result<Engine<'n>, InvalidOptionsError> {
        let (config_dir, secret) = {
            let config_dir = match cmn::assure_config_dir_exists(opt.value_of("folder").unwrap_or("~/.google-service-cli")) {
                Err(e) => return Err(InvalidOptionsError::single(e, 3)),
                Ok(p) => p,
            };

            match cmn::application_secret_from_directory(&config_dir, "appengine1-secret.json",
                                                         "{\"installed\":{\"auth_uri\":\"https://accounts.google.com/o/oauth2/auth\",\"client_secret\":\"hCsslbCUyfehWMmbkG8vTYxG\",\"token_uri\":\"https://accounts.google.com/o/oauth2/token\",\"client_email\":\"\",\"redirect_uris\":[\"urn:ietf:wg:oauth:2.0:oob\",\"oob\"],\"client_x509_cert_url\":\"\",\"client_id\":\"620010449518-9ngf7o4dhs0dka470npqvor6dc5lqb9b.apps.googleusercontent.com\",\"auth_provider_x509_cert_url\":\"https://www.googleapis.com/oauth2/v1/certs\"}}") {
                Ok(secret) => (config_dir, secret),
                Err(e) => return Err(InvalidOptionsError::single(e, 4))
            }
        };

        let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                        if opt.is_present("debug-auth") {
                                            hyper::Client::with_connector(mock::TeeConnector {
                                                    connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                                                })
                                        } else {
                                            hyper::Client::new()
                                        },
                                        JsonTokenStorage {
                                          program_name: "appengine1",
                                          db_dir: config_dir.clone(),
                                        }, Some(FlowType::InstalledRedirect(54324)));

        let client =
            if opt.is_present("debug") {
                hyper::Client::with_connector(mock::TeeConnector {
                        connector: hyper::net::HttpsConnector::<hyper::net::Openssl>::default()
                    })
            } else {
                hyper::Client::new()
            };
        let engine = Engine {
            opt: opt,
            hub: api::Appengine::new(client, auth),
            gp: vec!["$-xgafv", "access-token", "alt", "bearer-token", "callback", "fields", "key", "oauth-token", "pp", "pretty-print", "quota-user", "upload-type", "upload-protocol"],
            gpm: vec![
                    ("$-xgafv", "$.xgafv"),
                    ("access-token", "access_token"),
                    ("bearer-token", "bearer_token"),
                    ("oauth-token", "oauth_token"),
                    ("pretty-print", "prettyPrint"),
                    ("quota-user", "quotaUser"),
                    ("upload-type", "uploadType"),
                    ("upload-protocol", "upload_protocol"),
                ]
        };

        match engine._doit(true) {
            Err(Some(err)) => Err(err),
            Err(None)      => Ok(engine),
            Ok(_)          => unreachable!(),
        }
    }

    fn doit(&self) -> Result<(), DoitError> {
        match self._doit(false) {
            Ok(res) => res,
            Err(_) => unreachable!(),
        }
    }
}

fn main() {
    let mut exit_status = 0i32;
    let arg_data = [
        ("apps", "methods: 'create', 'get', 'locations-get', 'locations-list', 'operations-get', 'operations-list', 'patch', 'repair', 'services-delete', 'services-get', 'services-list', 'services-patch', 'services-versions-create', 'services-versions-delete', 'services-versions-get', 'services-versions-instances-debug', 'services-versions-instances-delete', 'services-versions-instances-get', 'services-versions-instances-list', 'services-versions-list' and 'services-versions-patch'", vec![
            ("create",
                    Some(r##"Creates an App Engine application for a Google Cloud Platform project. This requires a project that excludes an App Engine application. For details about creating a project without an application, see the Google Cloud Resource Manager create project topic (https://cloud.google.com/resource-manager/docs/creating-project)."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_create",
                  vec![
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("get",
                    Some(r##"Gets information about an application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Application resource to get. Example: apps/myapp."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-get",
                    Some(r##"Get information about a location."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_locations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Resource name for the location."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"locations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("locations-list",
                    Some(r##"Lists information about the supported locations for this service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_locations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The resource that owns the locations collection, if applicable."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("operations-get",
                    Some(r##"Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_operations-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation resource."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"operations-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("operations-list",
                    Some(r##"Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding below allows API services to override the binding to use different resource name schemes, such as users/*/operations."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_operations-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. The name of the operation collection."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("patch",
                    Some(r##"Updates the specified Application resource. You can update the following fields: auth_domain (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps#Application.FIELDS.auth_domain) default_cookie_expiration (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps#Application.FIELDS.default_cookie_expiration)"##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the Application resource to update. Example: apps/myapp."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("repair",
                    Some(r##"Recreates the required App Engine features for the specified App Engine application, for example a Cloud Storage bucket or App Engine service account. Use this method if you receive an error message about a missing feature, for example, Error retrieving the App Engine service account."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_repair",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the application to repair. Example: apps/myapp"##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-delete",
                    Some(r##"Deletes the specified service and all enclosed versions."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-get",
                    Some(r##"Gets the current configuration of the specified service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-list",
                    Some(r##"Lists all the services in the application."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Application resource. Example: apps/myapp."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-patch",
                    Some(r##"Updates the configuration of the specified service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-create",
                    Some(r##"Deploys code and resource files to a new version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-create",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent resource to create this version under. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-delete",
                    Some(r##"Deletes an existing Version resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-get",
                    Some(r##"Gets the specified Version resource. By default, only a BASIC_VIEW will be returned. Specify the FULL_VIEW parameter to get the full resource."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-instances-debug",
                    Some(r##"Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-debug",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-instances-delete",
                    Some(r##"Stops a running instance."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-delete",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-instances-get",
                    Some(r##"Gets instance information."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-get",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"instances-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-instances-list",
                    Some(r##"Lists the instances of a version."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-instances-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Version resource. Example: apps/myapp/services/default/versions/v1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-list",
                    Some(r##"Lists the versions of a service."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-list",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `parent`. Name of the parent Service resource. Example: apps/myapp/services/default."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `parent`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ("services-versions-patch",
                    Some(r##"Updates the specified Version resource. You can specify the following fields depending on the App Engine environment and type of scaling that the version resource uses: serving_status (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status): For Version resources that use basic scaling, manual scaling, or run in the App Engine flexible environment. instance_class (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class): For Version resources that run in the App Engine standard environment. automatic_scaling.min_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling): For Version resources that use automatic scaling and run in the App Engine standard environment. automatic_scaling.max_idle_instances (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling): For Version resources that use automatic scaling and run in the App Engine standard environment."##),
                    "Details at http://byron.github.io/google-apis-rs/google_appengine1_cli/apps_services-versions-patch",
                  vec![
                    (Some(r##"apps-id"##),
                     None,
                     Some(r##"Part of `name`. Name of the resource to update. Example: apps/myapp/services/default/versions/1."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"services-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"versions-id"##),
                     None,
                     Some(r##"Part of `name`. See documentation of `appsId`."##),
                     Some(true),
                     Some(false)),
        
                    (Some(r##"kv"##),
                     Some(r##"r"##),
                     Some(r##"Set various fields of the request structure, matching the key=value form"##),
                     Some(true),
                     Some(true)),
        
                    (Some(r##"v"##),
                     Some(r##"p"##),
                     Some(r##"Set various optional parameters, matching the key=value form"##),
                     Some(false),
                     Some(true)),
        
                    (Some(r##"out"##),
                     Some(r##"o"##),
                     Some(r##"Specify the file into which to write the program's output"##),
                     Some(false),
                     Some(false)),
                  ]),
            ]),
        
    ];
    
    let mut app = App::new("appengine1")
           .author("Sebastian Thiel <byronimo@gmail.com>")
           .version("1.0.0+20161208")
           .about("Provisions and manages App Engine applications.")
           .after_help("All documentation details can be found at http://byron.github.io/google-apis-rs/google_appengine1_cli")
           .arg(Arg::with_name("url")
                   .long("scope")
                   .help("Specify the authentication a method should be executed in. Each scope requires the user to grant this application permission to use it.If unset, it defaults to the shortest scope url for a particular method.")
                   .multiple(true)
                   .takes_value(true))
           .arg(Arg::with_name("folder")
                   .long("config-dir")
                   .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation.[default: ~/.google-service-cli")
                   .multiple(false)
                   .takes_value(true))
           .arg(Arg::with_name("debug")
                   .long("debug")
                   .help("Output all server communication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false))
           .arg(Arg::with_name("debug-auth")
                   .long("debug-auth")
                   .help("Output all communication related to authentication to standard error. `tx` and `rx` are placed into the same stream.")
                   .multiple(false)
                   .takes_value(false));
           
           for &(main_command_name, about, ref subcommands) in arg_data.iter() {
               let mut mcmd = SubCommand::with_name(main_command_name).about(about);
           
               for &(sub_command_name, ref desc, url_info, ref args) in subcommands {
                   let mut scmd = SubCommand::with_name(sub_command_name);
                   if let &Some(desc) = desc {
                       scmd = scmd.about(desc);
                   }
                   scmd = scmd.after_help(url_info);
           
                   for &(ref arg_name, ref flag, ref desc, ref required, ref multi) in args {
                       let arg_name_str =
                           match (arg_name, flag) {
                                   (&Some(an), _       ) => an,
                                   (_        , &Some(f)) => f,
                                    _                    => unreachable!(),
                            };
                       let mut arg = Arg::with_name(arg_name_str)
                                         .empty_values(false);
                       if let &Some(short_flag) = flag {
                           arg = arg.short(short_flag);
                       }
                       if let &Some(desc) = desc {
                           arg = arg.help(desc);
                       }
                       if arg_name.is_some() && flag.is_some() {
                           arg = arg.takes_value(true);
                       }
                       if let &Some(required) = required {
                           arg = arg.required(required);
                       }
                       if let &Some(multi) = multi {
                           arg = arg.multiple(multi);
                       }
                       scmd = scmd.arg(arg);
                   }
                   mcmd = mcmd.subcommand(scmd);
               }
               app = app.subcommand(mcmd);
           }
           
        let matches = app.get_matches();

    let debug = matches.is_present("debug");
    match Engine::new(matches) {
        Err(err) => {
            exit_status = err.exit_code;
            writeln!(io::stderr(), "{}", err).ok();
        },
        Ok(engine) => {
            if let Err(doit_err) = engine.doit() {
                exit_status = 1;
                match doit_err {
                    DoitError::IoError(path, err) => {
                        writeln!(io::stderr(), "Failed to open output file '{}': {}", path, err).ok();
                    },
                    DoitError::ApiError(err) => {
                        if debug {
                            writeln!(io::stderr(), "{:#?}", err).ok();
                        } else {
                            writeln!(io::stderr(), "{}", err).ok();
                        }
                    }
                }
            }
        }
    }

    std::process::exit(exit_status);
}