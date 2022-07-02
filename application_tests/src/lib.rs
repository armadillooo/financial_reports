#[cfg(test)]
mod tests {
    mod user_usecase_tests {
        use std::matches;
        use std::sync::Arc;

        use applications::users::{
            CreateCommand, DeleteCommand, GetCommand, UserApplicationService, UserData,
        };
        use infrastructures::users::InMemoryUserRepository;

        // テストに必要なオブジェクトの初期化
        fn setup() -> UserApplicationService<InMemoryUserRepository> {
            let user_repository = Arc::new(InMemoryUserRepository::new());
            let user_application = UserApplicationService::new(Arc::clone(&user_repository));

            user_application
        }

        #[test]
        fn create_user_saved() {
            let app_service = setup();
            let id = "1";
            let name = "hoge";
            let create_command = CreateCommand::new(id, name);
            let get_command = GetCommand::new(id);
            let created_user = UserData::new(id, name);

            assert!(matches!(app_service.save(create_command), Ok(_)));

            let get_user = app_service.get(get_command).unwrap();
            assert_eq!(get_user, created_user);
        }

        #[test]
        fn create_same_user_not_saved() {
            let app_service = setup();

            let id = "1";
            let name1 = "hoge";
            let create_command = CreateCommand::new(id, name1);
            let name2 = "sample name";
            let create_same_user_command = CreateCommand::new(id, name2);
            let get_command = GetCommand::new(id);
            let created_user = UserData::new(id, name1);

            assert!(matches!(app_service.save(create_command), Ok(_)));

            assert!(matches!(app_service.save(create_same_user_command), Err(_)));

            let get_user = app_service.get(get_command).unwrap();
            assert_eq!(get_user, created_user);
        }

        #[test]
        fn get_not_exist_user_return_error() {
            let app_service = setup();
            let id = "234";
            let get_command = GetCommand::new(id);

            assert!(matches!(app_service.get(get_command), Err(_)))
        }

        #[test]
        fn delete_user_return_ok() {
            let app_service = setup();
            let id = "234";
            let name = "delete user";
            let created_user = UserData::new(id, name);
            let create_command = CreateCommand::new(id, name);
            let delete_command = DeleteCommand::new(id);
            let get_command = GetCommand::new(id);

            assert!(matches!(app_service.save(create_command), Ok(_)));

            assert_eq!(app_service.get(get_command).unwrap(), created_user);

            assert!(matches!(app_service.delete(delete_command), Ok(_)));

            let get_command = GetCommand::new(id);
            assert!(matches!(app_service.get(get_command), Err(_)))
        }

        #[test]
        fn delete_not_exist_user_return_ok() {
            let app_service = setup();
            let id = "234";
            let get_command = GetCommand::new(id);
            let delete_command = DeleteCommand::new(id);

            assert!(matches!(app_service.get(get_command), Err(_)));
            
            assert!(matches!(app_service.delete(delete_command), Ok(_)))
        }
    }
}
