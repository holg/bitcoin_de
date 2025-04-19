// bitcoin_de_trading_api_sdk_v4/constants.rs
/// HTTP method constants used for API requests.
///
/// These constants define the standard HTTP methods supported by the Bitcoin.de API:
/// - GET: Used for retrieving data without modifying resources
/// - POST: Used for creating new resources or submitting data
/// - PUT: Used for updating existing resources (not used in current API version)
/// - DELETE: Used for removing resources
pub const HTTP_METHOD_GET: &str = "GET";
pub const HTTP_METHOD_POST: &str = "POST";
pub const HTTP_METHOD_PUT: &str = "PUT"; // actually not needed for this API version
pub const HTTP_METHOD_DELETE: &str = "DELETE";

/// The version number of the Bitcoin.de API being used.
///
/// This constant defines the API version that the SDK is designed to work with.
/// It is used in request URLs and headers to ensure compatibility with the
/// Bitcoin.de trading platform's API endpoints.
///
/// The current supported version is "4".
pub const API_VERSION: &str = "4";

/// Error codes returned by the Bitcoin.de API.
///
/// These constants define the possible error codes that can be returned by the Bitcoin.de API
/// when a request fails. Each error code corresponds to a specific error condition.
///
/// # Error Categories
///
/// - 1-8: Authentication and general request errors
/// - 9-17: Resource not found errors
/// - 18-25: Validation errors for orders and trades
/// - 26-44: Invalid ID and state errors
/// - 45-57: Resource existence and pending operation errors
/// - 58-90: User account and authentication errors
/// - 91-119: KYC and company information errors
/// - 120-208: Format and data type validation errors
/// - 9998-9999: System-level errors

