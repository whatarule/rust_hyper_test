### requirement:
  - You need to set the environment variable below.
    - TOKEN: access token for github
      - https://docs.github.com/en/rest/guides/getting-started-with-the-rest-api#using-personal-access-tokens
### usage:
```
$ cargo run <github api endpoint uri>

// example:
$ cargo run https://api.github.com/user
```
### references:
  - getting started: 
    - https://hyper.rs/guides/client/basic/
    - https://docs.github.com/en/rest/guides/getting-started-with-the-rest-api
  - https:
    - https://hyper.rs/guides/client/configuration/
  - with headers: 
    - https://hyper.rs/guides/client/advanced/
    - https://github.com/hyperium/hyper/blob/master/examples/client.rs
    - https://docs.github.com/en/actions/security-guides/automatic-token-authentication#example-2-calling-the-rest-api
