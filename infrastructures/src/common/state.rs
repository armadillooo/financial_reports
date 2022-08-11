use applications::users::UserApplicationServiceImpl;
use domain::users::UserRepository;
use presentation::session::SessionService;

/// ハンドラ間で共有されるオブジェクト
#[derive(Debug, Clone)]
pub struct State<U, S>
where
    U: UserRepository,
    S: SessionService,
    //O: OICDService,
{
    user_service: UserApplicationServiceImpl<U>,
    session_service: S,
    //oicd_service: O,
}

impl<U, S> State<U, S>
where
    U: UserRepository + Clone,
    S: SessionService + Clone,
    //O: OICDService,
{
    /// コンストラクタ
    pub fn new(
        user_service: UserApplicationServiceImpl<U>,
        session_service: S,
        //oicd_service: O,
    ) -> Self {
        Self {
            user_service,
            session_service,
            //oicd_service,
        }
    }
}
