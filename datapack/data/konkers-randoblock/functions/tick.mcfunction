

# if no infinite block exists in the position, kill existing ones and place a new one
execute positioned 0.5 60.5 0.5 unless entity @e[tag=konkers-randoblock-block,distance=..1] run function konkers-randoblock:infinite-block/create

execute as @e[tag=konkers-randoblock-block] at @s run function konkers-randoblock:infinite-block/manage