const env = require("hardhat");
require("dotenv").config();

const PRIVATE_KEY = process.env.PRIVATE_KEY;

require("@nomicfoundation/hardhat-toolbox");

module.exports = {
  solidity: "0.8.20",
  networks: {
    mantleTestnet: {
      url: "https://rpc.testnet.mantle.xyz",
      accounts: [env.], // 🔐 replace with your real private key
    },
    mantleMainnet: {
      url: "https://rpc.mantle.xyz",
      accounts: [env.],
    },
  },
};