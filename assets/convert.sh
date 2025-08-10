#!/bin/bash

# Image Aspect Ratio Converter (1:1 to 16:9) using ImageMagick
# Converts all images in a directory to 16:9 aspect ratio by center-cropping

# Default values
INPUT_DIR=""
OUTPUT_DIR=""
RECURSIVE=false
QUALITY=95
VERBOSE=false

# Supported image extensions
EXTENSIONS=("jpg" "jpeg" "png" "bmp" "tiff" "tif" "webp" "gif")

# Function to display usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS] INPUT_DIRECTORY

Convert all images in a directory from any aspect ratio to 16:9 by center-cropping.

OPTIONS:
    -o, --output DIR        Output directory (default: overwrite originals)
    -r, --recursive         Process subdirectories recursively
    -q, --quality NUM       JPEG quality 1-100 (default: 95)
    -v, --verbose           Show detailed output
    -h, --help             Show this help message

EXAMPLES:
    $0 ./images                                    # Crop all images in place
    $0 ./images -o ./cropped                       # Save to different directory
    $0 ./images -r -v                             # Recursive with verbose output
    $0 ./images -o ./output -q 90                 # Custom quality

REQUIREMENTS:
    ImageMagick must be installed (convert and identify commands)

WARNING:
    This script CROPS images to 16:9 ratio. Parts of the original image will be removed.
    The cropping is center-based, so edges will be trimmed equally.
EOF
}

# Function to check if ImageMagick is installed
check_imagemagick() {
    if ! command -v convert &> /dev/null || ! command -v identify &> /dev/null; then
        echo "Error: ImageMagick is not installed or not in PATH"
        echo "Install with: sudo apt install imagemagick  # or brew install imagemagick"
        exit 1
    fi
}

# Function to check if file is a supported image
is_supported_image() {
    local file="$1"
    local ext="${file##*.}"
    ext=$(echo "$ext" | tr '[:upper:]' '[:lower:]')

    for supported_ext in "${EXTENSIONS[@]}"; do
        if [[ "$ext" == "$supported_ext" ]]; then
            return 0
        fi
    done
    return 1
}

# Function to convert a single image
convert_image() {
    local input_file="$1"
    local output_file="$2"
    local filename=$(basename "$input_file")

    # Get original dimensions
    local dimensions=$(identify -format "%wx%h" "$input_file" 2>/dev/null)
    if [[ $? -ne 0 ]]; then
        echo "✗ Error reading: $filename"
        return 1
    fi

    local width=$(echo "$dimensions" | cut -d'x' -f1)
    local height=$(echo "$dimensions" | cut -d'x' -f2)

    # Calculate 16:9 target dimensions
    # Use the smaller dimension as reference to crop (not pad)
    local target_width target_height
    if (( width * 9 > height * 16 )); then
        # Height is the limiting factor - crop width
        target_height=$height
        target_width=$((height * 16 / 9))
    else
        # Width is the limiting factor - crop height
        target_width=$width
        target_height=$((width * 9 / 16))
    fi

    # Skip if already 16:9 (within 1 pixel tolerance)
    local current_ratio=$((width * 9))
    local target_ratio=$((height * 16))
    if (( current_ratio >= target_ratio - 16 && current_ratio <= target_ratio + 16 )); then
        if [[ "$input_file" != "$output_file" ]]; then
            cp "$input_file" "$output_file"
        fi
        [[ "$VERBOSE" == true ]] && echo "⊘ Skipped (already 16:9): $filename"
        return 0
    fi

    # Create the convert command for cropping
    local convert_cmd=(
        convert "$input_file"
        -gravity center
        -crop "${target_width}x${target_height}+0+0"
        +repage
    )

    # Add quality setting for JPEG files
    if [[ "${output_file,,}" =~ \.(jpg|jpeg)$ ]]; then
        convert_cmd+=(-quality "$QUALITY")
    fi

    convert_cmd+=("$output_file")

    # Execute conversion
    if "${convert_cmd[@]}" 2>/dev/null; then
        echo "✓ Cropped: $filename (${width}x${height} → ${target_width}x${target_height})"
        return 0
    else
        echo "✗ Error converting: $filename"
        return 1
    fi
}

