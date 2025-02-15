# Gauge full-stack example

The front-end is done in Yew and the back-end is done in Rocket.

### To build/run

```command
# build the front end first
cd front-end
# trunk will generate dist for it's static pages
trunk serve
cd ../back-end
# link the back-end to the front-end's pages
ln -s ../front-end/dist
# build and run the back-end
cargo run
```

The page should be ready at http://127.0.0.1:8000

