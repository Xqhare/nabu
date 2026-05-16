import os

directory = "xff-example-data"
files = sorted([f for f in os.listdir(directory) if f.endswith(".xff")])

mismatches = []
consistent = []
unlabelled = []

print(f"{'Filename':<45} | {'First Byte':<10} | {'Expected':<10} | {'Status'}")
print("-" * 85)

for filename in files:
    path = os.path.join(directory, filename)
    try:
        with open(path, "rb") as f:
            first_byte = f.read(1)
            if not first_byte:
                continue
            byte_val = first_byte[0]
    except Exception as e:
        print(f"Error reading {filename}: {e}")
        continue

    expected_ver = None
    if "v0" in filename:
        expected_ver = 0
    elif "v1" in filename:
        expected_ver = 1
    elif "v2" in filename:
        expected_ver = 3  # 0x03
    elif "v3" in filename:
        expected_ver = 88  # 'X' (0x58)
    
    status = "OK"
    expected_str = f"{expected_ver:02x}" if expected_ver is not None else "???"
    
    if expected_ver is not None:
        if byte_val != expected_ver:
            status = "MISMATCH"
            mismatches.append(filename)
        else:
            consistent.append(filename)
    else:
        status = "UNLABELLED"
        unlabelled.append(filename)

    print(f"{filename:<45} | {byte_val:02x}       | {expected_str:<10} | {status}")

print("\n" + "=" * 30)
print(f"Total Files:  {len(files)}")
print(f"Consistent:   {len(consistent)}")
print(f"Mismatches:   {len(mismatches)}")
print(f"Unlabelled:   {len(unlabelled)}")
print("=" * 30)

if mismatches:
    print("\nAction Required: The following files have version mismatches!")
    for m in mismatches:
        print(f"  - {m}")
