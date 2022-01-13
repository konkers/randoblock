# maintain the 'infinite block' entity
data merge entity @s {Duration: 2000000000, Age: 0}

# make a particles appear on the infinite block constantly
particle minecraft:ash ~ ~0.8 ~ 0.2 0.3 0.2 0.001 1 force


# give counter entity a tag if a block was mined this tick (or if it was set on fire or melted)
execute if block ~ ~ ~ minecraft:air run tag @s add konkers-randoblock-mined
execute if block ~ ~ ~ minecraft:fire run tag @s add konkers-randoblock-mined
execute if block ~ ~ ~ minecraft:water run tag @s add konkers-randoblock-mined

# replace the empty space after mining (air/fire/water) with barrier
execute as @s[tag=konkers-randoblock-mined] at @s run fill ~ ~ ~ ~ ~ ~ minecraft:barrier replace minecraft:air
execute as @s[tag=konkers-randoblock-mined] at @s run fill ~ ~ ~ ~ ~ ~ minecraft:barrier replace minecraft:fire
execute as @s[tag=konkers-randoblock-mined] at @s run fill ~ ~ ~ ~ ~ ~ minecraft:barrier replace minecraft:water

# on first load, while counter is 0, set the block to being mined, to trigger the tutorial
tag @s[scores={konkers-randoblock-counter=0}] add konkers-randoblock-mined


####################
# SAFETY MECHANICS #
####################

# preserve dropped items and teleport them on top of block
execute as @e[type=item,distance=..2] run data merge entity @s {NoAI: true, Invulnerable: true, Age: -32768}
execute as @e[type=item,distance=..2] unless entity @s[tag=konkers-randoblock-tped] at @s run tp @s 0.5 61.3 0.5
execute as @e[type=item,distance=..2] unless entity @s[tag=konkers-randoblock-tped] at @s run data merge entity @s {PickupDelay: 15, Motion: [0.0d, 0.0d, 0.0d]}
tag @e[type=item,distance=..2] add konkers-randoblock-tped

# when a new block is mined, the items are always teleported up again
execute as @s[tag=konkers-randoblock-mined] at @s run tp @e[type=item,distance=..2] 0.5 61.3 0.5
execute as @s[tag=konkers-randoblock-mined] at @s run execute as @e[type=item,distance=..2] at @s run data merge entity @s {Motion: [0.0d, 0.0d, 0.0d]}


# save players standing on top of the block that was just mined from falling down
execute as @s[tag=konkers-randoblock-mined] at @s run execute as @a[x=0,dx=0,y=61,dy=0,z=0,dz=0] at @s run tp @s ~ ~0.3 ~




execute as @s[tag=konkers-randoblock-mined] at @s run function konkers-randoblock:infinite-block/next

# count down cooldown
scoreboard players add @s konkers-randoblock-cooldown 0
scoreboard players remove @s[scores={konkers-randoblock-cooldown=1..}] konkers-randoblock-cooldown 1

tag @s[tag=konkers-randoblock-mined] remove konkers-randoblock-mined