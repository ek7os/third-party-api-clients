A fully generated, opinionated API client library for Ramp.

This library is generated from the Ramp OpenAPI
specs. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.

To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
ramp_api = "0.2.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use ramp_api::Client;

let ramp = Client::new(
    String::from("client-id"),
    String::from("client-secret"),
    String::from("redirect-uri"),
    String::from("token"),
    String::from("refresh-token")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `RAMP_CLIENT_ID`
- `RAMP_CLIENT_SECRET`
- `RAMP_REDIRECT_URI`

And then you can create a client from the environment.

```
use ramp_api::Client;

let ramp = Client::new_from_env(
    String::from("token"),
    String::from("refresh-token")
);
```

It is okay to pass empty values for token and `refresh_token`. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a `token` and `refresh_token`, use the following.

```
use ramp_api::Client;

async fn do_call() {
    let mut ramp = Client::new_from_env("", "");

    // Get the URL to request consent from the user.
    let user_consent_url = ramp.user_consent_url();

    // In your redirect URL capture the code sent.
    // Send it along to the request for the token.
    let code = "thing-from-redirect-url";
    let mut access_token = ramp.get_access_token(code).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = ramp.refresh_access_token().await.unwrap();
}
```