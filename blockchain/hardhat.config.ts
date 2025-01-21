import { HardhatUserConfig } from "hardhat/config";
import "@nomiclabs/hardhat-ethers";
import * as dotenv from "dotenv";

dotenv.config();

const ALCHEMY_API_KEY = process.env.ALCHEMY_API_KEY || "";
const PRIVATE_KEY = process.env.PRIVATE_KEY || "";
const CHAIN_ID = parseInt(process.env.CHAIN_ID || "31337", 10);

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      { version: "0.8.20" },
      { version: "0.8.1" },
      { version: "0.6.12" },
    ],
  },
  networks: {
    hardhat: {
      forking: {
        url: `https://eth-mainnet.alchemyapi.io/v2/${ALCHEMY_API_KEY}`,
        blockNumber: process.env.FORK_BLOCK 
          ? parseInt(process.env.FORK_BLOCK, 10) 
          : undefined,
      },
    },
    localhost: {
      url: "http://127.0.0.1:8545",
      chainId: CHAIN_ID,
      accounts: PRIVATE_KEY ? [PRIVATE_KEY] : [],
    },
  },
};

export default config;
