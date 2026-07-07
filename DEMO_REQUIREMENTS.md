# Demo Requirements — Smart File Organizer Product Demonstration

This document lists every file, property, and precondition needed to execute the product demonstration script. Follow the setup instructions before the defense.

---

## Directory Structure Before Setup

```
~/
├── Pictures/        (or create if missing)
├── Documents/       (or create if missing)
├── Videos/          (or create if missing)
├── Music/           (or create if missing)
├── Projects/        (or create if missing)
├── misc/            (create this directory)
├── Downloads/       (will be populated in Step 2)
└── .local/
    └── share/
        └── smart-file-organizer/
            └── organizer.log     (delete before demo, or pre-populate)
```

---

## Step 1: Create Source Directories

```bash
mkdir -p ~/Pictures
mkdir -p ~/Documents
mkdir -p ~/Videos
mkdir -p ~/Music
mkdir -p ~/Projects
mkdir -p ~/misc
mkdir -p ~/Downloads
```

---

## Step 2: Initial Messy Downloads Folder

These files must exist in `~/Downloads/` **before** the demo begins. They simulate a week of normal download activity. The organizer must NOT be running yet — these files are the "before" picture.

| #  | Filename                          | Ext    | Category | Size (bytes) | Notes                            |
|----|-----------------------------------|--------|----------|--------------|----------------------------------|
| 1  | `operating_systems_concepts.pdf`  | .pdf   | Documents | 2,400,000   | Mock academic paper              |
| 2  | `screenshot_2026.png`             | .png   | Images   | 512,000      | Screenshot, 1920x1080 mock       |
| 3  | `lecture_recording.mp4`           | .mp4   | Videos   | 15,000,000   | Mock video file                  |
| 4  | `project_archive.zip`             | .zip   | Archives | 3,200,000    | Mock compressed archive          |
| 5  | `data_export.csv`                 | .csv   | Documents | 85,000       | CSV spreadsheet                  |
| 6  | `script.py`                       | .py    | Code     | 1,200        | Python source file               |
| 7  | `ubuntu_image.iso`                | .iso   | Archives | —            | Large file, skip if low on space |
| 8  | `open_sans.ttf`                   | .ttf   | Fonts    | 220,000      | Font file                        |
| 9  | `README.md`                       | .md    | Documents | 450          | Markdown readme                  |

### Creation Commands

```bash
# Create files with realistic sizes in ~/Downloads/

# PDF — academic paper (use fallocate or dd)
fallocate -l 2400000 ~/Downloads/operating_systems_concepts.pdf 2>/dev/null || \
  dd if=/dev/urandom of=~/Downloads/operating_systems_concepts.pdf bs=1024 count=2344 2>/dev/null

# PNG — screenshot
fallocate -l 512000 ~/Downloads/screenshot_2026.png 2>/dev/null || \
  dd if=/dev/urandom of=~/Downloads/screenshot_2026.png bs=1024 count=500 2>/dev/null

# MP4 — video lecture
fallocate -l 15000000 ~/Downloads/lecture_recording.mp4 2>/dev/null || \
  dd if=/dev/urandom of=~/Downloads/lecture_recording.mp4 bs=1024 count=14649 2>/dev/null

# ZIP — project archive
fallocate -l 3200000 ~/Downloads/project_archive.zip 2>/dev/null || \
  dd if=/dev/urandom of=~/Downloads/project_archive.zip bs=1024 count=3125 2>/dev/null

# CSV — spreadsheet
fallocate -l 85000 ~/Downloads/data_export.csv 2>/dev/null || \
  dd if=/dev/urandom of=~/Downloads/data_export.csv bs=1024 count=83 2>/dev/null

# PY — Python script
echo '#!/usr/bin/env python3
"""Utility script for data processing."""
import csv, sys

def process(filepath):
    with open(filepath) as f:
        reader = csv.DictReader(f)
        return [row for row in reader]

if __name__ == "__main__":
    process(sys.argv[1])' > ~/Downloads/script.py

# ISO — disk image (optional, skip if storage is tight)
# fallocate -l 700000000 ~/Downloads/ubuntu_image.iso 2>/dev/null

# TTF — font file
fallocate -l 220000 ~/Downloads/open_sans.ttf 2>/dev/null || \
  dd if=/dev/urandom of=~/Downloads/open_sans.ttf bs=1024 count=215 2>/dev/null

# MD — markdown readme
echo '# Smart File Organizer
A file organizer for Linux built on Rust.

## Usage
```bash
./smart-file-organizer --mode rule
```' > ~/Downloads/README.md
```