/// Error code indicating a required HTTP header is missing from the request.
pub const ERROR_CODE_MISSING_HEADER: i32 = 1;
/// Error code indicating the API key provided is inactive or invalid.
pub const ERROR_CODE_INACTIVE_API_KEY: i32 = 2;
/// Error code indicating the request signature is incorrect or invalid.
pub const ERROR_CODE_WRONG_SIGNATURE: i32 = 3;
/// Error code indicating a required parameter is missing from a POST request.
pub const ERROR_CODE_MISSING_POST_PARAMETER: i32 = 4;
/// Error code indicating a required parameter is missing from a GET request.
pub const ERROR_CODE_MISSING_GET_PARAMETER: i32 = 5;
/// Error code indicating the nonce value is invalid or has been used before.
pub const ERROR_CODE_INVALID_NONCE: i32 = 6;
/// Error code indicating the requested API method does not exist.
pub const ERROR_CODE_UNKNOWN_API_METHOD: i32 = 7;
/// Error code indicating the API key does not have permission to access the requested resource.
pub const ERROR_CODE_PERMISSION_DENIED: i32 = 8;
/// Error code indicating the specified trading pair is not available for trading.
pub const ERROR_CODE_TRADING_PAIR_NOT_TRADABLE: i32 = 9;
/// Error code indicating the order type is invalid.
pub const ERROR_CODE_INVALID_ORDER_TYPE: i32 = 10;
/// Error code indicating the amount specified in the request is invalid.
pub const ERROR_CODE_INVALID_AMOUNT: i32 = 11;
/// Error code indicating the price specified in the request is invalid.
pub const ERROR_CODE_INVALID_PRICE: i32 = 12;
/// Error code indicating the requested order could not be found.
pub const ERROR_CODE_ORDER_NOT_FOUND: i32 = 13;
/// Error code indicating the requested trade could not be found.
pub const ERROR_CODE_TRADE_NOT_FOUND: i32 = 14;
/// Error code indicating the requested withdrawal could not be found.
pub const ERROR_CODE_WITHDRAWAL_NOT_FOUND: i32 = 15;
/// Error code indicating the requested deposit could not be found.
pub const ERROR_CODE_DEPOSIT_NOT_FOUND: i32 = 16;
/// Error code indicating the requested address could not be found.
pub const ERROR_CODE_ADDRESS_NOT_FOUND: i32 = 17;
/// Error code indicating the amount specified is below the minimum allowed.
pub const ERROR_CODE_AMOUNT_TOO_LOW: i32 = 18;
/// Error code indicating the amount specified is above the maximum allowed.
pub const ERROR_CODE_AMOUNT_TOO_HIGH: i32 = 19;
/// Error code indicating the price specified is below the minimum allowed.
pub const ERROR_CODE_PRICE_TOO_LOW: i32 = 20;
/// Error code indicating the price specified is above the maximum allowed.
pub const ERROR_CODE_PRICE_TOO_HIGH: i32 = 21;
/// Error code indicating the user has insufficient credits to complete the operation.
pub const ERROR_CODE_INSUFFICIENT_CREDITS: i32 = 22;
/// Error code indicating the volume is insufficient to complete the operation.
pub const ERROR_CODE_INSUFFICIENT_VOLUME: i32 = 23;
/// Error code indicating the payment option specified is invalid.
pub const ERROR_CODE_INVALID_PAYMENT_OPTION: i32 = 24;
/// Error code indicating the rating provided is invalid.
pub const ERROR_CODE_INVALID_RATING: i32 = 25;
/// Error code indicating the order ID provided is invalid.
pub const ERROR_CODE_INVALID_ORDER_ID: i32 = 26;
/// Error code indicating the trade ID provided is invalid.
pub const ERROR_CODE_INVALID_TRADE_ID: i32 = 27;
/// Error code indicating the withdrawal ID provided is invalid.
pub const ERROR_CODE_INVALID_WITHDRAWAL_ID: i32 = 28;
/// Error code indicating the deposit ID provided is invalid.
pub const ERROR_CODE_INVALID_DEPOSIT_ID: i32 = 29;
/// Error code indicating the address ID provided is invalid.
pub const ERROR_CODE_INVALID_ADDRESS_ID: i32 = 30;
/// Error code indicating the currency specified is invalid.
pub const ERROR_CODE_INVALID_CURRENCY: i32 = 31;
/// Error code indicating the trading pair specified is invalid.
pub const ERROR_CODE_INVALID_TRADING_PAIR: i32 = 32;
/// Error code indicating the order payment options are invalid.
pub const ERROR_CODE_INVALID_ORDER_PAYMENT_OPTIONS: i32 = 33;
/// Error code indicating the account ledger type is invalid.
pub const ERROR_CODE_INVALID_ACCOUNT_LEDGER_TYPE: i32 = 34;
/// Error code indicating the trust level specified is invalid.
pub const ERROR_CODE_INVALID_TRUST_LEVEL: i32 = 35;
/// Error code indicating the trade state is invalid.
pub const ERROR_CODE_INVALID_TRADE_STATE: i32 = 36;
/// Error code indicating the order state is invalid.
pub const ERROR_CODE_INVALID_ORDER_STATE: i32 = 37;
/// Error code indicating the withdrawal state is invalid.
pub const ERROR_CODE_INVALID_WITHDRAWAL_STATE: i32 = 38;
/// Error code indicating the deposit state is invalid.
pub const ERROR_CODE_INVALID_DEPOSIT_STATE: i32 = 39;
/// Error code indicating the payment method is invalid.
pub const ERROR_CODE_INVALID_PAYMENT_METHOD: i32 = 40;
/// Error code indicating the withdrawal rejection reason is invalid.
pub const ERROR_CODE_INVALID_WITHDRAWAL_REJECT_REASON: i32 = 41;
/// Error code indicating the deposit rejection reason is invalid.
pub const ERROR_CODE_INVALID_DEPOSIT_REJECT_REASON: i32 = 42;
/// Error code indicating the address pool state is invalid.
pub const ERROR_CODE_INVALID_ADDRESS_POOL_STATE: i32 = 43;
/// Error code indicating the outgoing address state is invalid.
pub const ERROR_CODE_INVALID_OUTGOING_ADDRESS_STATE: i32 = 44;
/// Error code indicating the address pool is empty.
pub const ERROR_CODE_ADDRESS_POOL_IS_EMPTY: i32 = 45;
/// Error code indicating no new address was created.
pub const ERROR_CODE_NO_NEW_ADDRESS_CREATED: i32 = 46;
/// Error code indicating the recipient address is invalid.
pub const ERROR_CODE_INVALID_RECIPIENT_ADDRESS: i32 = 47;
/// Error code indicating the recipient purpose is invalid.
pub const ERROR_CODE_INVALID_RECIPIENT_PURPOSE: i32 = 48;
/// Error code indicating the comment is invalid.
pub const ERROR_CODE_INVALID_COMMENT: i32 = 49;
/// Error code indicating the network fee is invalid.
pub const ERROR_CODE_INVALID_NETWORK_FEE: i32 = 50;
/// Error code indicating the volume currency to pay after fee is invalid.
pub const ERROR_CODE_INVALID_VOLUME_CURRENCY_TO_PAY_AFTER_FEE: i32 = 51;
/// Error code indicating the amount currency to trade after fee is invalid.
pub const ERROR_CODE_INVALID_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE: i32 = 52;
/// Error code indicating the is_paid_from_correct_bank_account flag is invalid.
pub const ERROR_CODE_INVALID_IS_PAID_FROM_CORRECT_BANK_ACCOUNT: i32 = 53;
/// Error code indicating a pending withdrawal already exists.
pub const ERROR_CODE_PENDING_WITHDRAWAL_EXISTS: i32 = 54;
/// Error code indicating a pending deposit already exists.
pub const ERROR_CODE_PENDING_DEPOSIT_EXISTS: i32 = 55;
/// Error code indicating a pending outgoing address already exists.
pub const ERROR_CODE_PENDING_OUTGOING_ADDRESS_EXISTS: i32 = 56;
/// Error code indicating a pending address pool already exists.
pub const ERROR_CODE_PENDING_ADDRESS_POOL_EXISTS: i32 = 57;
/// Error code indicating the email address is invalid.
pub const ERROR_CODE_INVALID_EMAIL_ADDRESS: i32 = 58;
/// Error code indicating the email address already exists.
pub const ERROR_CODE_EMAIL_ADDRESS_ALREADY_EXISTS: i32 = 59;
/// Error code indicating the email address was not found.
pub const ERROR_CODE_EMAIL_ADDRESS_NOT_FOUND: i32 = 60;
/// Error code indicating the password is invalid.
pub const ERROR_CODE_INVALID_PASSWORD: i32 = 61;
/// Error code indicating the password is too short.
pub const ERROR_CODE_PASSWORD_TOO_SHORT: i32 = 62;
/// Error code indicating the password is too weak.
pub const ERROR_CODE_PASSWORD_TOO_WEAK: i32 = 63;
/// Error code indicating the username is invalid.
pub const ERROR_CODE_INVALID_USERNAME: i32 = 64;
/// Error code indicating the username already exists.
pub const ERROR_CODE_USERNAME_ALREADY_EXISTS: i32 = 65;
/// Error code indicating the username was not found.
pub const ERROR_CODE_USERNAME_NOT_FOUND: i32 = 66;
/// Error code indicating the session ID is invalid.
pub const ERROR_CODE_INVALID_SESSION_ID: i32 = 67;
/// Error code indicating the session ID has expired.
pub const ERROR_CODE_SESSION_ID_EXPIRED: i32 = 68;
/// Error code indicating the activation code is invalid.
pub const ERROR_CODE_INVALID_ACTIVATION_CODE: i32 = 69;
/// Error code indicating the activation code has expired.
pub const ERROR_CODE_ACTIVATION_CODE_EXPIRED: i32 = 70;
/// Error code indicating the reset password code is invalid.
pub const ERROR_CODE_INVALID_RESET_PASSWORD_CODE: i32 = 71;
/// Error code indicating the reset password code has expired.
pub const ERROR_CODE_RESET_PASSWORD_CODE_EXPIRED: i32 = 72;
/// Error code indicating the two-factor authentication code is invalid.
pub const ERROR_CODE_INVALID_TWO_FACTOR_AUTHENTICATION_CODE: i32 = 73;
/// Error code indicating the two-factor authentication code has expired.
pub const ERROR_CODE_TWO_FACTOR_AUTHENTICATION_CODE_EXPIRED: i32 = 74;
/// Error code indicating the two-factor authentication code is incorrect.
pub const ERROR_CODE_TWO_FACTOR_AUTHENTICATION_CODE_INCORRECT: i32 = 75;
/// Error code indicating two-factor authentication is already enabled.
pub const ERROR_CODE_TWO_FACTOR_AUTHENTICATION_ALREADY_ENABLED: i32 = 76;
/// Error code indicating two-factor authentication is not enabled.
pub const ERROR_CODE_TWO_FACTOR_AUTHENTICATION_NOT_ENABLED: i32 = 77;
/// Error code indicating the phone number is invalid.
pub const ERROR_CODE_INVALID_PHONE_NUMBER: i32 = 78;
/// Error code indicating the phone number already exists.
pub const ERROR_CODE_PHONE_NUMBER_ALREADY_EXISTS: i32 = 79;
/// Error code indicating the phone number was not found.
pub const ERROR_CODE_PHONE_NUMBER_NOT_FOUND: i32 = 80;
/// Error code indicating the bank account is invalid.
pub const ERROR_CODE_INVALID_BANK_ACCOUNT: i32 = 81;
/// Error code indicating the bank account already exists.
pub const ERROR_CODE_BANK_ACCOUNT_ALREADY_EXISTS: i32 = 82;
/// Error code indicating the bank account was not found.
pub const ERROR_CODE_BANK_ACCOUNT_NOT_FOUND: i32 = 83;
/// Error code indicating the IBAN is invalid.
pub const ERROR_CODE_INVALID_IBAN: i32 = 84;
/// Error code indicating the BIC is invalid.
pub const ERROR_CODE_INVALID_BIC: i32 = 85;
/// Error code indicating the account holder is invalid.
pub const ERROR_CODE_INVALID_ACCOUNT_HOLDER: i32 = 86;
/// Error code indicating the PIN is invalid.
pub const ERROR_CODE_INVALID_PIN: i32 = 87;
/// Error code indicating the PIN is incorrect.
pub const ERROR_CODE_PIN_INCORRECT: i32 = 88;
/// Error code indicating the PIN is already set.
pub const ERROR_CODE_PIN_ALREADY_SET: i32 = 89;
/// Error code indicating the PIN is not set.
pub const ERROR_CODE_PIN_NOT_SET: i32 = 90;
/// Error code indicating the KYC state is invalid.
pub const ERROR_CODE_INVALID_KYC_STATE: i32 = 91;
/// Error code indicating the KYC state was not found.
pub const ERROR_CODE_KYC_STATE_NOT_FOUND: i32 = 92;
/// Error code indicating the legal form is invalid.
pub const ERROR_CODE_INVALID_LEGAL_FORM: i32 = 93;
/// Error code indicating the legal form was not found.
pub const ERROR_CODE_LEGAL_FORM_NOT_FOUND: i32 = 94;
/// Error code indicating the gender is invalid.
pub const ERROR_CODE_INVALID_GENDER: i32 = 95;
/// Error code indicating the gender was not found.
pub const ERROR_CODE_GENDER_NOT_FOUND: i32 = 96;
/// Error code indicating the title is invalid.
pub const ERROR_CODE_INVALID_TITLE: i32 = 97;
/// Error code indicating the title was not found.
pub const ERROR_CODE_TITLE_NOT_FOUND: i32 = 98;
/// Error code indicating the nationality is invalid.
pub const ERROR_CODE_INVALID_NATIONALITY: i32 = 99;
/// Error code indicating the nationality was not found.
pub const ERROR_CODE_NATIONALITY_NOT_FOUND: i32 = 100;
/// Error code indicating the birth date is invalid.
pub const ERROR_CODE_INVALID_BIRTH_DATE: i32 = 101;
/// Error code indicating the birth city is invalid.
pub const ERROR_CODE_INVALID_BIRTH_CITY: i32 = 102;
/// Error code indicating the birth country is invalid.
pub const ERROR_CODE_INVALID_BIRTH_COUNTRY: i32 = 103;
/// Error code indicating the residence street is invalid.
pub const ERROR_CODE_INVALID_RESIDENCE_STREET: i32 = 104;
/// Error code indicating the residence ZIP code is invalid.
pub const ERROR_CODE_INVALID_RESIDENCE_ZIP_CODE: i32 = 105;
/// Error code indicating the residence city is invalid.
pub const ERROR_CODE_INVALID_RESIDENCE_CITY: i32 = 106;
/// Error code indicating the residence country is invalid.
pub const ERROR_CODE_INVALID_RESIDENCE_COUNTRY: i32 = 107;
/// Error code indicating the company name is invalid.
pub const ERROR_CODE_INVALID_COMPANY_NAME: i32 = 108;
/// Error code indicating the company registration number is invalid.
///
/// This error is returned when the provided company register number
/// does not meet the required format or validation rules.
pub const ERROR_CODE_INVALID_COMPANY_REGISTER_NUMBER: i32 = 109;
/// Error code indicating the company VAT ID is invalid.
///
/// This error is returned when the provided Value Added Tax (VAT) identification
/// number does not conform to the expected format or fails validation.
pub const ERROR_CODE_INVALID_COMPANY_VAT_ID: i32 = 110;
/// Error code indicating the company street address is invalid.
///
/// This error is returned when the provided company street address
/// is missing, malformed, or otherwise fails validation.
pub const ERROR_CODE_INVALID_COMPANY_STREET: i32 = 111;
/// Error code indicating the company ZIP/postal code is invalid.
///
/// This error is returned when the provided company ZIP code
/// does not match the expected format for the given country.
pub const ERROR_CODE_INVALID_COMPANY_ZIP_CODE: i32 = 112;
/// Error code indicating the company city is invalid.
///
/// This error is returned when the provided company city name
/// is missing, malformed, or otherwise fails validation.
pub const ERROR_CODE_INVALID_COMPANY_CITY: i32 = 113;
/// Error code indicating the company country is invalid.
///
/// This error is returned when the provided company country
/// does not exist or is not in the expected format (e.g., ISO country code).
pub const ERROR_CODE_INVALID_COMPANY_COUNTRY: i32 = 114;
/// Error code indicating the company foundation date is invalid.
///
/// This error is returned when the provided company foundation date
/// is in an incorrect format or represents an impossible date.
pub const ERROR_CODE_INVALID_COMPANY_FOUNDATION_DATE: i32 = 115;
/// Error code indicating the company industry sector is invalid.
///
/// This error is returned when the provided industry sector
/// is not recognized or does not match one of the allowed values.
pub const ERROR_CODE_INVALID_COMPANY_INDUSTRY_SECTOR: i32 = 116;
/// Error code indicating the company website is invalid.
///
/// This error is returned when the provided company website URL
/// is malformed or does not follow proper URL format standards.
pub const ERROR_CODE_INVALID_COMPANY_WEBSITE: i32 = 117;
/// Error code indicating the company phone number is invalid.
///
/// This error is returned when the provided company phone number
/// does not match the expected format or contains invalid characters.
pub const ERROR_CODE_INVALID_COMPANY_PHONE_NUMBER: i32 = 118;
/// Error code indicating the tax number is invalid.
///
/// This error is returned when the provided tax identification number
/// does not conform to the expected format or fails validation checks.
pub const ERROR_CODE_INVALID_TAX_NUMBER: i32 = 119;
/// Error code indicating the array format provided is invalid.
///
/// This error is returned when an array parameter does not conform to the expected structure,
/// contains invalid elements, or is otherwise malformed.
pub const ERROR_CODE_INVALID_ARRAY_FORMAT: i32 = 120;

