const hre = require("hardhat");

async function main() {
  const Splitter = await hre.ethers.getContractFactory("Splitter");
  const splitter = await Splitter.deploy();

  await splitter.deployed();

  console.log(`✅ Splitter deployed to: ${splitter.address}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
