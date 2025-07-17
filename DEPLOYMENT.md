# AWS S3 Deployment Guide for gsazure2

This guide will walk you through packaging and deploying your Dioxus web application to AWS S3 for static website hosting.

## Prerequisites

- AWS CLI installed and configured with your credentials
- Rust and Cargo installed
- Node.js and npm installed (for Tailwind CSS)
- An AWS account with S3 access

## Step 1: Build the Project

First, ensure all dependencies are installed and build the project for web deployment:

```bash
# Install npm dependencies (for Tailwind CSS)
npm install

# Build the Dioxus project for web
dx build --release --platform web
```

This will create optimized production files in the `dist` directory.

## Step 2: Create an S3 Bucket

Create a new S3 bucket for hosting your website:

```bash
# Replace 'your-bucket-name' with a unique bucket name
aws s3 mb s3://your-bucket-name --region us-east-1
```

## Step 3: Configure Bucket for Static Website Hosting

Enable static website hosting on your bucket:

```bash
# Enable static website hosting
aws s3 website s3://your-bucket-name/ \
  --index-document index.html \
  --error-document error.html
```

## Step 4: Set Bucket Policy for Public Access

Create a bucket policy to allow public read access. Save this as `bucket-policy.json`:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "PublicReadGetObject",
      "Effect": "Allow",
      "Principal": "*",
      "Action": "s3:GetObject",
      "Resource": "arn:aws:s3:::your-bucket-name/*"
    }
  ]
}
```

Apply the policy:

```bash
aws s3api put-bucket-policy \
  --bucket your-bucket-name \
  --policy file://bucket-policy.json
```

## Step 5: Deploy to S3

Upload your built files to S3:

```bash
# Sync the dist folder to S3
aws s3 sync dist/ s3://your-bucket-name/ \
  --delete \
  --exclude ".git/*" \
  --exclude "*.map"
```

## Step 6: Access Your Website

Your website will be available at:
```
http://your-bucket-name.s3-website-us-east-1.amazonaws.com
```

## Automated Deployment Script

Create a `deploy.sh` script to automate the deployment process:

```bash
#!/bin/bash

# Configuration
BUCKET_NAME="your-bucket-name"
REGION="us-east-1"

# Build the project
echo "Building Dioxus project..."
npm install
dx build --release --platform web

# Check if build was successful
if [ ! -d "dist" ]; then
    echo "Build failed: dist directory not found"
    exit 1
fi

# Deploy to S3
echo "Deploying to S3..."
aws s3 sync dist/ s3://$BUCKET_NAME/ \
  --region $REGION \
  --delete \
  --exclude ".git/*" \
  --exclude "*.map" \
  --cache-control "public, max-age=3600"

# Set proper content types
aws s3 cp s3://$BUCKET_NAME/ s3://$BUCKET_NAME/ \
  --exclude "*" \
  --include "*.js" \
  --recursive \
  --metadata-directive REPLACE \
  --content-type "application/javascript" \
  --cache-control "public, max-age=31536000"

aws s3 cp s3://$BUCKET_NAME/ s3://$BUCKET_NAME/ \
  --exclude "*" \
  --include "*.wasm" \
  --recursive \
  --metadata-directive REPLACE \
  --content-type "application/wasm" \
  --cache-control "public, max-age=31536000"

echo "Deployment complete!"
echo "Website URL: http://$BUCKET_NAME.s3-website-$REGION.amazonaws.com"
```

Make the script executable:
```bash
chmod +x deploy.sh
```

## Optional: CloudFront Distribution

For better performance and HTTPS support, set up CloudFront:

1. Create a CloudFront distribution:
```bash
aws cloudfront create-distribution \
  --origin-domain-name your-bucket-name.s3.amazonaws.com \
  --default-root-object index.html
```

2. Configure the distribution to use your S3 bucket as the origin.

3. Update your deployment script to invalidate CloudFront cache:
```bash
# Add to deploy.sh after S3 sync
DISTRIBUTION_ID="your-distribution-id"
aws cloudfront create-invalidation \
  --distribution-id $DISTRIBUTION_ID \
  --paths "/*"
```

## GitHub Actions Deployment

For automated deployments, create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to S3

on:
  push:
    branches: [ main ]

env:
  AWS_REGION: us-east-1
  S3_BUCKET: your-bucket-name

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install Dioxus CLI
      run: cargo install dioxus-cli
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
    
    - name: Install dependencies
      run: npm install
    
    - name: Build project
      run: dx build --release --platform web
    
    - name: Configure AWS credentials
      uses: aws-actions/configure-aws-credentials@v2
      with:
        aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
        aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
        aws-region: ${{ env.AWS_REGION }}
    
    - name: Deploy to S3
      run: |
        aws s3 sync dist/ s3://${{ env.S3_BUCKET }}/ \
          --delete \
          --exclude ".git/*" \
          --exclude "*.map"
```

Remember to add your AWS credentials as GitHub secrets.

## Troubleshooting

### Common Issues:

1. **403 Forbidden Error**: Check bucket policy and ensure public access is enabled
2. **404 Not Found**: Verify index.html exists and static website hosting is configured
3. **CORS Issues**: Add CORS configuration to your S3 bucket if needed
4. **Large File Sizes**: Consider enabling gzip compression in CloudFront

### CORS Configuration (if needed):

```json
[
  {
    "AllowedHeaders": ["*"],
    "AllowedMethods": ["GET", "HEAD"],
    "AllowedOrigins": ["*"],
    "ExposeHeaders": []
  }
]
```

Apply CORS:
```bash
aws s3api put-bucket-cors \
  --bucket your-bucket-name \
  --cors-configuration file://cors.json
```
