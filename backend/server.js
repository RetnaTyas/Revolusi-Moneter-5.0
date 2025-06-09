const express = require('express');
const NodeCache = require('node-cache');
const { ethers } = require('ethers');
require('dotenv').config();

const goatAbi = require('./abi/GOAT.json').abi;
const meatAbi = require('./abi/MEAT.json').abi;

const app = express();
const cache = new NodeCache({ stdTTL: 60 });

const lodData = require('../lod_data.json');

const provider = new ethers.JsonRpcProvider(process.env.RPC_URL || 'http://localhost:8545');
const goatAddress = process.env.GOAT_ADDRESS;
const meatAddress = process.env.MEAT_ADDRESS;
let goatContract, meatContract;

if (goatAddress) goatContract = new ethers.Contract(goatAddress, goatAbi, provider);
if (meatAddress) meatContract = new ethers.Contract(meatAddress, meatAbi, provider);

async function fetchStats() {
  if (!goatContract || !meatContract) return { error: 'Contract addresses not configured' };
  const [goatSupply, meatSupply] = await Promise.all([
    goatContract.totalSupply(),
    meatContract.totalSupply()
  ]);
  const contractBalance = await goatContract.balanceOf(goatContract.target);
  return {
    goatSupply: goatSupply.toString(),
    meatSupply: meatSupply.toString(),
    totalStaked: contractBalance.toString()
  };
}

app.get('/health', (req, res) => {
  res.json({ status: 'ok' });
});

app.get('/stats', async (req, res) => {
  const cached = cache.get('stats');
  if (cached) return res.json(cached);
  try {
    const stats = await fetchStats();
    cache.set('stats', stats);
    res.json(stats);
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: 'failed' });
  }
});

app.get('/api/LOD/:commodity', (req, res) => {
  const key = req.params.commodity.toUpperCase();
  if (lodData[key]) {
    res.json(lodData[key]);
  } else {
    res.status(404).json({ error: 'not found' });
  }
});

const port = process.env.PORT || 3001;
app.listen(port, () => {
  console.log(`Backend listening on port ${port}`);
});
