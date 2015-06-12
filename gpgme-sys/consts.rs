#![allow(non_upper_case_globals)]
extern crate libc;
extern crate libgpg_error_sys;

pub use libgpg_error_sys::gpg_err_source_t as gpgme_err_source_t;
pub use libgpg_error_sys::gpg_err_code_t as gpgme_err_code_t;

pub use libgpg_error_sys::consts::*;

pub type gpgme_data_encoding_t = libc::c_uint;
pub const GPGME_DATA_ENCODING_NONE: gpgme_data_encoding_t = 0;
pub const GPGME_DATA_ENCODING_BINARY: gpgme_data_encoding_t = 1;
pub const GPGME_DATA_ENCODING_BASE64: gpgme_data_encoding_t = 2;
pub const GPGME_DATA_ENCODING_ARMOR: gpgme_data_encoding_t = 3;
pub const GPGME_DATA_ENCODING_URL: gpgme_data_encoding_t = 4;
pub const GPGME_DATA_ENCODING_URLESC: gpgme_data_encoding_t = 5;
pub const GPGME_DATA_ENCODING_URL0: gpgme_data_encoding_t = 6;

pub type gpgme_data_type_t = libc::c_uint;
pub const GPGME_DATA_TYPE_INVALID: gpgme_data_type_t = 0;
pub const GPGME_DATA_TYPE_UNKNOWN: gpgme_data_type_t = 1;
pub const GPGME_DATA_TYPE_PGP_SIGNED: gpgme_data_type_t = 0x10;
pub const GPGME_DATA_TYPE_PGP_OTHER: gpgme_data_type_t = 0x12;
pub const GPGME_DATA_TYPE_PGP_KEY: gpgme_data_type_t = 0x13;
pub const GPGME_DATA_TYPE_CMS_SIGNED: gpgme_data_type_t = 0x20;
pub const GPGME_DATA_TYPE_CMS_ENCRYPTED: gpgme_data_type_t = 0x21;
pub const GPGME_DATA_TYPE_CMS_OTHER: gpgme_data_type_t = 0x22;
pub const GPGME_DATA_TYPE_X509_CERT: gpgme_data_type_t = 0x23;
pub const GPGME_DATA_TYPE_PKCS12: gpgme_data_type_t = 0x24;

pub type gpgme_pubkey_algo_t = libc::c_uint;
pub const GPGME_PK_RSA: gpgme_pubkey_algo_t = 1;
pub const GPGME_PK_RSA_E: gpgme_pubkey_algo_t = 2;
pub const GPGME_PK_RSA_S: gpgme_pubkey_algo_t = 3;
pub const GPGME_PK_ELG_E: gpgme_pubkey_algo_t = 16;
pub const GPGME_PK_DSA: gpgme_pubkey_algo_t = 17;
pub const GPGME_PK_ELG: gpgme_pubkey_algo_t = 20;
pub const GPGME_PK_ECDSA: gpgme_pubkey_algo_t = 301;
pub const GPGME_PK_ECDH: gpgme_pubkey_algo_t = 302;

pub type gpgme_hash_algo_t = libc::c_uint;
pub const GPGME_MD_NONE: gpgme_hash_algo_t = 0;
pub const GPGME_MD_MD5: gpgme_hash_algo_t = 1;
pub const GPGME_MD_SHA1: gpgme_hash_algo_t = 2;
pub const GPGME_MD_RMD160: gpgme_hash_algo_t = 3;
pub const GPGME_MD_MD2: gpgme_hash_algo_t = 5;
pub const GPGME_MD_TIGER: gpgme_hash_algo_t = 6;
pub const GPGME_MD_HAVAL: gpgme_hash_algo_t = 7;
pub const GPGME_MD_SHA256: gpgme_hash_algo_t = 8;
pub const GPGME_MD_SHA384: gpgme_hash_algo_t = 9;
pub const GPGME_MD_SHA512: gpgme_hash_algo_t = 10;
pub const GPGME_MD_MD4: gpgme_hash_algo_t = 301;
pub const GPGME_MD_CRC32: gpgme_hash_algo_t = 302;
pub const GPGME_MD_CRC32_RFC1510: gpgme_hash_algo_t = 303;
pub const GPGME_MD_CRC24_RFC2440: gpgme_hash_algo_t = 304;