---

## Step 3: Source Files (Copied INTO Downloads During Demo)

These files live outside `~/Downloads/` and are copied in during the script to trigger inotify events.

| #  | Source Path                    | Ext    | Target Category | Used In                           |
|----|--------------------------------|--------|-----------------|-----------------------------------|
| 1  | `~/Pictures/test_logo.png`     | .png   | Images          | First trigger (Rule mode)         |
| 2  | `~/Documents/report.pdf`       | .pdf   | Documents       | Batch + Conflict resolution (x3)  |
| 3  | `~/Videos/lecture.mp4`         | .mp4   | Videos          | Batch processing                  |
| 4  | `~/Projects/archive.zip`       | .zip   | Archives        | Batch processing                  |
| 5  | `~/Music/song.mp3`             | .mp3   | Music           | Batch processing                  |
| 6  | `~/misc/obscure_format.xyz`    | .xyz   | Others          | Catch-all demo                    |
| 7  | `~/Documents/notes.txt`        | .txt   | 2026/07         | Date mode (mtime: July 2026)      |
| 8  | `~/Documents/old_report.pdf`   | .pdf   | 2024/03         | Date mode (mtime: March 2024)     |

### Creation Commands

```bash
# 1. test_logo.png (PNG image)
fallocate -l 150000 ~/Pictures/test_logo.png 2>/dev/null || \
  dd if=/dev/urandom of=~/Pictures/test_logo.png bs=1024 count=147 2>/dev/null

# 2. report.pdf (used in batch + conflict resolution — copied 3 times)
fallocate -l 500000 ~/Documents/report.pdf 2>/dev/null || \
  dd if=/dev/urandom of=~/Documents/report.pdf bs=1024 count=489 2>/dev/null

# 3. lecture.mp4 (video file)
fallocate -l 25000000 ~/Videos/lecture.mp4 2>/dev/null || \
  dd if=/dev/urandom of=~/Videos/lecture.mp4 bs=1024 count=24414 2>/dev/null

# 4. archive.zip (compressed archive)
fallocate -l 1800000 ~/Projects/archive.zip 2>/dev/null || \
  dd if=/dev/urandom of=~/Projects/archive.zip bs=1024 count=1758 2>/dev/null

# 5. song.mp3 (audio file)
fallocate -l 5000000 ~/Music/song.mp3 2>/dev/null || \
  dd if=/dev/urandom of=~/Music/song.mp3 bs=1024 count=4883 2>/dev/null

# 6. obscure_format.xyz (unknown extension → Others)
echo "This is an obscure file format." > ~/misc/obscure_format.xyz

# 7. notes.txt (modified: July 2026)
echo "Meeting notes: discussed architecture and deployment strategy." > ~/Documents/notes.txt
touch -t 202607071200 ~/Documents/notes.txt

# 8. old_report.pdf (modified: March 2024)
fallocate -l 350000 ~/Documents/old_report.pdf 2>/dev/null || \
  dd if=/dev/urandom of=~/Documents/old_report.pdf bs=1024 count=342 2>/dev/null
touch -t 202403151000 ~/Documents/old_report.pdf
```

---

## Step 4: Pre-populate the Organizer Log (Optional)

If you want `tail -10` to show something meaningful instead of an empty file:

