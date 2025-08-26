# ClipBook - Build and Install Guide

This guide provides step-by-step instructions to build and install ClipBook from source.

## Prerequisites

- **macOS Sonoma 14 or later**
- **Node.js version 20.11.0 or higher** - [Download here](https://nodejs.org/en/download)
- **Xcode Command Line Tools** - Install with: `xcode-select --install`

## Step 1: Clone and Setup

```bash
# Clone the repository
git clone https://github.com/thuanpham582002/ClipBook.git
cd ClipBook

# Install dependencies
npm install
```

## Step 2: Build the Application

```bash
# Build ClipBook with Molybden
npm run molybden build
```

**Note**: This uses a free 3-week trial of Molybden. The build process will:
- Download CMake automatically
- Compile the C++ native components
- Build the React frontend
- Package everything into a native macOS app
- Create a DMG installer

## Step 3: Self-Sign for Local Use

Since the app is not signed with an Apple Developer certificate, you need to self-sign it:

```bash
# Sign the built app for local use
codesign --force --deep --sign - ./build-dist/bin/ClipBook.app
```

## Step 4: Install Options

### Option A: Install from DMG (Recommended)
1. Open the generated DMG: `open ./build-dist/pack/ClipBook-1.29.2-arm64.dmg`
2. Drag ClipBook.app to the Applications folder
3. Launch from Applications or Spotlight

### Option B: Run Directly from Build
```bash
# Launch directly from build directory
open ./build-dist/bin/ClipBook.app
```

## Build Output Locations

- **App Bundle**: `./build-dist/bin/ClipBook.app`
- **DMG Installer**: `./build-dist/pack/ClipBook-1.29.2-arm64.dmg`

## Troubleshooting

### First Launch Security Warning
If macOS shows a security warning on first launch:
1. Go to **System Preferences** → **Privacy & Security**
2. Find ClipBook in the security section
3. Click **"Open Anyway"**

### Build Errors
- Ensure you have the latest Xcode Command Line Tools
- Node.js version must be 20.11.0 or higher
- Clear npm cache if needed: `npm cache clean --force`

## Technical Notes

- **Framework**: Built with Molybden (Chromium-based desktop framework)
- **Frontend**: React + TypeScript + Vite
- **Backend**: C++ with Objective-C for macOS integration
- **Architecture**: Universal binary (ARM64/x86_64)
- **License**: Uses Molybden trial license (3 weeks free)

## What Was Fixed

The original codebase had a TypeScript error in `src/db.tsx`. The `updateClip` function parameter was changed from `Clip` to `Partial<Clip>` to match Dexie's UpdateSpec type requirements.

## License and Usage

This build is for personal use only. ClipBook remains a paid application - consider purchasing a license to support the developer if you use it regularly.