# tools
## chain
Blockchain (rust-blockchain): This will be used to store each robot's actions as
blocks. Each robot appends a new block to the blockchain every time it performs
an action.

HBBFT Consensus (hbbft): The robots will broadcast their actions to other nodes
in the network. HBBFT will ensure that the robots reach a consensus on the
actions, even in the presence of faulty or malicious robots. In the example
above, after broadcasting the robot's action, HBBFT checks if consensus is
reached (this is simulated here with wait_for_consensus()).

Bevy: Manages the robot entities, allowing them to interact and perform actions.
Each robot has a data field representing its state, which is updated and
validated through the blockchain and HBBFT.

## organizations
Within our architecture, we chose to represent each entity via an ERC-721 token,
and their respective organizations as a Gnosis multisignature wallet. Each
ERC-721 has roles within the organization tree using Hats Protocol that can
unlock certain levels of permissions for the organization.

## config and assets
The population should be generated from a ron file. We can also write the
current state of the blockchain to a similar ron file. Bevy supports
hot-reloading of assets, so we can use this to update the population and
blockchain state without restarting the simulation.
