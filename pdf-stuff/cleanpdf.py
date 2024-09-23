def find_startxref_lines(filename):
    try:
        with open(filename, 'r') as file:
            lines = file.readlines()

        for i, line in enumerate(lines):
            if 'startxref' in line:
                # Print the line number containing 'startxref'
                print(f"Line {i + 1}: {line.strip()}")
                
                # Print the next line if it exists
                if i + 1 < len(lines):
                    print(f"Line {i + 2}: {lines[i + 1].strip()}")
                else:
                    print(f"Line {i + 2}: (No next line available)")
                
                print()  # Add a blank line for readability

    except FileNotFoundError:
        print(f"The file '{filename}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

# Usage example:
filename = 'hello-world.pdf'  # Replace with the path to your file
find_startxref_lines(filename)

