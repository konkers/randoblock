tellraw @a {"text":"creating oneblock","color":"blue"}

# kill any existing infinite blocks
kill @e[tag=konkers-randoblock-block]

# generate the first mineable block
setblock ~ ~ ~ minecraft:grass_block

# generate the infinite block entity
summon minecraft:area_effect_cloud ~ ~ ~ {PersistenceRequired: 0b, NoGravity: 1b, Duration: 2000000000, Tags: ["konkers-randoblock-block"]}