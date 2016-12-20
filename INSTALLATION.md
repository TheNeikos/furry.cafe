# Installing the furry.cafe locally

If you plan on hosting the furry.cafe locally for development these are the
steps to take (on Linux, other platforms are not supported, but should still
work, you will have to find the equivalent steps on your own):

1. Install rust nightly, cargo, postgresql and ruby (for sass)
2. Install `diesel_cli` through cargo:
    - `cargo install diesel_cli`
3. Clone this repository and switch to it
4. `cp .env.example .env`
5. Put in the correct values in the in `.env` files
    - For local installs you can put anything for the `COOKIE_SECRET`
    - If you do not put in valid `MAILGUN_*` config you will receive an error
      but it should continue working. You should check the console output for
      the activation code.
6. Create a user and database in postgres, be sure to put them into the `.env`
   file
7. Start the server with `cargo run`

If you work on the css don't forget to run in the root folder of the site
```
sass --watch src_assets/:assets/ --style=compressed
```

This will compile the sass files and put them correctly into the assets path.
