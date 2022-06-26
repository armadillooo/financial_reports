#[cfg(test)]
mod tests {
    mod user_tests {
        use std::matches;
        use std::sync::Arc;

        use applications::users::{
            CreateCommand, DeleteCommand, GetCommand, UserApplicationService, UserData,
        };
        use infrastructures::users::InMemoryUserRepository;

        #[test]
        fn create_user_success() {
            let repository = Arc::new(InMemoryUserRepository::new());
            let app_service = UserApplicationService::new(Arc::clone(&repository));
            let id = "1";
            let name = "hoge";
            let command = CreateCommand::new(id, name);

            app_service.save(command).expect("User save failed");

            let user = UserData::new(id, name);
            let store_user = Arc::clone(&repository)
                .store
                .borrow()
                .get(id)
                .expect("User not found.")
                .clone();
            let store_user = UserData::from(store_user.clone());

            assert_eq!(store_user, user);
        }

        #[test]
        fn create_same_user_error() {
            let repository = Arc::new(InMemoryUserRepository::new());
            let app_service = UserApplicationService::new(Arc::clone(&repository));
            let id = "1";
            let name = "hoge";
            let command = CreateCommand::new(id, name);
            let id2 = "1";
            let name2 = "sample name";
            let second_command = CreateCommand::new(id2, name2);

            app_service.save(command).expect("Save user failed");
            let result = app_service.save(second_command);

            assert!(matches!(result, Err(_)));
            assert_eq!(Arc::clone(&repository).store.borrow().len(), 1)
        }

        #[test]
        fn get_user_success() {
            let repository = Arc::new(InMemoryUserRepository::new());
            let app_service = UserApplicationService::new(Arc::clone(&repository));
            let id = "234";
            let name = "delete user";
            let user = UserData::new(id, name);
            let create_command = CreateCommand::new(id, name);
            let get_command = GetCommand::new(id);

            app_service.save(create_command).expect("User save failed");
            let result = app_service.get(get_command).expect("User not found");

            assert_eq!(user, result);
        }

        #[test]
        fn get_not_exist_user_error() {
            let repository = Arc::new(InMemoryUserRepository::new());
            let app_service = UserApplicationService::new(Arc::clone(&repository));
            let id = "234";
            let get_command = GetCommand::new(id);

            let result = app_service.get(get_command);

            assert!(matches!(result, Err(_)))
        }

        #[test]
        fn delete_user_success() {
            let repository = Arc::new(InMemoryUserRepository::new());
            let app_service = UserApplicationService::new(Arc::clone(&repository));
            let id = "234";
            let name = "delete user";
            let create_command = CreateCommand::new(id, name);
            let delete_command = DeleteCommand::new(id);
            let get_command = GetCommand::new(id);

            app_service.save(create_command).expect("User save failed");
            assert!(matches!(app_service.get(get_command), Ok(_)));

            app_service.delete(delete_command).expect("User not found");
            let get_command = GetCommand::new(id);
            assert!(matches!(app_service.get(get_command), Err(_)))
        }

        #[test]
        fn delete_not_exist_user_success() {
            let repository = Arc::new(InMemoryUserRepository::new());
            let app_service = UserApplicationService::new(Arc::clone(&repository));
            let id = "234";
            let get_command = GetCommand::new(id);
            let delete_command = DeleteCommand::new(id);

            assert!(matches!(app_service.get(get_command), Err(_)));
            assert!(matches!(app_service.delete(delete_command), Ok(_)))
        }
    }
}