/// Error code indicating the trust level rating provided is invalid.
///
/// This error is returned when a trust level rating value is outside the acceptable range
/// or does not match the expected format for user ratings.
pub const ERROR_CODE_INVALID_TRUST_LEVEL_RATING: i32 = 121;

/// Error code indicating the language code provided is invalid.
///
/// This error is returned when the language identifier does not match a supported
/// language code (typically ISO 639-1 format) recognized by the API.
pub const ERROR_CODE_INVALID_LANGUAGE: i32 = 122;

/// Error code indicating the locale format provided is invalid.
///
/// This error is returned when the locale string (typically language and country code)
/// does not conform to expected standards like ISO 639-1 combined with ISO 3166-1.
pub const ERROR_CODE_INVALID_LOCALE: i32 = 123;

/// Error code indicating the date format provided is invalid.
///
/// This error is returned when a date string does not match the expected format
/// (typically YYYY-MM-DD) or represents an impossible date.
pub const ERROR_CODE_INVALID_DATE_FORMAT: i32 = 124;

/// Error code indicating the time format provided is invalid.
///
/// This error is returned when a time string does not match the expected format
/// (typically HH:MM:SS) or represents an impossible time.
pub const ERROR_CODE_INVALID_TIME_FORMAT: i32 = 125;

