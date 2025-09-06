# This script makes sure the version number specified in the parser lib matches.
# It should only be run by CI/CD automation on build.

# Get version number
with open("version") as vf:
    version = vf.read().strip()

# Read Cargo.toml
with open("lib/Cargo.toml") as f:
    lines = f.readlines()

# Update Cargo.toml
with open("lib/Cargo.toml", "w") as f:
    for line in lines:
        if line.startswith("version ="):
            f.write(f'version = "{version}"\n')
        else:
            f.write(line)