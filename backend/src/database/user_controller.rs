use crate::general_helpers::ENV_VARS;

use super::types::user::User;

fn get_mock_data() -> Vec<User> {
    return vec![
        User::new(
            "rasmus".to_string(),
            vec!["jonathan".to_string(), "darth vader".to_string()],
        ),
        User::new(
            "jonathan".to_string(),
            vec!["rasmus".to_string(), "darth vader".to_string()],
        ),
        User::new(
            "darth vader".to_string(),
            vec!["rasmus".to_string(), "jonathan".to_string()],
        ),
    ];
}

pub fn read_users(ids: Vec<String>) -> Vec<User> {
    if ENV_VARS.use_mocked_database {
        return get_mock_data()
            .iter()
            .filter(|user| ids.contains(&user.id))
            .map(|user| user.clone())
            .collect::<Vec<User>>();
    } else {
        todo!("Implement this part of the database interaction");
    }
}