/// Error code indicating the datetime format provided is invalid.
///
/// This error is returned when a datetime string does not match the expected format
/// (typically YYYY-MM-DD HH:MM:SS) or represents an impossible datetime.
pub const ERROR_CODE_INVALID_DATETIME_FORMAT: i32 = 126;

/// Error code indicating the timestamp format provided is invalid.
///
/// This error is returned when a Unix timestamp is malformed, out of range,
/// or otherwise does not represent a valid point in time.
pub const ERROR_CODE_INVALID_TIMESTAMP_FORMAT: i32 = 127;

/// Error code indicating the boolean format provided is invalid.
///
/// This error is returned when a value that should be a boolean (true/false)
/// is in an incorrect format or cannot be interpreted as a boolean value.
pub const ERROR_CODE_INVALID_BOOLEAN_FORMAT: i32 = 128;

/// Error code indicating the integer format provided is invalid.
///
/// This error is returned when a value that should be an integer contains non-numeric
/// characters, is out of range, or otherwise cannot be parsed as an integer.
pub const ERROR_CODE_INVALID_INTEGER_FORMAT: i32 = 129;

/// Error code indicating the float format provided is invalid.
///
/// This error is returned when a value that should be a floating-point number
/// contains invalid characters or cannot be parsed as a valid decimal number.
pub const ERROR_CODE_INVALID_FLOAT_FORMAT: i32 = 130;

/// Error code indicating the enum value provided is invalid.
///
/// This error is returned when a value does not match any of the allowed options
/// in an enumerated list of valid values for a particular field.
pub const ERROR_CODE_INVALID_ENUM_VALUE: i32 = 131;

/// Error code indicating the JSON format provided is invalid.
///
/// This error is returned when a JSON string is malformed, contains syntax errors,
/// or otherwise cannot be parsed as valid JSON.
pub const ERROR_CODE_INVALID_JSON_FORMAT: i32 = 132;

/// Error code indicating the XML format provided is invalid.
///
/// This error is returned when an XML string is malformed, contains syntax errors,
/// or otherwise cannot be parsed as valid XML.
pub const ERROR_CODE_INVALID_XML_FORMAT: i32 = 133;

/// Error code indicating the CSV format provided is invalid.
///
/// This error is returned when a CSV string is malformed, has inconsistent delimiters,
/// or otherwise cannot be parsed as valid CSV data.
pub const ERROR_CODE_INVALID_CSV_FORMAT: i32 = 134;

/// Error code indicating the YAML format provided is invalid.
///
/// This error is returned when a YAML string is malformed, contains syntax errors,
/// or otherwise cannot be parsed as valid YAML.
pub const ERROR_CODE_INVALID_YAML_FORMAT: i32 = 135;

/// Error code indicating the MIME type provided is invalid.
///
/// This error is returned when a MIME type string does not conform to the standard
/// format (type/subtype) or specifies an unrecognized media type.
pub const ERROR_CODE_INVALID_MIME_TYPE: i32 = 136;

/// Error code indicating the file extension provided is invalid.
///
/// This error is returned when a file extension is not recognized, contains invalid
/// characters, or is not supported for the operation being performed.
pub const ERROR_CODE_INVALID_FILE_EXTENSION: i32 = 137;

/// Error code indicating the file size provided is invalid.
///
/// This error is returned when a file's size is outside the acceptable range
/// (typically too large or zero) for the operation being performed.
pub const ERROR_CODE_INVALID_FILE_SIZE: i32 = 138;

/// Error code indicating the image format provided is invalid.
///
/// This error is returned when an image file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_IMAGE_FORMAT: i32 = 139;

/// Error code indicating the image resolution provided is invalid.
///
/// This error is returned when an image's dimensions (width × height) are outside
/// the acceptable range for the operation being performed.
pub const ERROR_CODE_INVALID_IMAGE_RESOLUTION: i32 = 140;

/// Error code indicating the video format provided is invalid.
///
/// This error is returned when a video file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_VIDEO_FORMAT: i32 = 141;

/// Error code indicating the video resolution provided is invalid.
///
/// This error is returned when a video's dimensions (width × height) are outside
/// the acceptable range for the operation being performed.
pub const ERROR_CODE_INVALID_VIDEO_RESOLUTION: i32 = 142;

/// Error code indicating the audio format provided is invalid.
///
/// This error is returned when an audio file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_AUDIO_FORMAT: i32 = 143;

/// Error code indicating the audio bitrate provided is invalid.
///
/// This error is returned when an audio file's bitrate is outside the acceptable
/// range for the operation being performed.
pub const ERROR_CODE_INVALID_AUDIO_BITRATE: i32 = 144;

/// Error code indicating the document format provided is invalid.
///
/// This error is returned when a document file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_DOCUMENT_FORMAT: i32 = 145;

/// Error code indicating the document size provided is invalid.
///
/// This error is returned when a document's file size is outside the acceptable
/// range for the operation being performed.
pub const ERROR_CODE_INVALID_DOCUMENT_SIZE: i32 = 146;

/// Error code indicating the signature format provided is invalid.
///
/// This error is returned when a digital signature is malformed, uses an unsupported
/// algorithm, or otherwise fails validation.
pub const ERROR_CODE_INVALID_SIGNATURE_FORMAT: i32 = 147;

/// Error code indicating the encryption algorithm provided is invalid.
///
/// This error is returned when an encryption algorithm is not supported
/// or is inappropriate for the operation being performed.
pub const ERROR_CODE_INVALID_ENCRYPTION_ALGORITHM: i32 = 148;

/// Error code indicating the compression algorithm provided is invalid.
///
/// This error is returned when a compression algorithm is not supported
/// or is inappropriate for the operation being performed.
pub const ERROR_CODE_INVALID_COMPRESSION_ALGORITHM: i32 = 149;

