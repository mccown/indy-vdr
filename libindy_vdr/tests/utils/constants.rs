use indy_vdr::ledger::constants::{
    RICH_SCHEMA, RICH_SCHEMA_CRED_DEF, RICH_SCHEMA_CTX, RICH_SCHEMA_ENCODING, RICH_SCHEMA_MAPPING,
    RICH_SCHEMA_PRES_DEF, RS_CONTEXT_TYPE_VALUE, RS_CRED_DEF_TYPE_VALUE, RS_ENCODING_TYPE_VALUE,
    RS_MAPPING_TYPE_VALUE, RS_PRES_DEF_TYPE_VALUE, RS_SCHEMA_TYPE_VALUE,
};
use std::collections::HashMap;

pub const TRUSTEE_SEED: [u8; 64] = [
    48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
    84, 114, 117, 115, 116, 101, 101, 49, 227, 58, 175, 56, 31, 255, 166, 16, 154, 213, 145, 253,
    195, 135, 23, 148, 95, 143, 171, 247, 171, 240, 32, 134, 174, 64, 28, 99, 233, 145, 48, 151,
];
pub const TRUSTEE_DID: &str = "V4SGRU86Z58d6TV7PBUe6f";
pub const TRUSTEE_DID_FQ: &str = "did:sov:V4SGRU86Z58d6TV7PBUe6f";
//pub const TRUSTEE_VERKEY: &str = "GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
pub const STEWARD_SEED: [u8; 64] = [
    48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48,
    83, 116, 101, 119, 97, 114, 100, 49, 216, 39, 70, 88, 210, 59, 194, 228, 25, 77, 18, 50, 255,
    247, 14, 43, 52, 75, 175, 98, 16, 103, 235, 106, 169, 43, 47, 206, 240, 201, 78, 141,
];
pub const STEWARD_DID: &str = "V4SGRU86Z58d6TV7PBUe6f"; // TODO: change
                                                        //pub const MY1_SEED: [u8;64] = [48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 48, 77, 121, 49, 233, 198, 118, 109, 146, 166, 42, 37, 34, 23, 211, 11, 40, 37, 124, 32, 134, 8, 18, 195, 139, 30, 194, 70, 230, 160, 73, 245, 42, 208, 96, 25];
pub const MY1_DID: &str = "VsKV7grR1BUE29mG2Fm2kX";
pub const MY1_DID_FQ: &str = "did:sov:VsKV7grR1BUE29mG2Fm2kX";
pub const MY1_VERKEY: &str = "GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa";

lazy_static! {
    pub static ref RS_TYPE_TO_OP: HashMap<&'static str, &'static str> = [
        (RS_SCHEMA_TYPE_VALUE, RICH_SCHEMA),
        (RS_ENCODING_TYPE_VALUE, RICH_SCHEMA_ENCODING),
        (RS_CONTEXT_TYPE_VALUE, RICH_SCHEMA_CTX),
        (RS_MAPPING_TYPE_VALUE, RICH_SCHEMA_MAPPING),
        (RS_CRED_DEF_TYPE_VALUE, RICH_SCHEMA_CRED_DEF),
        (RS_PRES_DEF_TYPE_VALUE, RICH_SCHEMA_PRES_DEF),
    ]
    .iter()
    .copied()
    .collect();
}