pub type gpgme_sig_stat_t = libc::c_uint;
pub const GPGME_SIG_STAT_NONE: gpgme_sig_stat_t = 0;
pub const GPGME_SIG_STAT_GOOD: gpgme_sig_stat_t = 1;
pub const GPGME_SIG_STAT_BAD: gpgme_sig_stat_t = 2;
pub const GPGME_SIG_STAT_NOKEY: gpgme_sig_stat_t = 3;
pub const GPGME_SIG_STAT_NOSIG: gpgme_sig_stat_t = 4;
pub const GPGME_SIG_STAT_ERROR: gpgme_sig_stat_t = 5;
pub const GPGME_SIG_STAT_DIFF: gpgme_sig_stat_t = 6;
pub const GPGME_SIG_STAT_GOOD_EXP: gpgme_sig_stat_t = 7;
pub const GPGME_SIG_STAT_GOOD_EXPKEY: gpgme_sig_stat_t = 8;

pub type gpgme_sig_mode_t = libc::c_uint;
pub const GPGME_SIG_MODE_NORMAL: gpgme_sig_mode_t = 0;
pub const GPGME_SIG_MODE_DETACH: gpgme_sig_mode_t = 1;
pub const GPGME_SIG_MODE_CLEAR: gpgme_sig_mode_t = 2;

pub type gpgme_attr_t = libc::c_uint;
pub const GPGME_ATTR_KEYID: gpgme_attr_t = 1;
pub const GPGME_ATTR_FPR: gpgme_attr_t = 2;
pub const GPGME_ATTR_ALGO: gpgme_attr_t = 3;
pub const GPGME_ATTR_LEN: gpgme_attr_t = 4;
pub const GPGME_ATTR_CREATED: gpgme_attr_t = 5;
pub const GPGME_ATTR_EXPIRE: gpgme_attr_t = 6;
pub const GPGME_ATTR_OTRUST: gpgme_attr_t = 7;
pub const GPGME_ATTR_USERID: gpgme_attr_t = 8;
pub const GPGME_ATTR_NAME: gpgme_attr_t = 9;
pub const GPGME_ATTR_EMAIL: gpgme_attr_t = 10;
pub const GPGME_ATTR_COMMENT: gpgme_attr_t = 11;
pub const GPGME_ATTR_VALIDITY: gpgme_attr_t = 12;
pub const GPGME_ATTR_LEVEL: gpgme_attr_t = 13;
pub const GPGME_ATTR_TYPE: gpgme_attr_t = 14;
pub const GPGME_ATTR_IS_SECRET: gpgme_attr_t = 15;
pub const GPGME_ATTR_KEY_REVOKED: gpgme_attr_t = 16;
pub const GPGME_ATTR_KEY_INVALID: gpgme_attr_t = 17;
pub const GPGME_ATTR_UID_REVOKED: gpgme_attr_t = 18;
pub const GPGME_ATTR_UID_INVALID: gpgme_attr_t = 19;
pub const GPGME_ATTR_KEY_CAPS: gpgme_attr_t = 20;
pub const GPGME_ATTR_CAN_ENCRYPT: gpgme_attr_t = 21;
pub const GPGME_ATTR_CAN_SIGN: gpgme_attr_t = 22;
pub const GPGME_ATTR_CAN_CERTIFY: gpgme_attr_t = 23;
pub const GPGME_ATTR_KEY_EXPIRED: gpgme_attr_t = 24;
pub const GPGME_ATTR_KEY_DISABLED: gpgme_attr_t = 25;
pub const GPGME_ATTR_SERIAL: gpgme_attr_t = 26;
pub const GPGME_ATTR_ISSUER: gpgme_attr_t = 27;
pub const GPGME_ATTR_CHAINID: gpgme_attr_t = 28;
pub const GPGME_ATTR_SIG_STATUS: gpgme_attr_t = 29;
pub const GPGME_ATTR_ERRTOK: gpgme_attr_t = 30;
pub const GPGME_ATTR_SIG_SUMMARY: gpgme_attr_t = 31;
pub const GPGME_ATTR_SIG_CLASS: gpgme_attr_t = 32;

