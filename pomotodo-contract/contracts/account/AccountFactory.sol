pragma solidity >=0.4.21 <0.7.0;

import "./Account.sol";
import "../common/Base.sol";

contract AccountFactory is Base {
    function createAccount(bytes memory _username) public returns(bool) {
        Account account = new Account(_username, msg.sender);
        emit CreateAccount(msg.sender, _username, address(account));
        return true;
    }
}