/// Error code indicating the hash algorithm provided is invalid.
///
/// This error is returned when a hash algorithm is not supported
/// or is inappropriate for the operation being performed.
pub const ERROR_CODE_INVALID_HASH_ALGORITHM: i32 = 150;

/// Error code indicating the IP address provided is invalid.
///
/// This error is returned when an IP address string is malformed or does not
/// represent a valid IPv4 or IPv6 address.
pub const ERROR_CODE_INVALID_IP_ADDRESS: i32 = 151;

/// Error code indicating the domain name provided is invalid.
///
/// This error is returned when a domain name string is malformed, contains invalid
/// characters, or does not conform to DNS naming standards.
pub const ERROR_CODE_INVALID_DOMAIN_NAME: i32 = 152;

/// Error code indicating the URL format provided is invalid.
///
/// This error is returned when a URL string is malformed, missing required components,
/// or otherwise does not conform to URL standards.
pub const ERROR_CODE_INVALID_URL_FORMAT: i32 = 153;

/// Error code indicating the email format is generically invalid.
///
/// This error is returned when an email address string does not conform to the
/// standard format (local-part@domain) or contains invalid characters.
pub const ERROR_CODE_INVALID_EMAIL_FORMAT_GENERIC: i32 = 154;

/// Error code indicating the username format is generically invalid.
///
/// This error is returned when a username contains invalid characters, is too short,
/// or otherwise does not meet the system's username requirements.
pub const ERROR_CODE_INVALID_USERNAME_FORMAT_GENERIC: i32 = 155;

/// Error code indicating the password format is generically invalid.
///
/// This error is returned when a password does not meet the system's complexity
/// requirements or contains invalid characters.
pub const ERROR_CODE_INVALID_PASSWORD_FORMAT_GENERIC: i32 = 156;

/// Error code indicating the phone number format is generically invalid.
///
/// This error is returned when a phone number string contains invalid characters
/// or does not conform to expected international phone number formats.
pub const ERROR_CODE_INVALID_PHONE_NUMBER_FORMAT_GENERIC: i32 = 157;

/// Error code indicating the bank account format is generically invalid.
///
/// This error is returned when bank account details do not conform to the expected
/// format or contain invalid characters.
pub const ERROR_CODE_INVALID_BANK_ACCOUNT_FORMAT_GENERIC: i32 = 158;

/// Error code indicating the IBAN format is generically invalid.
///
/// This error is returned when an International Bank Account Number (IBAN) string
/// does not conform to the ISO 13616 standard or fails checksum validation.
pub const ERROR_CODE_INVALID_IBAN_FORMAT_GENERIC: i32 = 159;

/// Error code indicating the BIC format is generically invalid.
///
/// This error is returned when a Bank Identifier Code (BIC) string does not conform
/// to the ISO 9362 standard or contains invalid characters.
pub const ERROR_CODE_INVALID_BIC_FORMAT_GENERIC: i32 = 160;

/// Error code indicating the account holder format is generically invalid.
///
/// This error is returned when an account holder name contains invalid characters
/// or does not meet the system's requirements.
pub const ERROR_CODE_INVALID_ACCOUNT_HOLDER_FORMAT_GENERIC: i32 = 161;

/// Error code indicating the PIN format is generically invalid.
///
/// This error is returned when a Personal Identification Number (PIN) does not meet
/// length requirements or contains invalid characters.
pub const ERROR_CODE_INVALID_PIN_FORMAT_GENERIC: i32 = 162;

/// Error code indicating the KYC state format is generically invalid.
///
/// This error is returned when a Know Your Customer (KYC) state value does not match
/// one of the system's recognized verification states.
pub const ERROR_CODE_INVALID_KYC_STATE_FORMAT_GENERIC: i32 = 163;

/// Error code indicating the legal form format is generically invalid.
///
/// This error is returned when a legal form identifier does not match one of the
/// system's recognized business entity types.
pub const ERROR_CODE_INVALID_LEGAL_FORM_FORMAT_GENERIC: i32 = 164;

/// Error code indicating the gender format is generically invalid.
///
/// This error is returned when a gender value does not match one of the
/// system's recognized gender options.
pub const ERROR_CODE_INVALID_GENDER_FORMAT_GENERIC: i32 = 165;

/// Error code indicating the title format is generically invalid.
///
/// This error is returned when a personal title (Mr., Mrs., Dr., etc.) does not match
/// one of the system's recognized title options.
pub const ERROR_CODE_INVALID_TITLE_FORMAT_GENERIC: i32 = 166;

/// Error code indicating the nationality format is generically invalid.
///
/// This error is returned when a nationality identifier does not match a recognized
/// country code or contains invalid characters.
pub const ERROR_CODE_INVALID_NATIONALITY_FORMAT_GENERIC: i32 = 167;

/// Error code indicating the birth date format is generically invalid.
///
/// This error is returned when a birth date string is malformed, represents an impossible
/// date, or does not meet age requirements.
pub const ERROR_CODE_INVALID_BIRTH_DATE_FORMAT_GENERIC: i32 = 168;

/// Error code indicating the birth city format is generically invalid.
///
/// This error is returned when a birth city name contains invalid characters
/// or does not meet the system's requirements.
pub const ERROR_CODE_INVALID_BIRTH_CITY_FORMAT_GENERIC: i32 = 169;

/// Error code indicating the birth country format is generically invalid.
///
/// This error is returned when a birth country identifier does not match a recognized
/// country code or contains invalid characters.
pub const ERROR_CODE_INVALID_BIRTH_COUNTRY_FORMAT_GENERIC: i32 = 170;

/// Error code indicating the residence format is generically invalid.
///
/// This error is returned when residence address information is incomplete, contains
/// invalid characters, or otherwise fails validation.
pub const ERROR_CODE_INVALID_RESIDENCE_FORMAT_GENERIC: i32 = 171;

/// Error code indicating the company format is generically invalid.
///
/// This error is returned when company information is incomplete, contains
/// invalid characters, or otherwise fails validation.
pub const ERROR_CODE_INVALID_COMPANY_FORMAT_GENERIC: i32 = 172;

/// Error code indicating the company industry sector format is generically invalid.
///
/// This error is returned when an industry sector identifier does not match one of the
/// system's recognized industry categories.
pub const ERROR_CODE_INVALID_COMPANY_INDUSTRY_SECTOR_FORMAT_GENERIC: i32 = 173;

/// Error code indicating the company website format is generically invalid.
///
/// This error is returned when a company website URL is malformed, contains invalid
/// characters, or does not conform to URL standards.
pub const ERROR_CODE_INVALID_COMPANY_WEBSITE_FORMAT_GENERIC: i32 = 174;