pub type gpgme_validity_t = libc::c_uint;
pub const GPGME_VALIDITY_UNKNOWN: gpgme_validity_t = 0;
pub const GPGME_VALIDITY_UNDEFINED: gpgme_validity_t = 1;
pub const GPGME_VALIDITY_NEVER: gpgme_validity_t = 2;
pub const GPGME_VALIDITY_MARGINAL: gpgme_validity_t = 3;
pub const GPGME_VALIDITY_FULL: gpgme_validity_t = 4;
pub const GPGME_VALIDITY_ULTIMATE: gpgme_validity_t = 5;

pub type gpgme_protocol_t = libc::c_uint;
pub const GPGME_PROTOCOL_OpenPGP: gpgme_protocol_t = 0;
pub const GPGME_PROTOCOL_CMS: gpgme_protocol_t = 1;
pub const GPGME_PROTOCOL_GPGCONF: gpgme_protocol_t = 2;
pub const GPGME_PROTOCOL_ASSUAN: gpgme_protocol_t = 3;
pub const GPGME_PROTOCOL_G13: gpgme_protocol_t = 4;
pub const GPGME_PROTOCOL_UISERVER: gpgme_protocol_t = 5;
pub const GPGME_PROTOCOL_DEFAULT: gpgme_protocol_t = 254;
pub const GPGME_PROTOCOL_UNKNOWN: gpgme_protocol_t = 255;

pub type gpgme_keylist_mode_t = libc::c_uint;
pub const GPGME_KEYLIST_MODE_LOCAL: gpgme_keylist_mode_t = 1;
pub const GPGME_KEYLIST_MODE_EXTERN: gpgme_keylist_mode_t = 2;
pub const GPGME_KEYLIST_MODE_SIGS: gpgme_keylist_mode_t = 4;
pub const GPGME_KEYLIST_MODE_SIG_NOTATIONS: gpgme_keylist_mode_t = 8;
pub const GPGME_KEYLIST_MODE_EPHEMERAL: gpgme_keylist_mode_t = 128;
pub const GPGME_KEYLIST_MODE_VALIDATE: gpgme_keylist_mode_t = 256;

pub type gpgme_pinentry_mode_t = libc::c_uint;
pub const GPGME_PINENTRY_MODE_DEFAULT: gpgme_pinentry_mode_t = 0;
pub const GPGME_PINENTRY_MODE_ASK: gpgme_pinentry_mode_t = 1;
pub const GPGME_PINENTRY_MODE_CANCEL: gpgme_pinentry_mode_t = 2;
pub const GPGME_PINENTRY_MODE_ERROR: gpgme_pinentry_mode_t = 3;
pub const GPGME_PINENTRY_MODE_LOOPBACK: gpgme_pinentry_mode_t = 4;

pub type gpgme_export_mode_t = libc::c_uint;
pub const GPGME_EXPORT_MODE_EXTERN: gpgme_export_mode_t = 2;
pub const GPGME_EXPORT_MODE_MINIMAL: gpgme_export_mode_t = 4;

pub const GPGME_AUDITLOG_HTML: libc::c_uint = 1;
pub const GPGME_AUDITLOG_WITH_HELP: libc::c_uint = 128;

pub type gpgme_sig_notation_flags_t = libc::c_uint;
pub const GPGME_SIG_NOTATION_HUMAN_READABLE: gpgme_sig_notation_flags_t = 1;
pub const GPGME_SIG_NOTATION_CRITICAL: gpgme_sig_notation_flags_t = 2;

pub type gpgme_status_code_t = libc::c_uint;
pub const GPGME_STATUS_EOF: gpgme_status_code_t = 0;

pub const GPGME_STATUS_ENTER: gpgme_status_code_t = 1;
pub const GPGME_STATUS_LEAVE: gpgme_status_code_t = 2;
pub const GPGME_STATUS_ABORT: gpgme_status_code_t = 3;

pub const GPGME_STATUS_GOODSIG: gpgme_status_code_t = 4;
pub const GPGME_STATUS_BADSIG: gpgme_status_code_t = 5;
pub const GPGME_STATUS_ERRSIG: gpgme_status_code_t = 6;

