const { Wallet, Provider } = require("zksync-web3");
const { Deployer } = require("@matterlabs/hardhat-zksync-deploy");
const hre = require("hardhat");

const PRIVATE_KEY = "YOUR_PRIVATE_KEY"; // replace with your zkSync wallet private key

module.exports = async function () {
    const wallet = new Wallet(PRIVATE_KEY);

    const deployer = new Deployer(hre, wallet);

    const artifact = await deployer.loadArtifact("Splitter");

    const contract = await deployer.deploy(artifact, []);
    console.log(`✅ Splitter deployed at: ${contract.address}`);
};