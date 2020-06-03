pragma solidity >=0.4.21 <0.7.0;

contract Base {

    // Account
    event CreateAccount(address indexed account, bytes16 username, address accountInfoAddress);
    event ChangeAccountInfoSuccess(
        address indexed account,
        bytes16 username,
        address ethAddress,
        bytes btcAddress,
        bytes xmrAddress
    );
    event GetAccountInfo(bytes16 username, address ethAddress, bytes btcAddress, bytes xmrAddress);
}
