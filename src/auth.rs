use ring::rand::{SecureRandom, SystemRandom};
use rocket::form::Form;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::{Request, State};
use rocket_dyn_templates::Template;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::base::functions::to_i32;

const AUTH_COOKIE_NAME: &'static str = "AUTH_COOKIE";

// User data in Session.
pub struct User {
    pub client: i32,
    pub user_id: String,
}

pub struct SessionStorage {
    sessions: Arc<RwLock<HashMap<String, String>>>,
}

impl SessionStorage {
    pub fn new() -> Self {
        SessionStorage {
            sessions: Arc::new(Default::default()),
        }
    }

    /**
        Creates a new session for a given user, whereas one user can have multiple active sessions.
        The session ID is generated from a secure random number (base16 encoded)
            and stored in the sessions map so that when a user presents us with this id, we can look
            up their identity. Since we need to modify the session store by inserting,
            this requires a write lock.
        This function will panic, if the RwLock is poisoned,

    */
    pub fn create_session(&self, user: User) -> String {
        self.sessions
            .write()
            .map(|mut guard| {
                let mut bytes = [0u8; 32];
                SystemRandom::new()
                    .fill(&mut bytes)
                    .expect("System Random is broken");
                let session_id = hex::encode(bytes);
                guard.insert(session_id.clone(), user.user_id);
                session_id
            })
            .expect("Session storage lock is poisoned, aborting")
    }

    /**
        Loads a session from the store and returns the linked username.
        If the session is incalid and cannot be found, None is returned.
        This function will panic, if the RwLock is poisoned,
    */
    pub fn load_session(&self, session_id: &String) -> Option<User> {
        let guard = self.sessions.read().expect("Poision");
        let cu0 = guard
            .get(session_id)
            .map(|borrowed_user_name| borrowed_user_name.clone());
        if let Some(cu) = cu0 {
            let arr: Vec<&str> = cu.split("###").collect();
            return match arr.len() {
              2 => Some(User { client: to_i32(arr[0]), user_id : arr[1].to_string() }),
              _ => None
            };
        }
        None
    }

    /**
        Destroys a session by deleting the session ID from the store.
        Since we need to modify the session store by deleting, this requires a write lock.
        This function will panic, if the RwLock is poisoned,
    */
    pub fn destroy_session(&self, session_id: &String) {
        self.sessions
            .write()
            .map(|mut guard| {
                guard.remove(session_id);
            })
            .expect("Session storage lock is poisoned, aborting")
    }
}

/**
    Lots of things are going on here at the same time:
        - The User can be created from a request containing a valid session cookie.
            This can be done by implementing FromRequest for the Type. If the Creation fails, we
            return a Outcome::Failed to prevent any handlers to be called that require a user.
        - The From Request requires an associated type `Error`, we use String for simplicity here
        - This is the implementation of an async trait, currently not supported by rust.Therefore,
            the  `async-trait` crate is used, here re-exported by rocket. This enables us to use
            `await` syntax inthe trait implementation. For more details, read
            https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
        - Getting the session storage object from rockets managed state is async, hence we call
            await on it to allow processing of other requests concurrently.
        // TODO Fix Autn vs Authz
        - We generate a new User struct from the username for each and every request,
            since we do not have any important information in them to load properly from the DB.
*/
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User{
    type Error = String;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private(AUTH_COOKIE_NAME){
            Some(user_cookie) => {
                match request.guard::<&State<SessionStorage>>().await.succeeded(){
                    Some(guard) => {
                        match guard
                            .load_session(&user_cookie.value().to_string()) {
                            Some(user) => Outcome::Success(user),
                            None => Outcome::Failure((Status::Unauthorized,"Auth Cookie is invalid".to_string()))
                        }
                    }
                    None => Outcome::Failure((Status::InternalServerError,"Session storage poisioned".to_string()))
                }
            } ,
            None => Outcome::Failure((Status::Unauthorized,"Auth Cookie is missing".to_string()))
        }
    }
}

/**
    The Login and Logout routes handle the creation and destruction of session cookies.
    These cookies are stored as a `private cookie` and are protected against tampering by the user
        with authenticated encryption (AES-GCM). The key for this is configured in the `Rocket.toml`
         or re-created if not defined.
*/
#[get("/logout")]
pub fn logout_template(_user: User, cookies: &CookieJar<'_>, sessions: &State<SessionStorage>) -> Template {
    let cookie = cookies.get_private(AUTH_COOKIE_NAME)
        .expect("User guard worked but no Cookie?"); // this should be impossible to happen, hence we panic.
    sessions.destroy_session(&cookie.value().to_string());
    cookies.remove_private(cookie);
    Template::render("logout", &())
}

/**
    Holding the information that has to be passed from a login form.
    We do not handle CSRF attacks here
*/
#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginForm{
    client: String,
    username: String,
    password: String
}

#[get("/login")]
pub fn login_template() -> Template {
    let mut form = LoginForm{ client: 1.to_string(), username: "".into(), password: "".into() };
    if cfg!(debug_assertions) {
        form = LoginForm{ client: 1.to_string(), username: "wolfgang".into(), password: "wolfgang".into() };
    }
    Template::render("login", &form)
}

#[post("/login", format = "application/x-www-form-urlencoded", data = "<form>")]
pub fn login_form(form: Form<LoginForm>,  cookies: &CookieJar<'_>, sessions: &State<SessionStorage>) -> Result<Redirect, Redirect> {
    // Authentication is mocked by checking whether username and password are the same
    let authenticated = form.username == form.password;

    if authenticated {
        let user = User{ client: to_i32(form.client.as_str()), user_id: form.username.to_string() };
        let session_id = sessions.create_session(user);
        cookies.add_private(Cookie::new(AUTH_COOKIE_NAME, session_id));
        //Ok(Redirect::to("/admin/requests".to_string()))
        Ok(Redirect::to("/public".to_string()))
    }
    else {
        Err(Redirect::to("/auth/login".to_string()))
    }
}
