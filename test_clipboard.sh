#!/bin/bash

# Test script for clipboard functionality

echo "Testing clipboard monitoring..."

# Clean up old data
rm -rf ~/.local/share/clipstack/

# Start the application in background
cargo run > test_output.log 2>&1 &
APP_PID=$!

# Wait for app to start and create database
sleep 5

echo "Copying first item..."
echo "First test item" | xclip -selection clipboard
sleep 2

echo "Copying second item..."
echo "Second test item" | xclip -selection clipboard
sleep 2

echo "Copying third item..."
echo "Third test item" | xclip -selection clipboard
sleep 3

# Check if items were captured
echo "Checking database..."
if [ -f ~/.local/share/clipstack/history.db ]; then
    sqlite3 ~/.local/share/clipstack/history.db "SELECT content FROM clipboard_items ORDER BY timestamp DESC LIMIT 5;"
else
    echo "Database file not found!"
fi

# Kill the app
kill $APP_PID 2>/dev/null
wait $APP_PID 2>/dev/null

echo "Test complete. Check test_output.log for details."
echo "Runtime logs:"
cat test_output.log | grep -E "(INFO|Added|clipboard)" | tail -10