pub const GPGME_STATUS_BADARMOR: gpgme_status_code_t = 7;

pub const GPGME_STATUS_RSA_OR_IDEA: gpgme_status_code_t = 8;
pub const GPGME_STATUS_KEYEXPIRED: gpgme_status_code_t = 9;
pub const GPGME_STATUS_KEYREVOKED: gpgme_status_code_t = 10;

pub const GPGME_STATUS_TRUST_UNDEFINED: gpgme_status_code_t = 11;
pub const GPGME_STATUS_TRUST_NEVER: gpgme_status_code_t = 12;
pub const GPGME_STATUS_TRUST_MARGINAL: gpgme_status_code_t = 13;
pub const GPGME_STATUS_TRUST_FULLY: gpgme_status_code_t = 14;
pub const GPGME_STATUS_TRUST_ULTIMATE: gpgme_status_code_t = 15;

pub const GPGME_STATUS_SHM_INFO: gpgme_status_code_t = 16;
pub const GPGME_STATUS_SHM_GET: gpgme_status_code_t = 17;
pub const GPGME_STATUS_SHM_GET_BOOL: gpgme_status_code_t = 18;
pub const GPGME_STATUS_SHM_GET_HIDDEN: gpgme_status_code_t = 19;

pub const GPGME_STATUS_NEED_PASSPHRASE: gpgme_status_code_t = 20;
pub const GPGME_STATUS_VALIDSIG: gpgme_status_code_t = 21;
pub const GPGME_STATUS_SIG_ID: gpgme_status_code_t = 22;
pub const GPGME_STATUS_ENC_TO: gpgme_status_code_t = 23;
pub const GPGME_STATUS_NODATA: gpgme_status_code_t = 24;
pub const GPGME_STATUS_BAD_PASSPHRASE: gpgme_status_code_t = 25;
pub const GPGME_STATUS_NO_PUBKEY: gpgme_status_code_t = 26;
pub const GPGME_STATUS_NO_SECKEY: gpgme_status_code_t = 27;
pub const GPGME_STATUS_NEED_PASSPHRASE_SYM: gpgme_status_code_t = 28;
pub const GPGME_STATUS_DECRYPTION_FAILED: gpgme_status_code_t = 29;
pub const GPGME_STATUS_DECRYPTION_OKAY: gpgme_status_code_t = 30;
pub const GPGME_STATUS_MISSING_PASSPHRASE: gpgme_status_code_t = 31;
pub const GPGME_STATUS_GOOD_PASSPHRASE: gpgme_status_code_t = 32;
pub const GPGME_STATUS_GOODMDC: gpgme_status_code_t = 33;
pub const GPGME_STATUS_BADMDC: gpgme_status_code_t = 34;
pub const GPGME_STATUS_ERRMDC: gpgme_status_code_t = 35;
pub const GPGME_STATUS_IMPORTED: gpgme_status_code_t = 36;
pub const GPGME_STATUS_IMPORT_OK: gpgme_status_code_t = 37;
pub const GPGME_STATUS_IMPORT_PROBLEM: gpgme_status_code_t = 38;
pub const GPGME_STATUS_IMPORT_RES: gpgme_status_code_t = 39;
pub const GPGME_STATUS_FILE_START: gpgme_status_code_t = 40;
pub const GPGME_STATUS_FILE_DONE: gpgme_status_code_t = 41;
pub const GPGME_STATUS_FILE_ERROR: gpgme_status_code_t = 42;

pub const GPGME_STATUS_BEGIN_DECRYPTION: gpgme_status_code_t = 43;
pub const GPGME_STATUS_END_DECRYPTION: gpgme_status_code_t = 44;
pub const GPGME_STATUS_BEGIN_ENCRYPTION: gpgme_status_code_t = 45;
pub const GPGME_STATUS_END_ENCRYPTION: gpgme_status_code_t = 46;

