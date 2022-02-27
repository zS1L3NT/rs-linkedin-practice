#[allow(dead_code)]
fn sort_usernames(users: &mut Vec<&str>) {
    users.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