/// Error code indicating the company phone number format is generically invalid.
///
/// This error is returned when a company phone number contains invalid characters
/// or does not conform to expected international phone number formats.
pub const ERROR_CODE_INVALID_COMPANY_PHONE_NUMBER_FORMAT_GENERIC: i32 = 175;

/// Error code indicating the tax number format is generically invalid.
///
/// This error is returned when a tax identification number contains invalid characters
/// or does not conform to the expected format for the relevant tax authority.
pub const ERROR_CODE_INVALID_TAX_NUMBER_FORMAT_GENERIC: i32 = 176;

/// Error code indicating the language format is generically invalid.
///
/// This error is returned when a language identifier does not match a supported
/// language code or contains invalid characters.
pub const ERROR_CODE_INVALID_LANGUAGE_FORMAT_GENERIC: i32 = 177;
/// Error code indicating the locale format is generically invalid.
///
/// This error is returned when a locale string (typically language and country code)
/// does not conform to expected standards like ISO 639-1 combined with ISO 3166-1.
pub const ERROR_CODE_INVALID_LOCALE_FORMAT_GENERIC: i32 = 178;

/// Error code indicating the date format is generically invalid.
///
/// This error is returned when a date string does not match the expected format
/// (typically YYYY-MM-DD) or represents an impossible date.
pub const ERROR_CODE_INVALID_DATE_FORMAT_GENERIC: i32 = 179;

/// Error code indicating the time format is generically invalid.
///
/// This error is returned when a time string does not match the expected format
/// (typically HH:MM:SS) or represents an impossible time.
pub const ERROR_CODE_INVALID_TIME_FORMAT_GENERIC: i32 = 180;

/// Error code indicating the datetime format is generically invalid.
///
/// This error is returned when a datetime string does not match the expected format
/// (typically YYYY-MM-DD HH:MM:SS) or represents an impossible datetime.
pub const ERROR_CODE_INVALID_DATETIME_FORMAT_GENERIC: i32 = 181;

/// Error code indicating the timestamp format is generically invalid.
///
/// This error is returned when a Unix timestamp is malformed, out of range,
/// or otherwise does not represent a valid point in time.
pub const ERROR_CODE_INVALID_TIMESTAMP_FORMAT_GENERIC: i32 = 182;

/// Error code indicating the boolean format is generically invalid.
///
/// This error is returned when a value that should be a boolean (true/false)
/// is in an incorrect format or cannot be interpreted as a boolean value.
pub const ERROR_CODE_INVALID_BOOLEAN_FORMAT_GENERIC: i32 = 183;

/// Error code indicating the integer format is generically invalid.
///
/// This error is returned when a value that should be an integer contains non-numeric
/// characters, is out of range, or otherwise cannot be parsed as an integer.
pub const ERROR_CODE_INVALID_INTEGER_FORMAT_GENERIC: i32 = 184;

/// Error code indicating the float format is generically invalid.
///
/// This error is returned when a value that should be a floating-point number
/// contains invalid characters or cannot be parsed as a valid decimal number.
pub const ERROR_CODE_INVALID_FLOAT_FORMAT_GENERIC: i32 = 185;

/// Error code indicating the enum value format is generically invalid.
///
/// This error is returned when a value does not match any of the allowed options
/// in an enumerated list of valid values for a particular field.
pub const ERROR_CODE_INVALID_ENUM_VALUE_FORMAT_GENERIC: i32 = 186;

/// Error code indicating the JSON format is generically invalid.
///
/// This error is returned when a JSON string is malformed, contains syntax errors,
/// or otherwise cannot be parsed as valid JSON.
pub const ERROR_CODE_INVALID_JSON_FORMAT_GENERIC: i32 = 187;

/// Error code indicating the XML format is generically invalid.
///
/// This error is returned when an XML string is malformed, contains syntax errors,
/// or otherwise cannot be parsed as valid XML.
pub const ERROR_CODE_INVALID_XML_FORMAT_GENERIC: i32 = 188;

/// Error code indicating the CSV format is generically invalid.
///
/// This error is returned when a CSV string is malformed, has inconsistent delimiters,
/// or otherwise cannot be parsed as valid CSV data.
pub const ERROR_CODE_INVALID_CSV_FORMAT_GENERIC: i32 = 189;

/// Error code indicating the YAML format is generically invalid.
///
/// This error is returned when a YAML string is malformed, contains syntax errors,
/// or otherwise cannot be parsed as valid YAML.
pub const ERROR_CODE_INVALID_YAML_FORMAT_GENERIC: i32 = 190;

/// Error code indicating the MIME type format is generically invalid.
///
/// This error is returned when a MIME type string does not conform to the standard
/// format (type/subtype) or specifies an unrecognized media type.
pub const ERROR_CODE_INVALID_MIME_TYPE_FORMAT_GENERIC: i32 = 191;

/// Error code indicating the file extension format is generically invalid.
///
/// This error is returned when a file extension is not recognized, contains invalid
/// characters, or is not supported for the operation being performed.
pub const ERROR_CODE_INVALID_FILE_EXTENSION_FORMAT_GENERIC: i32 = 192;

/// Error code indicating the file size format is generically invalid.
///
/// This error is returned when a file's size is outside the acceptable range
/// (typically too large or zero) for the operation being performed.
pub const ERROR_CODE_INVALID_FILE_SIZE_FORMAT_GENERIC: i32 = 193;

/// Error code indicating the image format is generically invalid.
///
/// This error is returned when an image file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_IMAGE_FORMAT_FORMAT_GENERIC: i32 = 194;

/// Error code indicating the image resolution format is generically invalid.
///
/// This error is returned when an image's dimensions (width × height) are outside
/// the acceptable range for the operation being performed.
pub const ERROR_CODE_INVALID_IMAGE_RESOLUTION_FORMAT_GENERIC: i32 = 195;

/// Error code indicating the video format is generically invalid.
///
/// This error is returned when a video file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_VIDEO_FORMAT_FORMAT_GENERIC: i32 = 196;

/// Error code indicating the video resolution format is generically invalid.
///
/// This error is returned when a video's dimensions (width × height) are outside
/// the acceptable range for the operation being performed.
pub const ERROR_CODE_INVALID_VIDEO_RESOLUTION_FORMAT_GENERIC: i32 = 197;

/// Error code indicating the audio format is generically invalid.
///
/// This error is returned when an audio file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_AUDIO_FORMAT_FORMAT_GENERIC: i32 = 198;

/// Error code indicating the audio bitrate format is generically invalid.
///
/// This error is returned when an audio file's bitrate is outside the acceptable
/// range for the operation being performed.
pub const ERROR_CODE_INVALID_AUDIO_BITRATE_FORMAT_GENERIC: i32 = 199;

