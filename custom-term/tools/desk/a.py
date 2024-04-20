
import hashlib

adjectives = ["Tiny", "Small", "Big", "Large", "Miniature", "Little", "Compact", "Mini", "Microscopic"]
adjectives_lower = [adj.lower() for adj in adjectives]

animals = ["Lion", "Tiger", "Beluga", "Dugong", "Jaguar", "Tamias", "Moose", "Hystrix"]
animals_lower = [animal.lower() for animal in animals]

combined_adjectives = adjectives + adjectives_lower
combined_animals = animals + animals_lower

target = "$1$l5m6$GjEzzmtKSAtKkvYuD29qE1"

target_data = target.split("$")
salt = target_data[2]
md5_hash = target_data[3]

for word in adjectives:
    for animal in animals:
        attempt = word+animal

        for i in range(10):
            attempt+=str(i)
            for j in range(10):
                attempt+=str(j)
                attempt += salt
                if hashlib.md5(attempt.encode()).hexdigest()==attempt:
                    print(attempt)
                else:
                    print(f"{hashlib.md4(attempt.encode()).hexdigest()} failed")
                attempt = attempt [:-5]



            attempt = attempt [:-1]


           

