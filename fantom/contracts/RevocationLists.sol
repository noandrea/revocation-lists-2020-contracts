// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

contract RevocationLists {
    uint constant BIT_SET_SIZE_KB = 4;
    uint constant BIT_SET_SIZE_B = BIT_SET_SIZE_KB * 1024;
    uint256 constant BIT_SET_SIZE = uint256(BIT_SET_SIZE_B) * 8;

    struct RL2020 {
        address owner;
        bytes1[] bitSet;
    }

    mapping(string => RL2020) private revocationLists;

    modifier listExists(string memory id) {
        require(bytes(id).length != 0, "List name cannot be empty");
        require(
            revocationLists[id].owner != address(0),
            "List with this name does not exist"
        );
        _;
    }

    modifier listDoesNotExist(string memory id) {
        require(bytes(id).length > 0, "List name cannot be empty");
        require(
            revocationLists[id].owner == address(0),
            "List with this name already exists"
        );
        _;
    }

    // register a new revocation list
    function register_list(string memory id) public listDoesNotExist(id) {
        revocationLists[id] = RL2020(msg.sender, new bytes1[](BIT_SET_SIZE_B));
    }

    function capacity() public pure returns (uint256) {
        return BIT_SET_SIZE;
    }

    function get_encoded_list(
        string memory id
    ) public view listExists(id) returns (string memory) {
        require(bytes(id).length > 0, "List id cannot be empty");
        RL2020 memory list = revocationLists[id];
        require(list.owner != address(0), "List with this id does not exist");
        return bytesToHex(list.bitSet);
    }

    function set(
        string memory id,
        uint[] memory ones,
        uint[] memory zeroes
    ) public {
        require(bytes(id).length > 0, "List name cannot be empty");
        require(revocationLists[id].owner == msg.sender, "Not allowed");
        RL2020 storage list = revocationLists[id];
        require(
            list.owner == msg.sender,
            "Only the owner can set values in the list"
        );
        for (uint i = 0; i < ones.length; i++) {
            require(ones[i] >= 0, "Revocation list index cannot be negative");
            require(
                ones[i] < BIT_SET_SIZE,
                "Revocation list index is out of bounds"
            );
            list.bitSet[ones[i] / 8] |= bytes1(uint8(1 << (ones[i] % 8)));
        }
        for (uint i = 0; i < zeroes.length; i++) {
            require(zeroes[i] >= 0, "Revocation list index cannot be negative");
            require(
                zeroes[i] < BIT_SET_SIZE,
                "Revocation list index is out of bounds"
            );
            list.bitSet[zeroes[i] / 8] &= bytes1(
                uint8(~(1 << (zeroes[i] % 8)))
            );
        }
    }

    function is_set(
        string memory id,
        uint revocationListIndex
    ) public view listExists(id) returns (bool) {
        require(bytes(id).length > 0, "List id cannot be empty");
        require(
            revocationListIndex >= 0,
            "Revocation list index cannot be negative"
        );
        require(
            revocationListIndex < BIT_SET_SIZE,
            "Revocation list index is out of bounds"
        );
        RL2020 storage list = revocationLists[id];
        return
            list.bitSet[revocationListIndex / 8] &
                bytes1(uint8(1 << (revocationListIndex % 8))) !=
            0;
    }

    // Constant array of hex symbols used in encoding
    bytes16 constant _HEX_SYMBOLS = "0123456789abcdef";

    function bytesToHex(
        bytes1[] memory data
    ) public pure returns (string memory) {
        bytes memory result = new bytes(data.length * 2);
        uint256 j = 0;
        for (uint256 i = 0; i < data.length; i++) {
            bytes1 b = data[i];
            result[j++] = _HEX_SYMBOLS[uint8(b) >> 4];
            result[j++] = _HEX_SYMBOLS[uint8(b) & 0x0f];
        }
        return string(result);
    }
}