/// Error code indicating the document format is generically invalid.
///
/// This error is returned when a document file is in an unsupported format
/// or the format specified does not match the actual file content.
pub const ERROR_CODE_INVALID_DOCUMENT_FORMAT_FORMAT_GENERIC: i32 = 200;

/// Error code indicating the document size format is generically invalid.
///
/// This error is returned when a document's file size is outside the acceptable
/// range for the operation being performed.
pub const ERROR_CODE_INVALID_DOCUMENT_SIZE_FORMAT_GENERIC: i32 = 201;

/// Error code indicating the signature format is generically invalid.
///
/// This error is returned when a digital signature is malformed, uses an unsupported
/// algorithm, or otherwise fails validation.
pub const ERROR_CODE_INVALID_SIGNATURE_FORMAT_FORMAT_GENERIC: i32 = 202;

/// Error code indicating the encryption algorithm format is generically invalid.
///
/// This error is returned when an encryption algorithm is not supported
/// or is inappropriate for the operation being performed.
pub const ERROR_CODE_INVALID_ENCRYPTION_ALGORITHM_FORMAT_GENERIC: i32 = 203;

/// Error code indicating the compression algorithm format is generically invalid.
///
/// This error is returned when a compression algorithm is not supported
/// or is inappropriate for the operation being performed.
pub const ERROR_CODE_INVALID_COMPRESSION_ALGORITHM_FORMAT_GENERIC: i32 = 204;

/// Error code indicating the hash algorithm format is generically invalid.
///
/// This error is returned when a hash algorithm is not supported
/// or is inappropriate for the operation being performed.
pub const ERROR_CODE_INVALID_HASH_ALGORITHM_FORMAT_GENERIC: i32 = 205;

/// Error code indicating the IP address format is generically invalid.
///
/// This error is returned when an IP address string is malformed or does not
/// represent a valid IPv4 or IPv6 address.
pub const ERROR_CODE_INVALID_IP_ADDRESS_FORMAT_GENERIC: i32 = 206;

/// Error code indicating the domain name format is generically invalid.
///
/// This error is returned when a domain name string is malformed, contains invalid
/// characters, or does not conform to DNS naming standards.
pub const ERROR_CODE_INVALID_DOMAIN_NAME_FORMAT_GENERIC: i32 = 207;

/// Error code indicating the URL format is generically invalid.
///
/// This error is returned when a URL string is malformed, missing required components,
/// or otherwise does not conform to URL standards.
pub const ERROR_CODE_INVALID_URL_FORMAT_GENERIC_FORMAT_GENERIC: i32 = 208;

/// Error code indicating an unknown error has occurred.
///
/// This is a general error code returned when the system encounters an error
/// that doesn't match any of the more specific error codes.
pub const ERROR_CODE_UNKNOWN_ERROR: i32 = 9998;

/// Error code indicating the system is currently under maintenance.
///
/// This error is returned when the API or parts of it are temporarily unavailable
/// due to scheduled maintenance or unplanned downtime.
pub const ERROR_CODE_MAINTENANCE: i32 = 9999;

// API Method Names
pub const METHOD_SHOW_ORDERBOOK: &str = "showOrderbook";
pub const METHOD_SHOW_ORDER_DETAILS: &str = "showOrderDetails";
pub const METHOD_CREATE_ORDER: &str = "createOrder";
pub const METHOD_DELETE_ORDER: &str = "deleteOrder";
pub const METHOD_SHOW_MY_ORDERS: &str = "showMyOrders";
pub const METHOD_SHOW_MY_ORDER_DETAILS: &str = "showMyOrderDetails";
pub const METHOD_EXECUTE_TRADE: &str = "executeTrade";
pub const METHOD_SHOW_MY_TRADES: &str = "showMyTrades";
pub const METHOD_SHOW_MY_TRADE_DETAILS: &str = "showMyTradeDetails";
pub const METHOD_MARK_TRADE_AS_PAID: &str = "markTradeAsPaid";
pub const METHOD_MARK_TRADE_AS_PAYMENT_RECEIVED: &str = "markTradeAsPaymentReceived";
pub const METHOD_ADD_TRADE_RATING: &str = "addTradeRating";
pub const METHOD_SHOW_ACCOUNT_INFO: &str = "showAccountInfo";
pub const METHOD_CREATE_WITHDRAWAL: &str = "createWithdrawal";
pub const METHOD_DELETE_WITHDRAWAL: &str = "deleteWithdrawal";
pub const METHOD_SHOW_WITHDRAWAL: &str = "showWithdrawal";
pub const METHOD_SHOW_WITHDRAWALS: &str = "showWithdrawals";
pub const METHOD_SHOW_WITHDRAWAL_MIN_NETWORK_FEE: &str = "showWithdrawalMinNetworkFee";
pub const METHOD_REQUEST_DEPOSIT_ADDRESS: &str = "requestDepositAddress";
pub const METHOD_SHOW_DEPOSIT: &str = "showDeposit";
pub const METHOD_SHOW_DEPOSITS: &str = "showDeposits";
pub const METHOD_CREATE_OUTGOING_ADDRESS: &str = "createOutgoingAddress";
pub const METHOD_DELETE_OUTGOING_ADDRESS: &str = "deleteOutgoingAddress";
pub const METHOD_SHOW_OUTGOING_ADDRESSES: &str = "showOutgoingAddresses";
pub const METHOD_SHOW_PUBLIC_TRADE_HISTORY: &str = "showPublicTradeHistory";
pub const METHOD_SHOW_ORDERBOOK_COMPACT: &str = "showOrderbookCompact";
pub const METHOD_SHOW_RATES: &str = "showRates";
pub const METHOD_SHOW_ACCOUNT_LEDGER: &str = "showAccountLedger";
pub const METHOD_ADD_TO_ADDRESS_POOL: &str = "addToAddressPool";
pub const METHOD_REMOVE_FROM_ADDRESS_POOL: &str = "removeFromAddressPool";
pub const METHOD_LIST_ADDRESS_POOL: &str = "listAddressPool";
pub const METHOD_MARK_COINS_AS_TRANSFERRED: &str = "markCoinsAsTransferred";
pub const METHOD_MARK_COINS_AS_RECEIVED: &str = "markCoinsAsReceived";
pub const METHOD_SHOW_PERMISSIONS: &str = "showPermissions";

// Headers
pub const HEADER_X_NONCE: &str = "X-API-NONCE";
pub const HEADER_X_API_KEY: &str = "X-API-KEY";
pub const HEADER_X_API_SIGNATURE: &str = "X-API-SIGNATURE";