```bash
mkdir -p ~/.local/share/smart-file-organizer
cat > ~/.local/share/smart-file-organizer/organizer.log << 'LOGEOF'
[2026-07-01 09:12:34] MOVED: /home/demo/Downloads/photo.jpg -> /home/demo/Downloads/Images/photo.jpg
[2026-07-02 14:45:10] MOVED: /home/demo/Downloads/resume.pdf -> /home/demo/Downloads/Documents/resume.pdf
[2026-07-03 11:20:05] MOVED: /home/demo/Downloads/setup.msi -> /home/demo/Downloads/Programs/setup.msi
[2026-07-04 16:33:22] MOVED: /home/demo/Downloads/track.flac -> /home/demo/Downloads/Music/track.flac
[2026-07-05 08:55:47] MOVED: /home/demo/Downloads/config.yaml -> /home/demo/Downloads/Code/config.yaml
[2026-07-06 12:10:18] MOVED: /home/demo/Downloads/presentation.pptx -> /home/demo/Downloads/Documents/presentation.pptx
LOGEOF
```

Replace `/home/demo` with the actual demo user's home path.

---

## Step 5: Pre-organize Files to Simulate a "Used" State (Optional)

If you want the initial `tree` to show the Downloads folder already partially organized (for visual impact of the "before" state), skip this step. If you want to show the organizer already has folders from a prior run, create these directories with a few files:

```bash
# Create category folders with a few pre-existing files (optional)
mkdir -p ~/Downloads/{Images,Documents,Videos,Music,Archives,Code,Programs,Fonts,Others}

# Drop a few pre-sorted files to show the organizer has been running
echo "placeholder" > ~/Downloads/Images/wallpaper.jpg
echo "placeholder" > ~/Downloads/Documents/invoice.pdf
```

**Note:** The script assumes a genuinely messy flat folder for the opening. Skip this step unless you want to show cumulative results.

---

## Step 6: Clean the Log Before Demo

```bash
# Option A: Delete entirely (the organizer will re-create it)
rm -f ~/.local/share/smart-file-organizer/organizer.log

# Option B: Keep only pre-populated entries (from Step 4) and let the organizer append
# (do nothing — the file opens in append mode)
```

---

## Step 7: Verify the Setup

Run this verification script before the defense to ensure everything is in place:

```bash
#!/bin/bash
echo "=== Checking source files ==="
for f in \
  ~/Pictures/test_logo.png \
  ~/Documents/report.pdf \
  ~/Documents/notes.txt \
  ~/Documents/old_report.pdf \
  ~/Videos/lecture.mp4 \
  ~/Projects/archive.zip \
  ~/Music/song.mp3 \
  ~/misc/obscure_format.xyz
do
  if [ -f "$f" ]; then
    size=$(stat --printf="%s" "$f" 2>/dev/null || stat -f%z "$f" 2>/dev/null || echo "?")
    mtime=$(stat --printf="%y" "$f" 2>/dev/null || stat -f%Sm "$f" 2>/dev/null || echo "?")
    echo "  [OK] $f  (size: $size, mtime: $mtime)"
  else
    echo "  [MISSING] $f"
  fi
done

echo ""
echo "=== Checking initial Downloads folder ==="
for f in \
  ~/Downloads/operating_systems_concepts.pdf \
  ~/Downloads/screenshot_2026.png \
  ~/Downloads/lecture_recording.mp4 \
  ~/Downloads/project_archive.zip \
  ~/Downloads/data_export.csv \
  ~/Downloads/script.py \
  ~/Downloads/open_sans.ttf \
  ~/Downloads/README.md
do
  if [ -f "$f" ]; then
    echo "  [OK] $f"
  else
    echo "  [MISSING] $f"
  fi
done

echo ""
echo "=== Checking specific mtime for date-sort demo ==="
echo "notes.txt should show: July 2026"
ls -la ~/Documents/notes.txt 2>/dev/null || echo "  [MISSING]"
echo "old_report.pdf should show: March 2024"
ls -la ~/Documents/old_report.pdf 2>/dev/null || echo "  [MISSING]"

echo ""
echo "=== Checking binary ==="
if [ -f "./smart-file-organizer" ] || [ -f "./target/release/smart-file-organizer" ]; then
  echo "  [OK] Binary found"
else
  echo "  [WARN] Binary not found in current directory or target/release/"
  echo "  Build with: cargo build --release"
fi

echo ""
echo "Verification complete."
```

