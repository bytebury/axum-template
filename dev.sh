# Check for the .env file; otherwise create one
if [ ! -f .env ]; then
    echo ".env not found. Generating..."
    cat > .env <<EOF
DATABASE_URL="sqlite://database.db"
GOOGLE_CLIENT_ID="ADD_YOUR_CLIENT_ID"
GOOGLE_CLIENT_SECRET="ADD_YOUR_SECRET"
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


# Run cargo in watch mode
cargo watch -x run