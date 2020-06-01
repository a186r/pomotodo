pragma solidity >=0.4.21 <0.7.0;

contract Base {

    // Account
    event CreateAccount(address indexed account, bytes username, address accountInfoAddress);
    event ChangeAccountInfoSuccess(
        address indexed account,
        bytes username,
        address ethAddress,
        bytes btcAddress,
        bytes xmrAddress
    );
    event GetAccountInfo(bytes username, address ethAddress, bytes btcAddress, bytes xmrAddress);
}
