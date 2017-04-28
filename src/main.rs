fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        active: bool
    }

    let _first_user = User {
      username: String::from("ughitsaaron"),
      email: String::from("a.p.petcoff@gmail.com"),
      active: true
    };


    return ();
}
