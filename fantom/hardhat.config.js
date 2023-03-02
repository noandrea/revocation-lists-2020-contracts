
require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-ethers");

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async () => {
  const accounts = await ethers.getSigners();

  for (const account of accounts) {
    console.log(account.address);
  }
});

task("balances", "Prints the list of account balances", async () => {
  const accounts = await ethers.getSigners();

  for (const account of accounts) {
    balance = await ethers.provider.getBalance(account.address);
    console.log(account.address, "has balance", balance.toString());
  }
});

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  solidity: {
    compilers: [
      {
        version: "0.8.17"
      }
    ]
  },
  networks: {
    hardhat: {
      gasPrice: 470000000000,
      chainId: 43112,
    },
    ftmt: {
      url: 'https://rpc.testnet.fantom.network/',
      gasPrice: 470000000000,
      chainId: 4002,
      accounts: ["0x56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027",
      ]
    },
    ftm: {
      url: 'https://rpc.ftm.tools/',
      gasPrice: 470000000000,
      chainId: 250,
      accounts: [
      ]
    },
    goerli: {
      url: 'https://goerli.infura.io/v3/',
      gasPrice: 470000000000,
      chainId: 5,
      accounts: [
      ]
    },
    eth: {
      url: 'https://mainnet.infura.io/v3/',
      gasPrice: 470000000000,
      chainId: 1,
      accounts: [
      ]
    }
  }
};