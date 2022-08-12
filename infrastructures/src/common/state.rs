use applications::users::UserApplicationService;
use presentation::session::SessionService;

/// ハンドラ間で共有されるオブジェクト
pub trait State {
    type UserApplicationServiceState: UserApplicationService;
    type SessionServiceState: SessionService;

    fn user_application_service(&self) -> &Self::UserApplicationServiceState;
    fn session_service(&self) -> &Self::SessionServiceState;
}
