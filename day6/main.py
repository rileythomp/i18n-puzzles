corrupted_text = "religiÃ«n"
fixed_text = corrupted_text.encode("latin-1").decode("utf-8")
print(fixed_text)  # religiën