// Parameter names (need to add all parameter constants from PHP file)
pub const SHOW_ORDERBOOK_PARAMETER_TYPE: &str = "type";
pub const SHOW_ORDERBOOK_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const SHOW_ORDER_DETAILS_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const SHOW_ORDER_DETAILS_PARAMETER_ORDER_ID: &str = "order_id";
pub const CREATE_ORDER_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const CREATE_ORDER_PARAMETER_TYPE: &str = "type";
pub const CREATE_ORDER_PARAMETER_AMOUNT: &str = "amount";
pub const CREATE_ORDER_PARAMETER_PRICE: &str = "price";
pub const CREATE_ORDER_PARAMETER_ORDER_PAYMENT_OPTIONS: &str = "order_payment_options";
pub const DELETE_ORDER_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const DELETE_ORDER_PARAMETER_ORDER_ID: &str = "order_id";
pub const SHOW_MY_ORDER_DETAILS_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const EXECUTE_TRADE_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const EXECUTE_TRADE_PARAMETER_ORDER_ID: &str = "order_id";
pub const EXECUTE_TRADE_PARAMETER_TYPE: &str = "type";
pub const EXECUTE_TRADE_PARAMETER_AMOUNT_CURRENCY_TO_TRADE: &str = "amount_currency_to_trade";
pub const SHOW_MY_TRADE_DETAILS_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const SHOW_MY_TRADE_DETAILS_PARAMETER_TRADE_ID: &str = "trade_id";
pub const MARK_TRADE_AS_PAID_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const MARK_TRADE_AS_PAID_PARAMETER_TRADE_ID: &str = "trade_id";
pub const MARK_TRADE_AS_PAID_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE: &str = "volume_currency_to_pay_after_fee";
pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_TRADE_ID: &str = "trade_id";
pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_VOLUME_CURRENCY_TO_PAY_AFTER_FEE: &str = "volume_currency_to_pay_after_fee";
pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_RATING: &str = "rating";
pub const MARK_TRADE_AS_PAYMENT_RECEIVED_PARAMETER_IS_PAID_FROM_CORRECT_BANK_ACCOUNT: &str = "is_paid_from_correct_bank_account";
pub const ADD_TRADE_RATING_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const ADD_TRADE_RATING_PARAMETER_TRADE_ID: &str = "trade_id";
pub const ADD_TRADE_RATING_PARAMETER_RATING: &str = "rating";
pub const CREATE_WITHDRAWAL_PARAMETER_CURRENCY: &str = "currency";
pub const CREATE_WITHDRAWAL_PARAMETER_ADDRESS: &str = "address";
pub const CREATE_WITHDRAWAL_PARAMETER_AMOUNT: &str = "amount";
pub const CREATE_WITHDRAWAL_PARAMETER_NETWORK_FEE: &str = "network_fee";
pub const DELETE_WITHDRAWAL_PARAMETER_CURRENCY: &str = "currency";
pub const DELETE_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID: &str = "withdrawal_id";
pub const SHOW_WITHDRAWAL_PARAMETER_CURRENCY: &str = "currency";
pub const SHOW_WITHDRAWAL_PARAMETER_WITHDRAWAL_ID: &str = "withdrawal_id";
pub const SHOW_WITHDRAWALS_PARAMETER_CURRENCY: &str = "currency";
pub const SHOW_WITHDRAWAL_PARAMETER_MIN_NETWORK_FEE_CURRENCY: &str = "currency";
pub const REQUEST_DEPOSIT_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
pub const SHOW_DEPOSIT_PARAMETER_CURRENCY: &str = "currency";
pub const SHOW_DEPOSIT_PARAMETER_DEPOSIT_ID: &str = "deposit_id";
pub const SHOW_DEPOSITS_PARAMETER_CURRENCY: &str = "currency";
pub const CREATE_OUTGOING_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
pub const CREATE_OUTGOING_ADDRESS_PARAMETER_RECIPIENT_ADDRESS: &str = "recipient_address";
pub const CREATE_OUTGOING_ADDRESS_PARAMETER_RECIPIENT_PURPOSE: &str = "recipient_purpose";
pub const CREATE_OUTGOING_ADDRESS_PARAMETER_COMMENT: &str = "comment";
pub const DELETE_OUTGOING_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
pub const DELETE_OUTGOING_ADDRESS_PARAMETER_ADDRESS_ID: &str = "address_id";
pub const SHOW_OUTGOING_ADDRESS_PARAMETER_CURRENCY: &str = "currency";
pub const SHOW_PUBLIC_TRADE_HISTORY_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const SHOW_ORDER_BOOK_COMPACT_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const SHOW_RATES_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const SHOW_ACCOUNT_LEDGER_PARAMETER_CURRENCY: &str = "currency";
pub const ADD_TO_ADDRESS_POOL_PARAMETER_CURRENCY: &str = "currency";
pub const ADD_TO_ADDRESS_POOL_PARAMETER_ADDRESS: &str = "address";
pub const REMOVE_FROM_ADDRESS_POOL_PARAMETER_CURRENCY: &str = "currency";
pub const REMOVE_FROM_ADDRESS_POOL_PARAMETER_ADDRESS: &str = "address";
pub const LIST_ADDRESS_POOL_PARAMETER_CURRENCY: &str = "currency";
pub const MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const MARK_COINS_AS_TRANSFERRED_PARAMETER_TRADE_ID: &str = "trade_id";
pub const MARK_COINS_AS_TRANSFERRED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE: &str = "amount_currency_to_trade_after_fee";
pub const MARK_COINS_AS_RECEIVED_PARAMETER_TRADING_PAIR: &str = "trading_pair";
pub const MARK_COINS_AS_RECEIVED_PARAMETER_TRADE_ID: &str = "trade_id";
pub const MARK_COINS_AS_RECEIVED_PARAMETER_AMOUNT_CURRENCY_TO_TRADE_AFTER_FEE: &str = "amount_currency_to_trade_after_fee";
pub const MARK_COINS_AS_RECEIVED_PARAMETER_RATING: &str = "rating";

/// Configuration constants for the Bitcoin.de API client.
///
/// These constants define the default configuration options used when making
/// requests to the Bitcoin.de trading API.

/// Base URI for the Bitcoin.de API v4 endpoints.
/// All API requests will be sent to this base URL.
pub const API_BASE_URI: &str = "https://api.bitcoin.de/v4/";

/// Flag to determine whether SSL peer verification should be performed.
/// When set to true, the client will verify the SSL certificate of the API server.
pub const VERIFY_SSL_PEER: bool = true;

/// The API version string used in requests.
/// This constant references the API_VERSION constant to ensure consistency.
pub const API_VERSION_VALUE: &str = API_VERSION; // Use the API_VERSION constant
