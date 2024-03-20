# Quickstart

## Dependencies

An API key is required for this module to function.

Go to https://www.mailboxvalidator.com/plans#api to sign up for FREE API plan and you'll be given an API key.

## Installation

Just add `mailboxvalidator = "1.1.1"` into your *Cargo.toml*.

## Sample Codes

### Validate email

You can validate whether an email address is invalid or not as below:

```rust
use mailboxvalidator;

let validation_result = mailboxvalidator::validate_email("example@example.com",PASTE_API_KEY_HERE);

match validation_result {
    Ok(num) => {
        let ok_result = num;
        println!("{:#?}", ok_result);
    },
    Err(err) => println!("{:#?}", err),
};
```

### Check if an email is from a disposable email provider

You can validate whether an email address is disposable email address or not as below:

```rust
use mailboxvalidator;

let validation_result = mailboxvalidator::is_disposable_email("example@example.com",PASTE_API_KEY_HERE);

match validation_result {
    Ok(num) => {
        let ok_result = num;
        println!("{:#?}", ok_result);
    },
    Err(err) => println!("{:#?}", err),
};
```

### Check if an email is from a free email provider

You can validate whether an email address is free email address or not as below:

```rust
use mailboxvalidator;

let validation_result = mailboxvalidator::is_free_email("example@example.com",PASTE_API_KEY_HERE);

match validation_result {
    Ok(num) => {
        let ok_result = num;
        println!("{:#?}", ok_result);
    },
    Err(err) => println!("{:#?}", err),
};
```
