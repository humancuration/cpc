import os
import datetime

# Configuration
OUTPUT_FILENAME = "combined_project_cpc.txt"
# Common text file extensions
TEXT_FILE_EXTENSIONS = {
".svelte", ".js", ".scss", ".rs", ".toml", ".html", ".proto", ".kt", ".kts", ".swift"
}
# Files and directories to ignore
IGNORE_PATTERNS = {
    ".git", # Git directory
    # The output file itself

OUTPUT_FILENAME,
}
# Specific files to always ignore by full name (relative to project root)
IGNORE_FILES_BY_NAME = {
    "icon.svg",
}

# Ensure this script itself is ignored if it's in the scanned path
# This will be resolved to an absolute path later
IGNORE_FILES_BY_NAME.add(os.path.basename(__file__))


def should_ignore(path, root_dir):
    """Checks if a file or directory should be ignored."""
    relative_path = os.path.relpath(path, root_dir)
    normalized_path = relative_path.replace("\\", "/") + ("/" if os.path.isdir(path) else "")

    if os.path.basename(path) in IGNORE_FILES_BY_NAME:
        if os.path.basename(path) == OUTPUT_FILENAME and normalized_path == OUTPUT_FILENAME: # ensure it's the root output file
             return True
        elif os.path.basename(path) != OUTPUT_FILENAME: # other specifically named files
             return True


    for pattern in IGNORE_PATTERNS:
        # Ensure directory patterns end with a slash for proper matching
        pattern_to_match = pattern if pattern.endswith('/') else pattern
        if normalized_path.startswith(pattern_to_match):
            return True
        # Also check for exact filename matches within IGNORE_PATTERNS (like package-lock.json)
        if os.path.basename(path) == pattern and not pattern.endswith('/'):
             return True
    return False

def is_text_file(filename):
    """Checks if a file is likely a text file based on its extension."""
    return os.path.splitext(filename)[1].lower() in TEXT_FILE_EXTENSIONS

def main():
    project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    output_file_path = os.path.join(project_root, OUTPUT_FILENAME)

    # Add the script itself to ignore list using its absolute path context if needed
    # For now, basename matching in should_ignore should handle it.

    print(f"Project root: {project_root}")
    print(f"Outputting to: {output_file_path}")

    combined_content = []
    file_count = 0

    combined_content.append(f"--- Combined Project Text ---")
    combined_content.append(f"--- Generated on: {datetime.datetime.now().isoformat()} ---")
    combined_content.append(f"--- Project Root: {project_root} ---")
    combined_content.append("\n--- Note: This file combines various text-based source files from the project. ---")
    combined_content.append("--- Binary files, specific assets, and configured ignore patterns are excluded. ---")
    combined_content.append("-" * 80)

    for root, dirs, files in os.walk(project_root, topdown=True):
        # Filter directories to ignore
        dirs[:] = [d for d in dirs if not should_ignore(os.path.join(root, d), project_root)]

        for file in files:
            file_path = os.path.join(root, file)
            relative_file_path = os.path.relpath(file_path, project_root)

            if should_ignore(file_path, project_root):
                print(f"Ignoring: {relative_file_path}")
                continue

            if is_text_file(file):
                print(f"Processing: {relative_file_path}")
                try:
                    with open(file_path, "r", encoding="utf-8", errors="ignore") as f:
                        content = f.read()
                    
                    combined_content.append(f"\n\n--- START FILE: {relative_file_path.replace("\\\\", "/")} ---")
                    combined_content.append(content)
                    combined_content.append(f"--- END FILE: {relative_file_path.replace("\\\\", "/")} ---")
                    file_count += 1
                except Exception as e:
                    combined_content.append(f"\n\n--- ERROR READING FILE: {relative_file_path.replace("\\\\", "/")} ---")
                    combined_content.append(f"--- Error: {str(e)} ---")
                    print(f"Error reading {relative_file_path}: {e}")
            else:
                print(f"Skipping (not a text file or explicitly ignored): {relative_file_path}")


    try:
        with open(output_file_path, "w", encoding="utf-8") as f:
            f.write("\n".join(combined_content))
        print(f"\nSuccessfully combined {file_count} text files into {output_file_path}")
    except Exception as e:
        print(f"\nError writing to output file {output_file_path}: {e}")

if __name__ == "__main__":
    main() 