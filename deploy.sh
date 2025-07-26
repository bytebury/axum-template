docker build -t axum_template .

docker rm -f axum_template_container || true
docker container prune -f

docker run -d \
    -p 8080:8080 \
    -v $(pwd)/database.db:/app/database.db \
    -v $(pwd)/.env:/app/.env \
    --name axum_template_container axum_template
