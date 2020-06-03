pragma solidity >=0.4.21 <0.7.0;

import "../common/Base.sol";

contract Account is Base {
    address owner;
    AccountInfo accountInfo;

    ///@dev 构造函数
    ///@param _username bytes16可以存储8个中文或者16个英文字母
    ///@param _ethAddress  创建个人实体合约时的eth地址
    constructor (bytes16 _username, address _ethAddress) public {
        AccountInfo(_username, _ethAddress, "", "");
    }

    modifier Owner() {
        require(msg.sender == owner, "");
        _;
    }

    struct AccountInfo {
        bytes16 username;
        address ethAddress;
        bytes btcAddress;
        bytes xmrAddress;
    }

    function changeUsername(bytes16 _username) public Owner() returns(bool) {
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
        bytes16,
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

        return (accountInfo.username, accountInfo.ethAddress, accountInfo.btcAddress, accountInfo.xmrAddress);
    }

}