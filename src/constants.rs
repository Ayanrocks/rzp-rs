#[allow(dead_code)]
pub const SDK_VERSION: &str = "0.0.1";
pub const SDK_NAME: &str = "rzp-rs";

pub const BASE_URL: &str = "https://api.razorpay.com/v1";

/// orders api base path
pub const ORDER_URL: &str = "/orders";
/// invoices api base path
pub const INVOICE_URL: &str = "/invoices";
/// payments api base pth
pub const PAYMENT_URL: &str = "/payments";
/// refunds api base path
pub const REFUND_URL: &str = "/refunds";
/// cards api base path
pub const CARD_URL: &str = "/cards";
/// customers api base path
pub const CUSTOMER_URL: &str = "/customers";
/// addon api base path
pub const ADDON_URL: &str = "/addons";
/// transfers api base path
pub const TRANSFER_URL: &str = "/transfers";
/// virtual accounts base path
pub const VIRTUAL_ACCOUNT_URL: &str = "/virtual_accounts";
/// subscriptions base path
pub const SUBSCRIPTION_URL: &str = "/subscriptions";
/// plan api base path
pub const PLAN_URL: &str = "/plans";


/// Error Codes
pub const ERR_JSON_PARSING: String = String::from("error_parsing_json");
pub const ERR_SENDING_REQUEST: String = String::from("request_sending_failure");
pub const ERR_RAZORPAY_ERROR: String = String::from("razorpay_error");


/// Error Descriptions
pub const ERRDESC_RAZORPAY_ERROR: String = String::from("Something went wrong on Razorpay's side");
pub const ERRDESC_JSON_PARSING: String = String::from("Error parsing json response with serde");
pub const ERRDESC_SENDING_REQUEST: String = String::from("Error sending request, check error description for more details");
