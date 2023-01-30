const axios = require('axios')
const { ethers } = require('ethers')
const fs = require('fs')
require('dotenv').config();

const infuraUrl = `${process.env.rpcUrl}`
const addresses = JSON.parse(process.env.address)
const provider = new ethers.providers.JsonRpcProvider(infuraUrl)

const getAbi = async () => {

  for (let i = 0; i < addresses.length; i++) {
    const res = await axios.get(`https://api.etherscan.io/api?module=contract&action=getabi&address=${addresses[i]}&apikey=${process.env.apiKey}`)
    const abi = JSON.parse(res.data.result)
    fs.writeFile('./res/ABI/' + i + '.abi', JSON.stringify(abi), (err) => {
      if (err) throw err;
    })

    const code = await provider.getCode(addresses[i])
    const polish = code.substring(2)
    fs.writeFile('./res/BYTECODE/' + i + '.hex', polish, (err) => {
      if (err) throw err;
    })
  }
}
getAbi()