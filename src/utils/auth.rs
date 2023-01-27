use crate::utils::security::jwt_decode;

pub async fn get_user_id(header_authorization: &str) -> i32 {
    let jwt_decoded = jwt_decode(&header_authorization);

    jwt_decoded.unwrap().id.parse::<i32>().unwrap()
}