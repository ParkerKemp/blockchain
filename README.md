Integrate with sqlite3
Models: Block, BlockChain
Block: hash, last_hash, next_strength (# 0-bits), content, nonce, timestamp
BlockChain: blocks[], validate()

Try to load sqlite database. If not found, start with genesis block (hardcoded) and write it to new db
Randomize nonce and then hash until next_strength is met. Set next_strength based on elapsed time since last block. Write block to db and then repeat
