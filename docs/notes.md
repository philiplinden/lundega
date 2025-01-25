# entity

entity:
    name <------ for humans to follow along
    wallet <---- attributes funds and signs transactions
    passport <-- designates the entity's identity, including attestations and stamps

# things to do
## transfer funds
spawn an entity

spawn another entity

entity A has funds in its wallet.
entity B has no funds.

entity A sends funds to entity B.
- this should eventually be a crypto transaction


## get stamps
spawn an entity
give its passport a stamp with an attestation and expiry date

## identity verification
spawn entities with passports
have one entity validate a stamp on another entity's passport
have one entity validate a stamp that is not on another entity's passport

## dump cached chain actions
to keep the data volume low for entities, sometimes all entities should dump
their cached chain transactions to a master database that synthesizes the
chain actions into a single history.