pub const GPGME_STATUS_DELETE_PROBLEM: gpgme_status_code_t = 47;
pub const GPGME_STATUS_GET_BOOL: gpgme_status_code_t = 48;
pub const GPGME_STATUS_GET_LINE: gpgme_status_code_t = 49;
pub const GPGME_STATUS_GET_HIDDEN: gpgme_status_code_t = 50;
pub const GPGME_STATUS_GOT_IT: gpgme_status_code_t = 51;
pub const GPGME_STATUS_PROGRESS: gpgme_status_code_t = 52;
pub const GPGME_STATUS_SIG_CREATED: gpgme_status_code_t = 53;
pub const GPGME_STATUS_SESSION_KEY: gpgme_status_code_t = 54;
pub const GPGME_STATUS_NOTATION_NAME: gpgme_status_code_t = 55;
pub const GPGME_STATUS_NOTATION_DATA: gpgme_status_code_t = 56;
pub const GPGME_STATUS_POLICY_URL: gpgme_status_code_t = 57;
pub const GPGME_STATUS_BEGIN_STREAM: gpgme_status_code_t = 58;
pub const GPGME_STATUS_END_STREAM: gpgme_status_code_t = 59;
pub const GPGME_STATUS_KEY_CREATED: gpgme_status_code_t = 60;
pub const GPGME_STATUS_USERID_HINT: gpgme_status_code_t = 61;
pub const GPGME_STATUS_UNEXPECTED: gpgme_status_code_t = 62;
pub const GPGME_STATUS_INV_RECP: gpgme_status_code_t = 63;
pub const GPGME_STATUS_NO_RECP: gpgme_status_code_t = 64;
pub const GPGME_STATUS_ALREADY_SIGNED: gpgme_status_code_t = 65;
pub const GPGME_STATUS_SIGEXPIRED: gpgme_status_code_t = 66;
pub const GPGME_STATUS_EXPSIG: gpgme_status_code_t = 67;
pub const GPGME_STATUS_EXPKEYSIG: gpgme_status_code_t = 68;
pub const GPGME_STATUS_TRUNCATED: gpgme_status_code_t = 69;
pub const GPGME_STATUS_ERROR: gpgme_status_code_t = 70;
pub const GPGME_STATUS_NEWSIG: gpgme_status_code_t = 71;
pub const GPGME_STATUS_REVKEYSIG: gpgme_status_code_t = 72;
pub const GPGME_STATUS_SIG_SUBPACKET: gpgme_status_code_t = 73;
pub const GPGME_STATUS_NEED_PASSPHRASE_PIN: gpgme_status_code_t = 74;
pub const GPGME_STATUS_SC_OP_FAILURE: gpgme_status_code_t = 75;
pub const GPGME_STATUS_SC_OP_SUCCESS: gpgme_status_code_t = 76;
pub const GPGME_STATUS_CARDCTRL: gpgme_status_code_t = 77;
pub const GPGME_STATUS_BACKUP_KEY_CREATED: gpgme_status_code_t = 78;
pub const GPGME_STATUS_PKA_TRUST_BAD: gpgme_status_code_t = 79;
pub const GPGME_STATUS_PKA_TRUST_GOOD: gpgme_status_code_t = 80;
pub const GPGME_STATUS_PLAINTEXT: gpgme_status_code_t = 81;
pub const GPGME_STATUS_INV_SGNR: gpgme_status_code_t = 82;
pub const GPGME_STATUS_NO_SGNR: gpgme_status_code_t = 83;
pub const GPGME_STATUS_SUCCESS: gpgme_status_code_t = 84;
pub const GPGME_STATUS_DECRYPTION_INFO: gpgme_status_code_t = 85;

pub const GPGME_INCLUDE_CERTS_DEFAULT: libc::c_int = -256;

pub type gpgme_event_io_t = libc::c_uint;
pub const GPGME_EVENT_START: gpgme_event_io_t = 0;
pub const GPGME_EVENT_DONE: gpgme_event_io_t = 1;
pub const GPGME_EVENT_NEXT_KEY: gpgme_event_io_t = 2;
pub const GPGME_EVENT_NEXT_TRUSTITEM: gpgme_event_io_t = 3;

