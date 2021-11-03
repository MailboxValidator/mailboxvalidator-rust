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

#![doc(html_root_url = "https://docs.rs/mailboxvalidator/1.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use serde::Deserialize;
use serde::Serialize;

pub use reqwest::Error as ReqError;

///! Wrapper result type returning `reqwest` errors
pub type MailboxValidatorResult<T> = Result<T, ReqError>;

/// MailboxValidator Single Validation API result record.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct SingleEmailValidationRecord {
    email_address: String,
    domain: String,
    is_free: String,
    is_syntax: String,
    is_domain: String,
    is_smtp: String,
    is_verified: String,
    is_server_down: String,
    is_greylisted: String,
    is_disposable: String,
    is_suppressed: String,
    is_role: String,
    is_high_risk: String,
    is_catchall: String,
    status: String,
    error_code: String,
    error_message: String,
    mailboxvalidator_score: String,
    time_taken: String,
    credits_available: i64,
}

/// MailboxValidator Disposable Email API result record.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct DisposableEmailRecord {
    email_address: String,
    is_disposable: String,
    error_code: String,
    error_message: String,
    credits_available: i64,
}

/// MailboxValidator Free Email API result record.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FreeEmailRecord {
    email_address: String,
    is_free: String,
    error_code: String,
    error_message: String,
    credits_available: i64,
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
            "https://api.mailboxvalidator.com/v1/validation/single?email={}&key={}&format=json",
            email_address, apikey
        );
    let res = client
    .get(url)
    .send()?
    .error_for_status()?;

    let parsed: SingleEmailValidationRecord = res.json()?;
    let json_value = serde_json::json!(&parsed);
    Ok(json_value)
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
            "https://api.mailboxvalidator.com/v1/email/disposable?email={}&key={}&format=json",
            email_address, apikey
        );
    let res = client1
    .get(url)
    .send()?
    .error_for_status()?;

    let parsed: DisposableEmailRecord = res.json()?;
    let json_value = serde_json::json!(&parsed);
    Ok(json_value)
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
            "https://api.mailboxvalidator.com/v1/email/free?email={}&key={}&format=json",
            email_address, apikey
        );
    let res = client
    .get(url)
    .send()?
    .error_for_status()?;

    let parsed: FreeEmailRecord = res.json()?;
    let json_value = serde_json::json!(&parsed);
    Ok(json_value)
}