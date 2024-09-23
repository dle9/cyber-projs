def string_to_octal(s):
    return ' '.join(f'{ord(c):o}' for c in s)

def octal_to_string(octal_str):
    octal_values = octal_str.split()
    characters = [chr(int(octal, 8)) for octal in octal_values]
    return ''.join(characters)

octal_string = "150 145 154 154 157"
original_string = octal_to_string(octal_string)
print(original_string)

# Example usage
input_string = "hello"
octal_string = string_to_octal(input_string)
print(octal_string)
print(octal_to_string(octal_string))

