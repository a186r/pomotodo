pragma solidity >=0.4.21 <0.7.0;

import "../common/Base.sol";

contract Account is Base {
    address owner;
    AccountInfo accountInfo;

    constructor (bytes memory _username, address _ethAddress) public {
        AccountInfo(_username, _ethAddress, "", "");
    }

    modifier Owner() {
        require(msg.sender == owner, "");
        _;
    }

    struct AccountInfo {
        bytes username;
        address ethAddress;
        bytes btcAddress;
        bytes xmrAddress;
    }

    function changeUsername(bytes memory _username) public Owner() returns(bool) {
        accountInfo.username = _username;
        emit ChangeAccountInfoSuccess(
            owner,
            accountInfo.username,
            accountInfo.ethAddress,
            accountInfo.btcAddress,
            accountInfo.xmrAddress
        );
        return true;
    }

    function getAccountInfo() public returns(
        bytes memory,
        address,
        bytes memory,
        bytes memory
    ) {
        emit GetAccountInfo(
            accountInfo.username,
            accountInfo.ethAddress,
            accountInfo.btcAddress,
            accountInfo.xmrAddress
        );
    }

}
