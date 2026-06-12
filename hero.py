import argparse
import json
import sys
from pathlib import Path

REQUIRED_KEYS = {"tanks", "damage", "supports"}

def invalid_json_error(e: json.JSONDecodeError): error(f"Invalid JSON: {e}")

def error(message: str):
    print(f"fatal: {message}", file=sys.stderr)
    sys.exit(1)


def capitalize_first(s: str) -> str:
    return " ".join([s[0].upper() + s[1:] for s in s.split()])


def load_json(file_path: Path) -> dict:
    if not file_path.exists():
        data = {"tanks": [], "damage": [], "supports": []}
        file_path.write_text(json.dumps(data, indent=4))
        return data

    try:
        with file_path.open("r", encoding="utf-8") as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        invalid_json_error(e)

    # In the path where the var is unbound, we exit with code 1 anyways.
    validate_json(data) # pyright: ignore[reportPossiblyUnboundVariable]
    return data # pyright: ignore[reportPossiblyUnboundVariable]

def validate_json(data: dict):
    if not isinstance(data, dict):
        error("JSON root must be an object.")

    keys = set(data.keys())
    if keys != REQUIRED_KEYS:
        error(
            f"JSON must contain exactly these keys: "
            f"{sorted(REQUIRED_KEYS)}"
        )

    for key in REQUIRED_KEYS:
        if not isinstance(data[key], list):
            error(f"Key '{key}' must be a list.")

        for hero in data[key]:
            if not isinstance(hero, str):
                error(f"All entries in key '{key}' must be strings.")


def save_json(file_path: Path, data: dict, beautify: bool):
    with file_path.open("w", encoding="utf-8") as f:
        if beautify:
            json.dump(data, f, indent=4, ensure_ascii=False)
        else:
            json.dump(data, f, ensure_ascii=False)
        f.write("\n")


def add_hero(data: dict, hero_name: str, role: str, file_name: str, recursive: bool = False):

    def role_name(s):
        if s == "tanks" or s == "supports":
            return s[:-1]
        return s
    
    role_map = {
        "tank": "tanks",
        "damage": "damage",
        "support": "supports",
        "dps": "damage",
        "healer": "supports",

    }

    role_key = role_map[role.lower()]



    # Prevent duplicates across all roles
    for r, heroes in data.items():
        if hero_name in heroes:
            if r == role_key:
                error(f"{capitalize_first(r)} hero '{hero_name}' already exists.")
            else:
                # Move hero to the new role
                remove_hero(data, hero_name, recursive=True)
                add_hero(data, hero_name, role, file_name, recursive=True)
                if not recursive:
                    print(f"Moved hero {hero_name} to {capitalize_first(role_name(role_key))}")
                return
            break

    data[role_key].append(hero_name)
    data[role_key].sort()
    if not recursive:
        print(f"Added {capitalize_first(role_name(role_key))} hero {hero_name} to {file_name}")


def remove_hero(data: dict, hero_name: str, recursive: bool = False):
    for role, heroes in data.items():
        if hero_name in heroes:
            heroes.remove(hero_name)
            if not recursive:
                print(f"Removed {role} hero {hero_name}")
            return

    error(f"Hero '{hero_name}' not found.")


def build_parser():
    parser = argparse.ArgumentParser(
        description="Manage hero entries in a hero JSON file."
    )

    parser.add_argument("file_name", help="Path to hero JSON file")
    parser.add_argument(
        "action",
        choices=["add", "rm"],
        help="Action to perform"
    )

    parser.add_argument(
        "hero",
        type=str.lower,
        help="Hero name"
    )

    parser.add_argument(
        "-r",
        "--role",
        choices=["tank", "dps", "support"],
        type=str.lower,
        help="Specify the hero role."
    )
    parser.add_argument(
        "-b",
        "--beautify",
        action="store_true",
        help="Beautify the JSON on save"
    )
    parser.add_argument(
        "-t",
        "--tank",
        action="store_true",
        help="Store the Hero in the role Tank."
    )
    parser.add_argument(
        "-d",
        "--damage",
        action="store_true",
        help="Store the Hero in the role Damage."

    )
    parser.add_argument(
        "-s",
        "--support",
        action="store_true",
        help="Store the Hero in the role Support."
    )
    return parser

def multi_xor(b: list[bool]):
    true_indices = [i for i, v in enumerate(b) if v]
    if len(true_indices) == 1: return true_indices[0]
    else: return -1



def main():
    parser = build_parser()
    args = parser.parse_args()

    file_path = Path(args.file_name)
    data = load_json(file_path)

    role = ""
    if args.action == "add":
        if not args.role:
            role_selector = multi_xor([args.tank, args.damage, args.support])
            if not role_selector >= 0:
                error("fatal: argument '-r | --role' is required for the add action.")
            else:
                match role_selector:
                    case 0:
                        role = "tank"
                    case 1:
                        role = "damage"
                    case 2:
                        role = "support"
        else:
            role = args.role

        hero_name = capitalize_first(args.hero)
        add_hero(data, hero_name, role, args.file_name)

    elif args.action == "rm":
        hero_name = capitalize_first(args.hero)
        remove_hero(data, hero_name)

    save_json(file_path, data, args.beautify)


if __name__ == "__main__":
    main()