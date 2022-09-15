use crate::routes::session::Session;

pub fn get_session_by_id(session_id: u32) -> Session {
    let session = Session {
        session_id,
    };

   session
}