# Function to process directory
process_directory() {
    local dir="$1"
    local output_base="$2"
    local processed=0
    local failed=0
    local skipped=0

    # Find command based on recursive option
    local find_cmd="find \"$dir\""
    if [[ "$RECURSIVE" == false ]]; then
        find_cmd="find \"$dir\" -maxdepth 1"
    fi

    # Process all files
    while IFS= read -r -d '' file; do
        # Skip directories
        [[ -d "$file" ]] && continue

        # Check if it's a supported image
        if ! is_supported_image "$file"; then
            [[ "$VERBOSE" == true ]] && echo "⊘ Skipped (unsupported): $(basename "$file")"
            ((skipped++))
            continue
        fi

        # Determine output file path
        local output_file
        if [[ -n "$output_base" ]]; then
            # Create output directory structure
            local rel_path="${file#"$dir"}"
            rel_path="${rel_path#/}"  # Remove leading slash
            output_file="$output_base/$rel_path"
            local output_dir=$(dirname "$output_file")
            mkdir -p "$output_dir"
        else
            output_file="$file"
        fi

        # Convert the image
        if convert_image "$file" "$output_file"; then
            ((processed++))
        else
            ((failed++))
        fi

    done < <(eval "$find_cmd -type f -print0")

    # Print summary
    echo
    echo "Processing complete:"
    echo "  Successfully cropped: $processed images"
    echo "  Failed: $failed images"
    [[ "$VERBOSE" == true ]] && echo "  Skipped (unsupported): $skipped files"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -r|--recursive)
            RECURSIVE=true
            shift
            ;;
        -c|--color)
            echo "Warning: Background color option removed in crop mode"
            echo "Use the padding version if you need background colors"
            shift 2
            ;;
        -q|--quality)
            if [[ "$2" =~ ^[0-9]+$ ]] && (( $2 >= 1 && $2 <= 100 )); then
                QUALITY="$2"
            else
                echo "Error: Quality must be a number between 1 and 100"
                exit 1
            fi
            shift 2
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo "Error: Unknown option $1"
            usage
            exit 1
            ;;
        *)
            if [[ -z "$INPUT_DIR" ]]; then
                INPUT_DIR="$1"
            else
                echo "Error: Multiple input directories specified"
                usage
                exit 1
            fi
            shift
            ;;
    esac
done

# Validate arguments
if [[ -z "$INPUT_DIR" ]]; then
    echo "Error: Input directory is required"
    usage
    exit 1
fi

if [[ ! -d "$INPUT_DIR" ]]; then
    echo "Error: '$INPUT_DIR' is not a valid directory"
    exit 1
fi

# Check ImageMagick installation
check_imagemagick

# Create output directory if specified
if [[ -n "$OUTPUT_DIR" ]]; then
    mkdir -p "$OUTPUT_DIR"
    if [[ $? -ne 0 ]]; then
        echo "Error: Cannot create output directory '$OUTPUT_DIR'"
        exit 1
    fi
fi

# Display configuration
echo "Image Aspect Ratio Converter (Crop to 16:9)"
echo "==========================================="
echo "Input directory: $INPUT_DIR"
[[ -n "$OUTPUT_DIR" ]] && echo "Output directory: $OUTPUT_DIR" || echo "Mode: Overwriting original files"
echo "JPEG quality: $QUALITY"
echo "Recursive: $([[ "$RECURSIVE" == true ]] && echo "Yes" || echo "No")"
echo "Verbose: $([[ "$VERBOSE" == true ]] && echo "Yes" || echo "No")"
echo "WARNING: Images will be CROPPED (parts removed) to fit 16:9 ratio"
echo "------------------------------------------"

# Process the directory
process_directory "$INPUT_DIR" "$OUTPUT_DIR"
