//! Package to use MailboxValidator API for email validation.
//! It enables user to easily validate if an email address is valid, 
//! a type of disposable email or free email.
//! 
//! This module can be useful in many types of projects, for example
//! 
//! - to validate an user's email during sign up
//! - to clean your mailing list prior to email sending
//! - to perform fraud check
//! - and so on
//! 
//! You can get a free API key from here: <https://www.mailboxvalidator.com/plans#api>.
//! 
//! # Example
//!
//! ```
//! use mailboxvalidator;
//!
//! let validation_result = mailboxvalidator::validate_email("example@example.com",YOUR_API_KEY);
//!
//! match validation_result {
//!     Ok(num) => {
//!         let ok_result = num;
//!         assert_eq!(ok_result["status"], "False");
//!         assert_eq!(ok_result["error_code"], "");
//!     },
//!     Err(err) => println!("{:#?}", err),
//! };
//! ```

#![doc(html_root_url = "https://docs.rs/mailboxvalidator/1.1.1")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::Deserialize;
use serde::Serialize;

use reqwest::StatusCode;

pub use reqwest::Error as ReqError;

// #[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
// pub enum ALLELE {
    // bool(bool),
    // null(null),
// }

///! Wrapper result type returning `reqwest` errors
pub type MailboxValidatorResult<T> = Result<T, ReqError>;

/// MailboxValidator Single Validation API result record.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SingleEmailValidationRecord {
    email_address: String,
    base_email_address: String,
    domain: String,
    is_free: Option<bool>,
    is_syntax: Option<bool>,
    is_domain: Option<bool>,
    is_smtp: Option<bool>,
    is_verified: Option<bool>,
    is_server_down: Option<bool>,
    is_greylisted: Option<bool>,
    is_disposable: Option<bool>,
    is_suppressed: Option<bool>,
    is_role: Option<bool>,
    is_high_risk: Option<bool>,
    is_catchall: Option<bool>,
    is_dmarc_enforced: Option<bool>,
    is_strict_spf: Option<bool>,
    website_exist: Option<bool>,
    status: Option<bool>,
    mailboxvalidator_score: f64,
    time_taken: f64,
    credits_available: i64,
}

/// MailboxValidator Disposable Email API result record.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DisposableEmailRecord {
    email_address: String,
    is_disposable: Option<bool>,
    credits_available: i64,
}

/// MailboxValidator Free Email API result record.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FreeEmailRecord {
    email_address: String,
    is_free: Option<bool>,
    credits_available: i64,
}

/// MailboxValidator Error object
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecord {
    error: ErrorRecord1,
}

/// MailboxValidator Error Response object 
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecord1 {
    error_code: i64,
    error_message: String,
}

/// Validates email address using MailboxValidator Single Validation API.
///
/// # Examples
///
/// ```
/// let validation_result = mailboxvalidator::validate_email("example@example.com",YOUR_API_KEY);
///
/// match validation_result {
///     Ok(num) => {
///         let ok_result = num;
///         assert_eq!(ok_result["status"], "False");
///         assert_eq!(ok_result["error_code"], "");
///     },
///     Err(err) => println!("{:#?}", err),
/// };
/// ```
///
/// # Errors
///
/// * Error when connecting to MailboxValidator API.
pub fn validate_email(email_address: &str, apikey: &str) -> MailboxValidatorResult<serde_json::value::Value>  {
    let client = reqwest::blocking::Client::new();
    let url = format!(
            "https://api.mailboxvalidator.com/v2/validation/single?email={}&key={}&format=json&source=sdk-rust-mbv",
            email_address, apikey
        );
    let res = client
    .get(url)
    .send()?;

    if res.status() == StatusCode::OK {
		let parsed: SingleEmailValidationRecord = res.json()?;
		let json_value = serde_json::json!(&parsed);
		return Ok(json_value);		
	} else if (res.status() == StatusCode::BAD_REQUEST) || (res.status() == StatusCode::UNAUTHORIZED) {
		let parsed: ErrorRecord = res.json()?;
		let json_value = serde_json::json!(&parsed);
		return Ok(json_value);
	} else {
		println!("Something else happened. Status: {:?}", res.status());
	}

    // Ok(())
    Ok(().into())
}

/// Validates email address using MailboxValidator Disposable Email API.
///
/// # Examples
///
/// ```
/// let validation_result = mailboxvalidator::validate_email("example@example.com",YOUR_API_KEY);
///
/// match validation_result {
///     Ok(num) => {
///         let ok_result = num;
///         assert_eq!(ok_result["is_disposable"], "True");
///         assert_eq!(ok_result["error_code"], "");
///     },
///     Err(err) => println!("{:#?}", err),
/// };
/// ```
///
/// # Errors
///
/// * Error when connecting to MailboxValidator API.
pub fn is_disposable_email(email_address: &str, apikey: &str) -> MailboxValidatorResult<serde_json::value::Value>  {
    let client1 = reqwest::blocking::Client::new();
    let url = format!(
            "https://api.mailboxvalidator.com/v2/email/disposable?email={}&key={}&format=json&source=sdk-rust-mbv",
            email_address, apikey
        );
    let res = client1
    .get(url)
    .send()?;

    if res.status() == StatusCode::OK {
		let parsed: DisposableEmailRecord = res.json()?;
		let json_value = serde_json::json!(&parsed);
		return Ok(json_value);		
	} else if (res.status() == StatusCode::BAD_REQUEST) || (res.status() == StatusCode::UNAUTHORIZED) {
		let parsed: ErrorRecord = res.json()?;
		let json_value = serde_json::json!(&parsed);
		return Ok(json_value);
	} else {
		println!("Something else happened. Status: {:?}", res.status());
	}

    // Ok(())
    Ok(().into())
}

/// Validates email address using MailboxValidator Free Email API.
///
/// # Examples
///
/// ```
/// let validation_result = mailboxvalidator::validate_email(YOUR_EMAIL_ADDRESS,YOUR_API_KEY);
///
/// match validation_result {
///     Ok(num) => {
///         let ok_result = num;
///         assert_eq!(ok_result["is_free"], "False");
///         assert_eq!(ok_result["error_code"], "");
///     },
///     Err(err) => println!("{:#?}", err),
/// };
/// ```
///
/// # Errors
///
/// * Error when connecting to MailboxValidator API.
pub fn is_free_email(email_address: &str, apikey: &str) -> MailboxValidatorResult<serde_json::value::Value>  {
    let client = reqwest::blocking::Client::new();
    let url = format!(
            "https://api.mailboxvalidator.com/v2/email/free?email={}&key={}&format=json&source=sdk-rust-mbv",
            email_address, apikey
        );
    let res = client
    .get(url)
    .send()?;

    if res.status() == StatusCode::OK {
		let parsed: FreeEmailRecord = res.json()?;
		let json_value = serde_json::json!(&parsed);
		return Ok(json_value);		
	} else if (res.status() == StatusCode::BAD_REQUEST) || (res.status() == StatusCode::UNAUTHORIZED) {
		let parsed: ErrorRecord = res.json()?;
		let json_value = serde_json::json!(&parsed);
		return Ok(json_value);
	} else {
		println!("Something else happened. Status: {:?}", res.status());
	}

    // Ok(())
    Ok(().into())
}