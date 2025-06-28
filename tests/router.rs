use schnell::router::match_route;

#[test]
fn test_match_route() {
    assert!(match_route("/", "/"));
    assert!(match_route("/users", "/users"));
    assert!(match_route("/users/:id", "/users/123"));
    assert!(match_route("/users/:id", "/users/123"));
    assert!(match_route("/users/:id", "/users/123"));
    assert_eq!(match_route("/users/:id", "/users/123/"), true);
    assert_eq!(
        match_route("/users/messages/:message_id", "/users/:userid"),
        false
    );
    assert_eq!(
        match_route("/users/messages/:message_id", "/users/123/messages/456"),
        true
    );
    assert_eq!(
        match_route("/users/messages/:message_id", "/users/123/messages/456/"),
        false
    );
}
