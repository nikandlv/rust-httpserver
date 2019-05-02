### modularized multi-application Rust http server / backend boilerplate

this is a highly versatile boilerplate project to get an http server up and running in no time

#### Why rust+actix ?

rust is known to be the fastest (atleast it tries to) language in the means of performance and safety and efficiency.

actix is a library for actors, so when combined with rust http server it will spawn new actors to handle the requests simultaneously, this will result in much much more capacity and performance

combining these two bad boys will result in an incredible performance

also rust type safety makes sure everything will run where it should be.

actix will provide alot of utility to handle incoming and outgoing data correct and with ease

#### Usecases

Rust http servers are mostly used for building fancy and fast API networks

Any low end systems needing an http server to work with

### Installing and usage

The project is using cargo project management system.
`cargo run` to start the server then visit `127.0.0.1:7522` in your browser dont ask why the port is `7522` XD

#### Design pattern
A server can handle multiple applications, an application will have its own routes and prefixes 

in order to keep to code clean we divide the applications and in their routes
here we have `main.rs` which contains the main code we import all of our applications there, then we will import them into `applications.rs` and initailize them

myapp module starting point can be defined as you want here we used `init` function, but no matter what the return type if this entry point should be a filled and ready to use App instance

in the `myapp` module we have our `mod.rs` which has the init function, and we assign `/` path to the sub mod named root which is the root of our website handler/controller.
now every request to that path will be given into the `myapp/root` module 

now if we want an `/about` page we will define it in `root/mod.rs` and we create the handler in `/src/root/about/mod.rs` and so on.

with this structure in mind we will maintain a simple and easy to develop application

#### What about database ?

That depends on your whether you want to use mysql/pgsql/redis/mongo

you can use `http://diesel.rs/guides/getting-started/` if you want an orm 

but keep in mind that we create controllers and handlers in their own folder so we can add these functions in there and only there, we can also have a shared module under the application containing our database functions which our handlers can import from `super` and use.

#### Notes

if you want to have a database then use a future return handler so the main thread wont be blocked. this way you can have somewhat of a non-blocking api!

#### Next milestone
config files
logging system
more routes and examples