pub type gpgme_encrypt_flags_t = libc::c_uint;
pub const GPGME_ENCRYPT_ALWAYS_TRUST: gpgme_encrypt_flags_t = 1;
pub const GPGME_ENCRYPT_NO_ENCRYPT_TO: gpgme_encrypt_flags_t = 2;
pub const GPGME_ENCRYPT_PREPARE: gpgme_encrypt_flags_t = 4;
pub const GPGME_ENCRYPT_EXPECT_SIGN: gpgme_encrypt_flags_t = 8;


pub type gpgme_sigsum_t = libc::c_uint;
pub const GPGME_SIGSUM_VALID: gpgme_sigsum_t = 0x0001;
pub const GPGME_SIGSUM_GREEN: gpgme_sigsum_t = 0x0002;
pub const GPGME_SIGSUM_RED:  gpgme_sigsum_t = 0x0004;
pub const GPGME_SIGSUM_KEY_REVOKED: gpgme_sigsum_t = 0x0010;
pub const GPGME_SIGSUM_KEY_EXPIRED: gpgme_sigsum_t = 0x0020;
pub const GPGME_SIGSUM_SIG_EXPIRED: gpgme_sigsum_t = 0x0040;
pub const GPGME_SIGSUM_KEY_MISSING: gpgme_sigsum_t = 0x0080;
pub const GPGME_SIGSUM_CRL_MISSING: gpgme_sigsum_t = 0x0100;
pub const GPGME_SIGSUM_CRL_TOO_OLD: gpgme_sigsum_t = 0x0200;
pub const GPGME_SIGSUM_BAD_POLICY: gpgme_sigsum_t = 0x0400;
pub const GPGME_SIGSUM_SYS_ERROR: gpgme_sigsum_t = 0x0800;

pub const GPGME_IMPORT_NEW: libc::c_uint = 1;
pub const GPGME_IMPORT_UID: libc::c_uint = 2;
pub const GPGME_IMPORT_SIG: libc::c_uint = 4;
pub const GPGME_IMPORT_SUBKEY: libc::c_uint = 8;
pub const GPGME_IMPORT_SECRET: libc::c_uint = 16;

pub type gpgme_conf_level_t = libc::c_uint;
pub const GPGME_CONF_BASIC: gpgme_conf_level_t = 0;
pub const GPGME_CONF_ADVANCED: gpgme_conf_level_t = 1;
pub const GPGME_CONF_EXPERT: gpgme_conf_level_t = 2;
pub const GPGME_CONF_INVISIBLE: gpgme_conf_level_t = 3;
pub const GPGME_CONF_INTERNAL: gpgme_conf_level_t = 4;

pub type gpgme_conf_type_t = libc::c_uint;
pub const GPGME_CONF_NONE: gpgme_conf_type_t = 0;
pub const GPGME_CONF_STRING: gpgme_conf_type_t = 1;
pub const GPGME_CONF_INT32: gpgme_conf_type_t = 2;
pub const GPGME_CONF_UINT32: gpgme_conf_type_t = 3;

pub const GPGME_CONF_FILENAME: gpgme_conf_type_t = 32;
pub const GPGME_CONF_LDAP_SERVER: gpgme_conf_type_t = 33;
pub const GPGME_CONF_KEY_FPR: gpgme_conf_type_t = 34;
pub const GPGME_CONF_PUB_KEY: gpgme_conf_type_t = 35;
pub const GPGME_CONF_SEC_KEY: gpgme_conf_type_t = 36;
pub const GPGME_CONF_ALIAS_LIST: gpgme_conf_type_t = 37;

pub const GPGME_CONF_GROUP: gpgme_conf_type_t = (1 << 0);
pub const GPGME_CONF_OPTIONAL: gpgme_conf_type_t = (1 << 1);
pub const GPGME_CONF_LIST: gpgme_conf_type_t = (1 << 2);
pub const GPGME_CONF_RUNTIME: gpgme_conf_type_t = (1 << 3);
pub const GPGME_CONF_DEFAULT: gpgme_conf_type_t = (1 << 4);
pub const GPGME_CONF_DEFAULT_DESC: gpgme_conf_type_t = (1 << 5);
pub const GPGME_CONF_NO_ARG_DESC: gpgme_conf_type_t = (1 << 6);
pub const GPGME_CONF_NO_CHANGE: gpgme_conf_type_t = (1 << 7);
