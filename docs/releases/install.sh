#!/bin/bash

# Function to check if curl is installed
check_curl() {
  if ! command -v curl &> /dev/null; then
    echo "Error: 'curl' is not installed. Aborting."
    exit 1
  fi
}

# Function to detect the operating system
detect_os() {
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="Linux"
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="Mac"
  else
    echo "Error: Unsupported operating system. Aborting."
    exit 1
  fi
}

# Function to download and save the file based on the operating system
download_file() {
  local download_url

  if [ "$OS" == "Linux" ]; then
    download_url="https://github.com/edvm/GPTi/raw/main/docs/releases/linux/gpti"
  elif [ "$OS" == "Mac" ]; then
    download_url="https://github.com/edvm/GPTi/raw/main/docs/releases/mac-arm/gpti"
  fi

  echo "Downloading file from $download_url..."
  curl -LJO "$download_url" -o "$save_directory/gpti"

  if [ $? -ne 0 ]; then
    echo "Error: Failed to download the file. Aborting."
    exit 1
  fi
}

# Function to set execute permissions on the downloaded file
set_execute_permissions() {
  chmod +x "$save_directory/gpti"
}

# Function to print success message and exit
print_success() {
  echo "File successfully saved at: $save_directory/gpti"
  exit 0
}

# Main script
check_curl
detect_os

# Ask user for the save directory
read -p "Enter the directory where you want to save the file: " save_directory

# Ensure the save directory exists
mkdir -p "$save_directory" || { echo "Error: Unable to create directory. Aborting."; exit 1; }

download_file
set_execute_permissions
print_success
