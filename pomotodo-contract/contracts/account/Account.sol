pragma solidity >=0.4.21 <0.7.0;

contract Account {

    constructor () public {

    }

    struct Account{
        bytes32 username,
        address eth_address,
        bytes32 btc_address,
        bytes xmr_address,
    }

}
