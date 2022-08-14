use crate::{auth::OICDService, session::SessionService};
use applications::users::UserApplicationService;

/// ハンドラ間で共有されるオブジェクト
pub trait Utility {
    type UserApplicationServiceState: UserApplicationService;
    type SessionServiceState: SessionService;
    type OICDServiceState: OICDService;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState;
    fn session_service(&self) -> &Self::SessionServiceState;
    fn oicd_service(&self) -> &Self::OICDServiceState;
}
