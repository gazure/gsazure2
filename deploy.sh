#!/bin/bash

# AWS S3 Deployment Script for gsazure2
# =====================================

# Configuration - Update these values
BUCKET_NAME="grantazure.com"
REGION="us-east-1"
PROFILE="default"  # AWS CLI profile to use

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

print_error() {
    echo -e "${RED}[$(date +'%Y-%m-%d %H:%M:%S')] ERROR:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[$(date +'%Y-%m-%d %H:%M:%S')] WARNING:${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_status "Checking dependencies..."

    if ! command -v dx &> /dev/null; then
        print_error "Dioxus CLI (dx) is not installed. Install with: cargo install dioxus-cli"
        exit 1
    fi

    if ! command -v aws &> /dev/null; then
        print_error "AWS CLI is not installed. Please install it first."
        exit 1
    fi

    if ! command -v npm &> /dev/null; then
        print_error "npm is not installed. Please install Node.js and npm."
        exit 1
    fi
}

# Build the project
build_project() {
    print_status "Installing npm dependencies..."
    npm install
    if [ $? -ne 0 ]; then
        print_error "npm install failed"
        exit 1
    fi

    print_status "Building Dioxus project for web..."
    dx build --release --platform web
    if [ $? -ne 0 ]; then
        print_error "Dioxus build failed"
        exit 1
    fi

    # Check if dist directory was created
    if [ ! -d "dist" ]; then
        print_error "Build failed: dist directory not found"
        exit 1
    fi

    print_status "Build completed successfully"
}

# Deploy to S3
deploy_to_s3() {
    print_status "Starting deployment to S3..."

    # Check if bucket exists
    if ! aws s3 ls "s3://${BUCKET_NAME}" --profile "${PROFILE}" 2>&1 | grep -q 'NoSuchBucket'; then
        print_status "Bucket ${BUCKET_NAME} exists"
    else
        print_warning "Bucket ${BUCKET_NAME} does not exist. Creating it..."
        aws s3 mb "s3://${BUCKET_NAME}" --region "${REGION}" --profile "${PROFILE}"
        if [ $? -ne 0 ]; then
            print_error "Failed to create bucket"
            exit 1
        fi
    fi

    # Sync files to S3
    print_status "Syncing files to S3..."
    aws s3 sync dist/ "s3://${BUCKET_NAME}/" \
        --region "${REGION}" \
        --profile "${PROFILE}" \
        --delete \
        --exclude ".git/*" \
        --exclude "*.map" \
        --exclude ".DS_Store" \
        --cache-control "public, max-age=3600"

    if [ $? -ne 0 ]; then
        print_error "S3 sync failed"
        exit 1
    fi

    # Set proper content types for specific file types
    print_status "Setting content types..."

    # JavaScript files
    aws s3 cp "s3://${BUCKET_NAME}/" "s3://${BUCKET_NAME}/" \
        --exclude "*" \
        --include "*.js" \
        --recursive \
        --region "${REGION}" \
        --profile "${PROFILE}" \
        --metadata-directive REPLACE \
        --content-type "application/javascript" \
        --cache-control "public, max-age=31536000"

    # WebAssembly files
    aws s3 cp "s3://${BUCKET_NAME}/" "s3://${BUCKET_NAME}/" \
        --exclude "*" \
        --include "*.wasm" \
        --recursive \
        --region "${REGION}" \
        --profile "${PROFILE}" \
        --metadata-directive REPLACE \
        --content-type "application/wasm" \
        --cache-control "public, max-age=31536000"

    # CSS files
    aws s3 cp "s3://${BUCKET_NAME}/" "s3://${BUCKET_NAME}/" \
        --exclude "*" \
        --include "*.css" \
        --recursive \
        --region "${REGION}" \
        --profile "${PROFILE}" \
        --metadata-directive REPLACE \
        --content-type "text/css" \
        --cache-control "public, max-age=3600"

    print_status "Content types set successfully"
}

# Configure bucket for static website hosting
configure_website() {
    print_status "Configuring bucket for static website hosting..."

    aws s3 website "s3://${BUCKET_NAME}/" \
        --index-document index.html \
        --error-document error.html \
        --region "${REGION}" \
        --profile "${PROFILE}"

    if [ $? -eq 0 ]; then
        print_status "Static website hosting configured"
    else
        print_error "Failed to configure static website hosting"
        exit 1
    fi
}

# Main execution
main() {
    echo "======================================"
    echo "AWS S3 Deployment Script for gsazure2"
    echo "======================================"
    echo ""

    # Check configuration
    if [ "${BUCKET_NAME}" = "your-bucket-name-here" ]; then
        print_error "Please update BUCKET_NAME in this script before running"
        exit 1
    fi

    check_dependencies
    build_project
    deploy_to_s3
    configure_website

    echo ""
    print_status "Deployment completed successfully!"
    echo ""
    echo "Website URL: http://${BUCKET_NAME}.s3-website-${REGION}.amazonaws.com"
    echo ""
    echo "Note: If this is your first deployment, you may need to:"
    echo "  1. Set the bucket policy for public access (see bucket-policy.json)"
    echo "  2. Wait a few minutes for DNS propagation"
    echo ""
}

# Run main function
main
