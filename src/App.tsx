import React, { useEffect, useState } from 'react';
import './App.css';

import '@polkadot/api-augment';

import { ApiPromise } from '@polkadot/api';
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
import { WsProvider } from '@polkadot/rpc-provider';

function App() {
  const [accounts, setAccount] = useState([]);
  const [selectedAcccount, setSelectedAccount] = useState();

  const setup = async () => {
    const wsProvider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: wsProvider });

    console.log(api.genesisHash.toHex());
    console.log((await api.rpc.system.properties()).toHuman());
  };

  const handleConnection = async () => {
    const extensions = await web3Enable('management-app');
    if (extensions.length === 0) {
      return;
    }
    const allAccounts = await web3Accounts();
    console.log(allAccounts);
  };

  useEffect(() => {
    setup();
  },[]);

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={handleConnection}> Connect To Wallet </button>
      </header>
    </div>
  );
}

export default App;
