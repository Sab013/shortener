use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Encode, Decode, Eq, Hash, Serialize, Deserialize, ToSchema)]
pub struct Slug(pub String);

#[derive(Clone, Debug, PartialEq, Encode, Decode, Serialize, Deserialize, ToSchema)]
pub struct LongUrl(pub String);

#[derive(Debug, Clone, PartialEq, Decode, Encode, Serialize, Deserialize, ToSchema)]
pub struct ShortLink {
    pub slug: Slug,
    pub url: LongUrl,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct LinkStats {
    pub link: ShortLink,
    pub redirect_count: u64,
}
