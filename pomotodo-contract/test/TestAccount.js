const AccountFactory = artifacts.require('AccountFactory');
const Account = artifacts.require('Account');

let accountFactoryInstance;
let accountInstance;

contract('AccountFactory', accounts => {
    describe('合约部署', function() {
        it("部署工厂合约", async() => {
            accountFactoryInstance = await AccountFactory.new();
        });
    })

    describe('账户注册', function() {
        it("注册个人账户合约", async() => {
            let result = await accountFactoryInstance.createAccount("0x757365726e616d65a");
            for(var i = 0; i < result.logs.length; i++) {
                if(result.logs[i].event == "CreateAccount"){
                    accountInstance = Account.at(result.logs[i].args.accountInfoAddress);
                }
            }
        });
    })

    describe('个人账户合约功能测试', function() {
        it("查询当前信息", async() => {
            let result = await accountInstance.getAccountInfo;
            console.log(result);
        });
    })
})
