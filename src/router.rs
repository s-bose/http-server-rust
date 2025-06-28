pub fn match_route(route: &str, incoming: &str) -> bool {
    let route_parts = route.split('/').collect::<Vec<&str>>();
    let incoming_parts = incoming.split('/').collect::<Vec<&str>>();

    if route_parts.len() != incoming_parts.len() {
        return false;
    }

    for (route_part, incoming_part) in route_parts.iter().zip(incoming_parts.iter()) {
        if route_part.starts_with(':') {
            continue;
        }
        if route_part != incoming_part {
            return false;
        }
    }

    true
}