---

## Quick Setup Summary

Run this from the project root to set up the entire demo environment:

```bash
# 1. Create source directories
mkdir -p ~/Pictures ~/Documents ~/Videos ~/Music ~/Projects ~/misc ~/Downloads

# 2. Create messy Downloads files
dd if=/dev/urandom of=~/Downloads/operating_systems_concepts.pdf bs=1024 count=2344 2>/dev/null
dd if=/dev/urandom of=~/Downloads/screenshot_2026.png bs=1024 count=500 2>/dev/null
dd if=/dev/urandom of=~/Downloads/lecture_recording.mp4 bs=1024 count=14649 2>/dev/null
dd if=/dev/urandom of=~/Downloads/project_archive.zip bs=1024 count=3125 2>/dev/null
dd if=/dev/urandom of=~/Downloads/data_export.csv bs=1024 count=83 2>/dev/null
echo '#!/usr/bin/env python3
print("hello")' > ~/Downloads/script.py
dd if=/dev/urandom of=~/Downloads/open_sans.ttf bs=1024 count=215 2>/dev/null
echo '# README' > ~/Downloads/README.md

# 3. Create source files for copy triggers
dd if=/dev/urandom of=~/Pictures/test_logo.png bs=1024 count=147 2>/dev/null
dd if=/dev/urandom of=~/Documents/report.pdf bs=1024 count=489 2>/dev/null
dd if=/dev/urandom of=~/Videos/lecture.mp4 bs=1024 count=24414 2>/dev/null
dd if=/dev/urandom of=~/Projects/archive.zip bs=1024 count=1758 2>/dev/null
dd if=/dev/urandom of=~/Music/song.mp3 bs=1024 count=4883 2>/dev/null
echo "obscure" > ~/misc/obscure_format.xyz

# 4. Date-sort files with specific mtimes
echo "Meeting notes: July 2026" > ~/Documents/notes.txt
touch -t 202607071200 ~/Documents/notes.txt
dd if=/dev/urandom of=~/Documents/old_report.pdf bs=1024 count=342 2>/dev/null
touch -t 202403151000 ~/Documents/old_report.pdf

# 5. Clear previous log
rm -f ~/.local/share/smart-file-organizer/organizer.log

# 6. Build the binary
cargo build --release

echo "Demo environment ready."
```

---

## File Inventory Summary

| Location                | Count | Purpose                            |
|-------------------------|-------|------------------------------------|
| ~/Downloads/ (initial)  | 8     | Messy starting state               |
| ~/Pictures/             | 1     | Rule-mode trigger (PNG)            |
| ~/Documents/            | 3     | Batch, date-sort, conflict demo    |
| ~/Videos/               | 1     | Batch processing                   |
| ~/Projects/             | 1     | Batch processing                   |
| ~/Music/                | 1     | Batch processing                   |
| ~/misc/                 | 1     | Catch-all (Others) demo            |
| **Total files**         | **15**|                                    |

---

## mtime Reference for Date-Sort Demo

| File                    | mtime           | Expected Destination      |
|-------------------------|-----------------|---------------------------|
| `notes.txt`             | 2026-07-07      | `~/Downloads/2026/07/`    |
| `old_report.pdf`        | 2024-03-15      | `~/Downloads/2024/03/`    |

Verify these with `stat ~/Documents/notes.txt` and `stat ~/Documents/old_report.pdf` before the demo.
