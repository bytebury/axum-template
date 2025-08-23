npm i

# Check for the .env file; otherwise create one
if [ ! -f .env ]; then
    echo ".env not found. Generating..."
    cat > .env <<EOF
APP_NAME="axum-template"
APP_DISPLAY_NAME="Axum Template"
APP_PORT="8080"

DATABASE_URL="sqlite://database.db"

JWT_SECRET="SOMETHING-TOP-SECRET"

GOOGLE_ANALYTICS_ID="ADD_YOUR_ANALYTICS_ID"
GOOGLE_CLIENT_ID="ADD_YOUR_CLIENT_ID"
GOOGLE_CLIENT_SECRET="ADD_YOUR_SECRET"
GOOGLE_CALLBACK_URL="http://localhost:8080/auth/google/callback"

WEBSITE_URL="http://localhost:8080" # or https://yourdomain.com
COOKIE_URL="localhost:8080" # or .yourdomain.com

STRIPE_SECRET="ADD_YOUR_STRIPE_SECRET"
STRIPE_WEBHOOK_SECRET="ADD_YOUR_STRIPE_WEBHOOK_SECRET"
STRIPE_PRICE_ID="ADD_YOUR_STRIPE_PRICE_ID"
EOF
    echo ".env generated."
else
    echo ".env file found."
fi

# Check for the database file if it does not exist
if [ ! -f database.db ]; then
    echo "database not found. Generating..."
    touch database.db
    echo "database.db generated."
else
    echo "database file found."
fi


# Run cargo and TailwindCSS in watch mode
npx concurrently "cargo watch -x run" "npx tailwindcss -i './public/styles/tailwind.css' -o './public/styles/main-0.min.css' --watch"