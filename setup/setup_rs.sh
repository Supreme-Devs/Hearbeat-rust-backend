# #!/bin/bash
# set -e

# echo "🔹 Setting up Rust backend..."

# # Go into the correct folder
# cd "$(dirname "$0")/../store"

# # Load env vars (needed for DATABASE_URL)
# if [ -f .env ]; then
#   export $(grep -v '^#' .env | xargs)
# fi

# # Wait for Postgres to be ready
# until pg_isready -h localhost -p 5432 -U postgres -d postgres > /dev/null 2>&1; do
#   echo "⏳ Waiting for Postgres to be ready..."
#   sleep 2
# done

# # Run Diesel migration
# diesel migration run

# echo "✅ Rust backend setup complete!"



#  For now run the diesel migration run manually inside the store folder
