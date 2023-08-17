#! IMPORTANT
# Please first download cah-cards-full.json from https://crhallberg.com/cah/ and place it in the same directory as this script. Proceed to run this script to generate the card files.

import json
import os
import bson

official = {}
BASE = os.path.dirname(os.path.realpath(__file__))

with open(f"{BASE}/cah-cards-full.json") as f:
	data = json.load(f)

	for pack in data:
		name = pack["name"]
		
		# compile data
		data = {
			"name": name,
			"official": pack["official"],
			"white": [white["text"] for white in pack["white"]],
			"black": [{ "text": black["text"], "pick": black["pick"] } for black in pack["black"]]
		}

		# write to file
		with open(f"{BASE}/packs/{''.join(e for e in name if e.isalnum()).lower()}.bson", "wb") as f:
			f.write(bson.dumps(data))
