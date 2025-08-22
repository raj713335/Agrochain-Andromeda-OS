require("@nomiclabs/hardhat-waffle");


const INFURA_URL = '';
const PRIVATE_KEY = '';
const COINBASE_URL = 'https://goerli.ethereum.coinbasecloud.net';

module.exports = {
    solidity: "0.8.4",
    networks: {
        goerli: {
            url: COINBASE_URL,
            accounts: [`0x${PRIVATE_KEY}`]
        }
    },
    paths: {
    artifacts: "./src/backend/artifacts",
    sources: "./src/backend/contracts",
    cache: "./src/backend/cache",
    tests: "./src/backend/test"
    },
};
