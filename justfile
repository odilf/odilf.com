[parallel]
watch: serve watch-build

watch-build:
    bacon run

serve:
    env --chdir=target/debug/site -S live-server --port 5173 --index

deploy:
    cargo run --release && wrangler pages deploy target/release/site/ --project-name "odilf-site" --branch main
