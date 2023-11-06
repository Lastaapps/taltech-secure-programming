# Brutus
Totally secure web store for all your ciphers.

Web can be found at [lastope2.sh.cvut.cz/brutus](https://lastope2.sh.cvut.cz/brutus)

## Building and running
Same as normal rust application
(it will take a while to build for the first time):
```cargo run --release```

After that access the URL shown at the end of the logs.

After you register new admin account
run `sqlite3 database.sqlite < make_admin_admin.sql`
to make admin real admin.

As you can see, the project uses SQLite - don't be surprised when
some operations are kind of slow.
But for a homework it is fine in my opinion.

### Docker
You can also build a docker image and run it,
don't forget about env variables (they are of course not in the image)
and volumes (not included bellow, as I will use docker compose anyway).

```
docker build -t brutus .
docker run -it --env-file .env -p 8000:8000 brutus
```

## Security
Secrets are stored in .env.
They are version only to make it simpler for you to set up the project.

As the apps runs on localhost, it does not use HTTPS and secure cookies.
Other than that it should be quite secure (I hope).
JWT token is signed and encrypted, http-only flag is set,
code injection does not work, ... (I hope again).

User authentication is handled using JWT tokens.
Methods are checked by using either
`Antonius` (any logged-in user) or
`Ceasar` (system admin).

I also made an attempt to implement soft delete.

Also, this app uses no JS! (Was no easy to achieve).

## Structure
`./migrations` - SQL commands to create the database tables, run automatically
`./templates` - HTML templates for Tera templating engine
`./target` - rust build files, delete after grading, can be quite huge
`./src` - code
`./src/api` - all the API endpoint and their logic
`./src/domain` - shared business logic

